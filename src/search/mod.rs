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

// Aspiration window tuning
const ASP_MIN_DEPTH: u8 = 4;
const ASP_BASE_DELTA: i32 = 25; // Quarter pawn
const ASP_EXPANSION_FACTOR: i32 = 2;
const ASP_MAX_RETRIES: u8 = 3;

#[rustfmt::skip]
pub fn search(pos: &mut Position, reporter: &impl Reporter, stopper: &Stopper) {
    let mut tt = TranspositionTable::new();
    let mut killers = KillerMoves::new();
    let mut report = Report::new();

    let mut last_eval: i32 = 0;
    let max_depth = stopper.depth.unwrap_or(MAX_DEPTH);

    for depth in 1..=max_depth {
        let mut pv = MoveList::new();

        // Bypass aspiration search for shallow depths or near-mate situations
        let do_aspiration_search = depth >= ASP_MIN_DEPTH && last_eval.abs() < EVAL_MATE_THRESHOLD;
        let (mut delta_low, mut delta_high) = (ASP_BASE_DELTA, ASP_BASE_DELTA);

        let (mut alpha, mut beta) = if do_aspiration_search {
            (
                (last_eval - delta_low).max(EVAL_MIN),
                (last_eval + delta_high).min(EVAL_MAX),
            )
        } else {
            (EVAL_MIN, EVAL_MAX)
        };

        last_eval = {
            let eval_final;
            let mut retries = 0;

            loop {
                let eval = alphabeta::search(pos, depth, alpha, beta, &mut pv, &mut tt, &mut killers, &mut report, stopper);

                if (eval > alpha && eval < beta) || stopper.should_stop(&report) {
                    eval_final = eval;
                    break;
                }

                retries += 1;
                if retries > ASP_MAX_RETRIES {
                    alpha = EVAL_MIN;
                    beta = EVAL_MAX;
                    continue;
                }

                if eval <= alpha {
                    delta_low *= ASP_EXPANSION_FACTOR;
                    alpha = (last_eval - delta_low).max(EVAL_MIN);
                } else if eval >= beta {
                    delta_high *= ASP_EXPANSION_FACTOR;
                    beta = (last_eval + delta_high).min(EVAL_MAX);
                }
            }

            eval_final
        };

        if stopper.should_stop(&report) {
            break;
        }

        report.depth = depth;
        report.pv = Some(sanitise_pv(pos.clone(), (pv.clone(), last_eval)));
        report.tt_stats = (tt.usage, tt.capacity);

        reporter.send(&report);
    }
}

fn sanitise_pv(mut pos: Position, (moves, eval): (MoveList, i32)) -> (MoveList, i32) {
    for (index, mv) in moves.iter().enumerate() {
        pos.do_move(mv);

        if pos.is_fifty_move_draw() || pos.is_repetition_draw(0) {
            return (MoveList::from_slice(&moves[..=index]), EVAL_DRAW);
        }
    }

    (moves, eval)
}
