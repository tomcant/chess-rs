mod castling;
mod fen;

use crate::board::{Board, Colour, Square};
use crate::move_generator::Move;
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

impl GameState {
    pub fn do_move(&mut self, mv: &Move) {
        let piece = self.board.get_piece_at(mv.from).unwrap();

        self.board.put_piece(piece, mv.to);
        self.board.clear_square(mv.from);

        self.colour_to_move = self.colour_to_move.flip();
    }

    pub fn undo_move(&mut self, mv: &Move) {
        let piece = self.board.get_piece_at(mv.to).unwrap();

        self.board.put_piece(piece, mv.from);
        self.board.clear_square(mv.to);

        self.colour_to_move = self.colour_to_move.flip();
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        board::{Piece, Square},
        game_state::GameState,
        move_generator::Move,
    };

    #[test]
    fn test_do_move() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let mut state: GameState = fen.parse().unwrap();

        let mv = Move {
            from: "e2".parse::<Square>().unwrap(),
            to: "e4".parse::<Square>().unwrap(),
            captured: None,
            promoted: None,
        };

        assert_eq!(state.board.get_piece_at(mv.from), Some(Piece::WhitePawn));

        state.do_move(&mv);

        assert_eq!(state.board.get_piece_at(mv.to), Some(Piece::WhitePawn));
        assert!(!state.board.has_piece_at(mv.from));
    }

    #[test]
    fn test_undo_move() {
        let fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 1";
        let mut state: GameState = fen.parse().unwrap();

        let mv = Move {
            from: "e2".parse::<Square>().unwrap(),
            to: "e4".parse::<Square>().unwrap(),
            captured: None,
            promoted: None,
        };

        assert_eq!(state.board.get_piece_at(mv.to), Some(Piece::WhitePawn));

        state.undo_move(&mv);

        assert_eq!(state.board.get_piece_at(mv.from), Some(Piece::WhitePawn));
        assert!(!state.board.has_piece_at(mv.to));
    }
}
