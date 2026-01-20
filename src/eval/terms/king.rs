use super::EvalTerm;
use crate::colour::Colour;
use crate::piece::Piece;
use crate::position::Board;
use crate::square::Square;
use lazy_static::lazy_static;

const PAWN_SHIELD_CLOSE: i32 = 12;
const PAWN_SHIELD_FAR: i32 = 6;

pub fn eval(colour: Colour, board: &Board) -> EvalTerm {
    let king_square = Square::first(board.pieces(Piece::king(colour)));

    // Evaluate pawn shields when the king is out of the centre.
    if (3..5).contains(&king_square.file()) {
        return EvalTerm::zero();
    }

    let (close, far) = PAWN_SHIELDS[colour][king_square];
    let pawns = board.pieces(Piece::pawn(colour));
    let close_pawns = (pawns & close).count_ones() as i32;
    let far_pawns = (pawns & far).count_ones() as i32;

    EvalTerm::new(close_pawns * PAWN_SHIELD_CLOSE + far_pawns * PAWN_SHIELD_FAR, 0)
}

lazy_static! {
    static ref PAWN_SHIELDS: [[(u64, u64); 64]; 2] =
        [build_pawn_shields(Colour::White), build_pawn_shields(Colour::Black)];
}

fn build_pawn_shields(colour: Colour) -> [(u64, u64); 64] {
    let mut masks = [(0, 0); 64];
    let squares: [_; 64] = std::array::from_fn(|index| Square::from_index(index as u8));

    for square in squares {
        let rank = square.rank();
        let close_rank = match colour {
            Colour::White if rank < 7 => Some(rank + 1),
            Colour::Black if rank > 0 => Some(rank - 1),
            _ => None,
        };
        let far_rank = match colour {
            Colour::White if rank < 6 => Some(rank + 2),
            Colour::Black if rank > 1 => Some(rank - 2),
            _ => None,
        };

        let (mut close, mut far) = (0, 0);

        for diff in [-1, 0, 1] {
            let file = square.file() as i8 + diff;

            if (0..8).contains(&file) {
                if let Some(rank) = close_rank {
                    close |= Square::from_file_and_rank(file as u8, rank).u64();
                }
                if let Some(rank) = far_rank {
                    far |= Square::from_file_and_rank(file as u8, rank).u64();
                }
            }
        }

        masks[square] = (close, far);
    }

    masks
}
