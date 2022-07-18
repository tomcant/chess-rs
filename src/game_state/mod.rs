mod castling;
mod fen;

use crate::board::{Board, Colour, Square};
use castling::CastlingAbility;

#[derive(Debug)]
pub struct GameState {
    board: Board,
    colour_to_move: Colour,
    castling_ability: CastlingAbility,
    en_passant_square: Option<Square>,
    half_move_clock: u8,
    full_move_counter: u8,
}
