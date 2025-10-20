use self::{
    killers::KillerMoves,
    report::{Report, Reporter},
    stopper::Stopper,
};
use crate::eval::*;
use crate::movegen::{Move, MoveList};
use crate::position::Position;

pub mod report;
pub mod stopper;
pub mod tt;

mod alphabeta;
mod killers;
mod quiescence;

const MAX_DEPTH: u8 = u8::MAX;

#[rustfmt::skip]
pub fn search(pos: &mut Position, reporter: &impl Reporter, stopper: &impl Stopper) {
    let mut pv = MoveList::new();
    let mut tt = tt::Table::with_mb(tt::size_mb());
    let mut killers = KillerMoves::new();
    let mut report = Report::new();

    let max_depth = match stopper.max_depth() {
        Some(depth) => depth,
        None => MAX_DEPTH,
    };

    for depth in 1..=max_depth {
        report.depth = depth;

        let eval = alphabeta::search(pos, depth, EVAL_MIN, EVAL_MAX, &mut pv, &mut tt, &mut killers, &mut report, stopper);

        if stopper.should_stop(&report) {
            break;
        }

        report.pv = Some((pv.clone(), eval));
        report.tt_stats = (tt.usage, tt.capacity);
        reporter.send(&report);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::*;
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
    fn report_the_number_of_moves_until_mate() {
        let mut pos = parse_fen("8/8/4R3/8/k1K5/8/8/8 w - - 0 1");
        let reporter = ReporterSpy::new();

        search(&mut pos, &reporter, &StopperStub(1));

        assert_eq!(reporter.last_moves_until_mate(), Some(1));
    }

    mod doubles {
        use super::*;
        use std::cell::{Cell, RefCell};

        pub struct ReporterSpy {
            depths: RefCell<Vec<u8>>,
            last_pv_moves: RefCell<MoveList>,
            last_nodes: Cell<u128>,
            last_moves_until_mate: Cell<Option<u8>>,
        }

        impl ReporterSpy {
            pub fn new() -> Self {
                Self {
                    depths: RefCell::new(vec![]),
                    last_pv_moves: RefCell::new(MoveList::new()),
                    last_nodes: Cell::new(0),
                    last_moves_until_mate: Cell::new(None),
                }
            }

            pub fn depths(&self) -> Vec<u8> {
                self.depths.borrow().clone()
            }

            pub fn last_pv_moves(&self) -> MoveList {
                self.last_pv_moves.borrow().clone()
            }

            pub fn last_nodes(&self) -> u128 {
                self.last_nodes.get()
            }

            pub fn last_moves_until_mate(&self) -> Option<u8> {
                self.last_moves_until_mate.get()
            }
        }

        impl Reporter for ReporterSpy {
            fn send(&self, report: &Report) {
                self.depths.borrow_mut().push(report.depth);
                self.last_nodes.set(report.nodes);
                self.last_moves_until_mate.set(report.moves_until_mate());

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
