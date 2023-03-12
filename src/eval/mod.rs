use crate::colour::Colour;
use crate::piece::PieceType;
use crate::position::{Board, Position};

mod material;
mod psqt;

pub const EVAL_MAX: i32 = 10_000;
pub const EVAL_MIN: i32 = -EVAL_MAX;
pub const EVAL_STALEMATE: i32 = 0;
pub const EVAL_CHECKMATE: i32 = 9_900;

pub fn eval(pos: &Position) -> i32 {
    let eval = (material::eval(Colour::White, &pos.board) - material::eval(Colour::Black, &pos.board))
        + (psqt::eval(Colour::White, &pos.board) - psqt::eval(Colour::Black, &pos.board));

    match pos.colour_to_move {
        Colour::White => eval,
        _ => -eval,
    }
}
