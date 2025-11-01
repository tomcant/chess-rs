use self::{
    killers::KillerMoves,
    report::{Report, Reporter},
    stopper::Stopper,
    tt::TranspositionTable,
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

pub const MAX_DEPTH: u8 = u8::MAX;

#[rustfmt::skip]
pub fn search(pos: &mut Position, reporter: &impl Reporter, stopper: &Stopper) {
    let max_depth = stopper.depth.unwrap_or(MAX_DEPTH);
    let mut tt = TranspositionTable::new();
    let mut killers = KillerMoves::new();
    let mut report = Report::new();

    for depth in 1..=max_depth {
        let mut pv = MoveList::new();
        let eval = alphabeta::search(pos, depth, EVAL_MIN, EVAL_MAX, &mut pv, &mut tt, &mut killers, &mut report, stopper);

        if stopper.should_stop(&report) {
            break;
        }

        report.depth = depth;
        report.pv = Some(sanitise_pv(pos.clone(), (pv.clone(), eval)));
        report.tt_stats = (tt.usage, tt.capacity);

        reporter.send(&report);
    }
}

fn sanitise_pv(mut pos: Position, (moves, eval): (MoveList, i32)) -> (MoveList, i32) {
    for (index, mv) in moves.iter().enumerate() {
        pos.do_move(mv);

        if pos.is_fifty_move_draw() || pos.is_repetition_draw() {
            return (MoveList::from_slice(&moves[..=index]), EVAL_DRAW);
        }
    }

    (moves, eval)
}
