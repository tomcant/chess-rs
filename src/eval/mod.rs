use crate::colour::Colour::*;
use crate::position::Position;
use crate::search::MAX_DEPTH;

pub mod material;
mod phase;
mod psqt;

use phase::{MAX_PHASE, phase};

pub const EVAL_MAX: i32 = 10_000;
pub const EVAL_MIN: i32 = -EVAL_MAX;
pub const EVAL_DRAW: i32 = 0;
pub const EVAL_MATE: i32 = EVAL_MAX;
pub const EVAL_MATE_THRESHOLD: i32 = EVAL_MATE - MAX_DEPTH as i32;

pub fn eval(pos: &Position) -> i32 {
    let material = material::eval(White, &pos.board) - material::eval(Black, &pos.board);
    let psqt_non_king = psqt::eval_non_king(White, &pos.board) - psqt::eval_non_king(Black, &pos.board);

    // King PSQT is the only tapered (MG/EG) term for now.
    let psqt_mg_king = psqt::eval_king_mg(White, &pos.board) - psqt::eval_king_mg(Black, &pos.board);
    let psqt_eg_king = psqt::eval_king_eg(White, &pos.board) - psqt::eval_king_eg(Black, &pos.board);

    let eval_mg = material + psqt_non_king + psqt_mg_king;
    let eval_eg = material + psqt_non_king + psqt_eg_king;

    let phase = phase(&pos.board);
    let eval = (eval_mg * phase + eval_eg * (MAX_PHASE - phase)) / MAX_PHASE;

    match pos.colour_to_move {
        White => eval,
        _ => -eval,
    }
}
