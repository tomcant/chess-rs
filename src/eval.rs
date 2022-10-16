use crate::board::Board;
use crate::colour::Colour;
use crate::piece::PieceType;
use crate::position::Position;

const PIECE_WEIGHTS: [i32; 6] = [1, 3, 3, 5, 9, 0];

pub const EVAL_MAX: i32 = 10_000;
pub const EVAL_MIN: i32 = -EVAL_MAX;
pub const EVAL_STALEMATE: i32 = 0;

pub trait Evaluator {
    fn evaluate(&self) -> i32;
}

impl Evaluator for Position {
    fn evaluate(&self) -> i32 {
        let material_diff = count_material(Colour::White, &self.board) - count_material(Colour::Black, &self.board);

        match self.colour_to_move {
            Colour::White => material_diff,
            _ => -material_diff,
        }
    }
}

fn count_material(colour: Colour, board: &Board) -> i32 {
    PieceType::types().iter().fold(0, |acc, piece_type| {
        acc + PIECE_WEIGHTS[*piece_type as usize] * board.pieces(*piece_type, colour).count_ones() as i32
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fen::START_POS_FEN;

    #[test]
    fn evaluate_an_even_position() {
        let pos = parse_fen(START_POS_FEN);

        assert_eq!(pos.evaluate(), 0);
    }

    #[test]
    fn evaluate_a_material_advantage() {
        let pos = parse_fen("4kbnr/8/8/8/8/8/4P3/3QKBNR w - - 0 1");

        assert_eq!(
            pos.evaluate(),
            PIECE_WEIGHTS[PieceType::Pawn as usize] + PIECE_WEIGHTS[PieceType::Queen as usize]
        );
    }

    #[test]
    fn evaluate_a_material_disadvantage() {
        let pos = parse_fen("3qkbnr/4p3/8/8/8/8/4P3/3QKB2 w - - 0 1");

        assert_eq!(
            pos.evaluate(),
            -(PIECE_WEIGHTS[PieceType::Knight as usize] + PIECE_WEIGHTS[PieceType::Rook as usize])
        );
    }

    fn parse_fen(str: &str) -> Position {
        let pos = str.parse();
        assert!(pos.is_ok());

        pos.unwrap()
    }
}
