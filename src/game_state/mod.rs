mod castling;
mod fen;

use crate::board::{Board, Colour, Square};
use castling::CastlingAbility;

#[derive(Debug)]
pub struct GameState {
    pub board: Board,
    pub colour_to_move: Colour,
    pub castling_ability: CastlingAbility,
    pub en_passant_square: Option<Square>,
    pub half_move_clock: u8,
    pub full_move_counter: u8,
}
