use crate::colour::Colour::*;
use crate::position::Position;
use crate::search::MAX_DEPTH;

pub mod material;
mod psqt;

pub const EVAL_MAX: i32 = 10_000;
pub const EVAL_MIN: i32 = -EVAL_MAX;
pub const EVAL_DRAW: i32 = 0;
pub const EVAL_CHECKMATE: i32 = EVAL_MAX;
pub const EVAL_CHECKMATE_THRESHOLD: i32 = EVAL_CHECKMATE - MAX_DEPTH as i32;

pub fn eval(pos: &Position) -> i32 {
    let eval = (material::eval(White, &pos.board) - material::eval(Black, &pos.board))
        + (psqt::eval(White, &pos.board) - psqt::eval(Black, &pos.board));

    match pos.colour_to_move {
        White => eval,
        _ => -eval,
    }
}
