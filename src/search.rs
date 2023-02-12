use crate::attacks::is_in_check;
use crate::eval::{Evaluator, EVAL_CHECKMATE, EVAL_MAX, EVAL_MIN, EVAL_STALEMATE};
use crate::movegen::MoveGenerator;
use crate::position::Position;
use crate::r#move::Move;
use std::time::{Duration, Instant};

pub struct Report {
    pub depth: u8,
    pub pv: Option<(Vec<Move>, i32)>,
    pub elapsed: Duration,
    pub nodes: u128,
}

impl Report {
    fn new() -> Self {
        Self {
            depth: 0,
            pv: None,
            elapsed: Duration::ZERO,
            nodes: 0,
        }
    }

    fn depth(&mut self, depth: u8) {
        self.depth = depth;
    }

    fn pv(&mut self, moves: Vec<Move>, eval: i32) {
        self.pv = Some((moves, eval));
    }

    fn elapsed(&mut self, time: Duration) {
        self.elapsed = time;
    }

    fn node(&mut self) {
        self.nodes += 1;
    }
}

pub trait Reporter {
    fn send(&mut self, report: &Report);
}

pub struct Stopper {
    max_depth: u8,
}

impl Stopper {
    pub fn new(max_depth: u8) -> Self {
        Self { max_depth }
    }

    pub fn should_stop(&self, depth: u8) -> bool {
        depth > self.max_depth
    }
}

pub fn search(pos: &mut Position, reporter: &mut dyn Reporter, stopper: &Stopper) {
    let start = Instant::now();
    let mut report = Report::new();
    let mut pv = vec![];

    for depth in 1.. {
        if stopper.should_stop(depth) {
            break;
        }

        let eval = alpha_beta(pos, depth, EVAL_MIN, EVAL_MAX, &mut pv, &mut report);

        report.depth(depth);
        report.elapsed(start.elapsed());
        report.pv(pv.clone(), eval);

        reporter.send(&report);
    }
}

fn alpha_beta(
    pos: &mut Position,
    depth: u8,
    mut alpha: i32,
    beta: i32,
    pv: &mut Vec<Move>,
    report: &mut Report,
) -> i32 {
    if depth == 0 {
        return quiescence(pos, alpha, beta, &mut vec![], report);
    }

    report.node();

    let (pv_move, mut next_ply_pv) = split_pv(pv);
    let colour_to_move = pos.colour_to_move;
    let mut has_legal_move = false;

    for mv in order_moves(&pos.generate_all_moves(), pv_move) {
        pos.do_move(&mv);

        if is_in_check(colour_to_move, &pos.board) {
            pos.undo_move(&mv);
            continue;
        }

        has_legal_move = true;

        let eval = -alpha_beta(pos, depth - 1, -beta, -alpha, &mut next_ply_pv, report);

        if eval >= beta {
            pos.undo_move(&mv);
            return beta;
        }

        if eval > alpha {
            alpha = eval;

            pv.clear();
            pv.push(*mv);
            pv.append(&mut next_ply_pv);
        }

        pos.undo_move(&mv);
    }

    if !has_legal_move {
        return if is_in_check(colour_to_move, &pos.board) {
            EVAL_CHECKMATE
        } else {
            EVAL_STALEMATE
        };
    }

    alpha
}

fn quiescence(pos: &mut Position, mut alpha: i32, beta: i32, pv: &mut Vec<Move>, report: &mut Report) -> i32 {
    report.node();

    let eval = pos.evaluate();

    if eval >= beta {
        return beta;
    }

    if eval > alpha {
        alpha = eval;
    }

    let (pv_move, mut next_ply_pv) = split_pv(pv);
    let colour_to_move = pos.colour_to_move;

    for mv in order_moves(&pos.generate_capture_moves(), pv_move) {
        pos.do_move(&mv);

        if is_in_check(colour_to_move, &pos.board) {
            pos.undo_move(&mv);
            continue;
        }

        let eval = -quiescence(pos, -beta, -alpha, &mut next_ply_pv, report);

        if eval >= beta {
            pos.undo_move(&mv);
            return beta;
        }

        if eval > alpha {
            alpha = eval;

            pv.clear();
            pv.push(*mv);
            pv.append(&mut next_ply_pv);
        }

        pos.undo_move(&mv);
    }

    alpha
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
    use crate::castling::CastlingRights;
    use crate::square::Square;
    use doubles::ReporterSpy;

    #[test]
    fn report_the_depth() {
        let mut pos = Position::startpos();
        let mut reporter = ReporterSpy::new();

        search(&mut pos, &mut reporter, &Stopper::new(3));

        assert_eq!(vec![1, 2, 3], reporter.depths);
    }

    #[test]
    fn report_the_principal_variation() {
        let mut pos = Position::startpos();
        let mut reporter = ReporterSpy::new();

        search(&mut pos, &mut reporter, &Stopper::new(1));

        assert!(!reporter.last_pv_moves.is_empty());
    }

    #[test]
    fn report_an_elapsed_time_greater_than_zero() {
        let mut pos = Position::startpos();
        let mut reporter = ReporterSpy::new();

        search(&mut pos, &mut reporter, &Stopper::new(1));

        assert!(reporter.last_elapsed.gt(&Duration::ZERO));
    }

    #[test]
    fn report_the_node_count() {
        let mut pos = Position::startpos();
        let mut reporter = ReporterSpy::new();

        search(&mut pos, &mut reporter, &Stopper::new(1));

        assert_eq!(21, reporter.last_nodes);
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
                is_en_passant: false,
            }
        }

        let pv_move = make_move(Square::from_index(0), Square::from_index(1));

        let moves = [
            make_move(Square::from_index(2), Square::from_index(3)),
            make_move(Square::from_index(4), Square::from_index(5)),
            pv_move,
        ];

        let ordered_moves = order_moves(&moves, Some(pv_move));

        assert_eq!(ordered_moves[0].mv, pv_move);
    }

    mod doubles {
        use super::*;

        pub struct ReporterSpy {
            pub depths: Vec<u8>,
            pub last_pv_moves: Vec<Move>,
            pub last_elapsed: Duration,
            pub last_nodes: u128,
        }

        impl ReporterSpy {
            pub fn new() -> Self {
                Self {
                    depths: vec![],
                    last_pv_moves: vec![],
                    last_elapsed: Duration::ZERO,
                    last_nodes: 0,
                }
            }
        }

        impl Reporter for ReporterSpy {
            fn send(&mut self, report: &Report) {
                self.depths.push(report.depth);
                self.last_elapsed = report.elapsed;
                self.last_nodes = report.nodes;

                if let Some((moves, _)) = &report.pv {
                    self.last_pv_moves = moves.clone();
                }
            }
        }
    }
}
