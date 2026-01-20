use super::EvalTerm;
use crate::colour::Colour;
use crate::piece::Piece;
use crate::position::Board;
use crate::square::{FILES, Square};

const OPEN_FILE_MG: i32 = 15;
const OPEN_FILE_EG: i32 = 10;

const HALF_OPEN_FILE_MG: i32 = 8;
const HALF_OPEN_FILE_EG: i32 = 6;

pub fn eval(colour: Colour, board: &Board) -> EvalTerm {
    let (mut mg, mut eg) = (0, 0);
    let mut our_rooks = board.pieces(Piece::rook(colour));
    let our_pawns = board.pieces(Piece::pawn(colour));
    let all_pawns = our_pawns | board.pieces(Piece::pawn(colour.flip()));

    while our_rooks != 0 {
        let square = Square::next(&mut our_rooks);
        let file = FILES[square.file() as usize];

        if all_pawns & file == 0 {
            mg += OPEN_FILE_MG;
            eg += OPEN_FILE_EG;
        } else if our_pawns & file == 0 {
            mg += HALF_OPEN_FILE_MG;
            eg += HALF_OPEN_FILE_EG;
        }
    }

    EvalTerm::new(mg, eg)
}
