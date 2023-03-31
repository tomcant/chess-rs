use self::{
    report::{Report, Reporter},
    stopper::Stopper,
};
use crate::eval::*;
use crate::movegen::Move;
use crate::position::Position;

pub mod report;
pub mod stopper;

mod alphabeta;
mod quiescence;

const MAX_DEPTH: u8 = u8::MAX;

pub fn search(pos: &mut Position, reporter: &impl Reporter, stopper: &impl Stopper) {
    let mut pv = vec![];
    let mut report = Report::new();

    let max_depth = match stopper.max_depth() {
        Some(depth) => depth,
        None => MAX_DEPTH,
    };

    for depth in 1..=max_depth {
        report.depth = depth;

        let eval = alphabeta::search(pos, depth, EVAL_MIN, EVAL_MAX, &mut pv, &mut report, stopper);

        if stopper.should_stop(&report) {
            break;
        }

        report.pv = Some((pv.clone(), eval));
        reporter.send(&report);
    }
}

fn split_pv(pv: &mut [Move]) -> (Option<Move>, Vec<Move>) {
    if let Some((head, tail)) = pv.split_first() {
        (Some(*head), tail.to_vec())
    } else {
        (None, vec![])
    }
}

struct OrderedMove {
    mv: Move,
    order: u8,
}

impl std::ops::Deref for OrderedMove {
    type Target = Move;

    fn deref(&self) -> &Self::Target {
        &self.mv
    }
}

const ORDER_PV_MOVE: u8 = 0;
const ORDER_NON_PV_MOVE: u8 = 1;

fn order_moves(moves: &[Move], pv_move: Option<Move>) -> Vec<OrderedMove> {
    let has_pv_move = pv_move.is_some();

    let mut moves = moves
        .iter()
        .map(|mv| OrderedMove {
            mv: *mv,
            order: if has_pv_move && *mv == pv_move.unwrap() {
                ORDER_PV_MOVE
            } else {
                ORDER_NON_PV_MOVE
            },
        })
        .collect::<Vec<OrderedMove>>();

    if has_pv_move {
        moves.sort_unstable_by_key(|mv| mv.order);
    }

    moves
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::position::CastlingRights;
    use crate::square::Square;
    use doubles::*;

    #[test]
    fn report_the_depth() {
        let reporter = ReporterSpy::new();

        search(&mut Position::startpos(), &reporter, &StopperStub(3));

        assert_eq!(vec![1, 2, 3], reporter.depths());
    }

    #[test]
    fn report_the_principal_variation() {
        let reporter = ReporterSpy::new();

        search(&mut Position::startpos(), &reporter, &StopperStub(1));

        assert!(!reporter.last_pv_moves().is_empty());
    }

    #[test]
    fn report_the_node_count() {
        let reporter = ReporterSpy::new();

        search(&mut Position::startpos(), &reporter, &StopperStub(1));

        assert_eq!(21, reporter.last_nodes());
    }

    #[test]
    fn order_pv_move_to_front() {
        fn make_move(from: Square, to: Square) -> Move {
            Move {
                from,
                to,
                captured_piece: None,
                promotion_piece: None,
                castling_rights: CastlingRights::none(),
                half_move_clock: 0,
                is_en_passant: false,
            }
        }

        let pv_move = make_move(Square::A1, Square::B1);

        let moves = [
            make_move(Square::C1, Square::D1),
            make_move(Square::E1, Square::F1),
            pv_move,
        ];

        let ordered_moves = order_moves(&moves, Some(pv_move));

        assert_eq!(ordered_moves[0].mv, pv_move);
    }

    mod doubles {
        use super::*;
        use std::cell::{Cell, RefCell};

        pub struct ReporterSpy {
            depths: RefCell<Vec<u8>>,
            last_pv_moves: RefCell<Vec<Move>>,
            last_nodes: Cell<u128>,
        }

        impl ReporterSpy {
            pub fn new() -> Self {
                Self {
                    depths: RefCell::new(vec![]),
                    last_pv_moves: RefCell::new(vec![]),
                    last_nodes: Cell::new(0),
                }
            }

            pub fn depths(&self) -> Vec<u8> {
                self.depths.borrow().clone()
            }

            pub fn last_pv_moves(&self) -> Vec<Move> {
                self.last_pv_moves.borrow().clone()
            }

            pub fn last_nodes(&self) -> u128 {
                self.last_nodes.get()
            }
        }

        impl Reporter for ReporterSpy {
            fn send(&self, report: &Report) {
                self.depths.borrow_mut().push(report.depth);
                self.last_nodes.set(report.nodes);

                if let Some((moves, _)) = &report.pv {
                    *self.last_pv_moves.borrow_mut() = moves.clone();
                }
            }
        }

        pub struct StopperStub(pub u8);

        impl Stopper for StopperStub {
            fn should_stop(&self, report: &Report) -> bool {
                report.depth > self.0
            }

            fn max_depth(&self) -> Option<u8> {
                Some(self.0)
            }
        }
    }
}
