use crate::board::Board;
use crate::colour::Colour;
use crate::game::GameState;
use crate::piece::PieceType;

const PIECE_WEIGHTS: [i32; 6] = [1, 3, 3, 5, 8, 0];

pub trait Evaluator {
    fn evaluate(&self) -> i32;
}

impl Evaluator for GameState {
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

    #[test]
    fn evaluate_the_start_position() {
        let state = parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

        assert_eq!(state.evaluate(), 0);
    }

    #[test]
    fn evaluate_a_material_advantage() {
        let state = parse_fen("4kbnr/8/8/8/8/8/4P3/3QKBNR w - - 0 1");

        assert_eq!(
            state.evaluate(),
            PIECE_WEIGHTS[PieceType::Pawn as usize] + PIECE_WEIGHTS[PieceType::Queen as usize]
        );
    }

    #[test]
    fn evaluate_a_material_disadvantage() {
        let state = parse_fen("3qkbnr/4p3/8/8/8/8/4P3/3QKB2 w - - 0 1");

        assert_eq!(
            state.evaluate(),
            -(PIECE_WEIGHTS[PieceType::Knight as usize] + PIECE_WEIGHTS[PieceType::Rook as usize])
        );
    }

    fn parse_fen(str: &str) -> GameState {
        let state = str.parse();
        assert!(state.is_ok());

        state.unwrap()
    }
}
