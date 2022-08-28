use crate::board::Board;
use crate::colour::Colour;
use crate::r#move::Move;
use crate::square::Square;

bitflags::bitflags! {
    pub struct CastlingAbility: u8 {
        const NONE = 0;
        const WHITE_KING = 1;
        const WHITE_QUEEN = 2;
        const BLACK_KING = 4;
        const BLACK_QUEEN = 8;
        const ALL = 15;
    }
}

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

        self.board.clear_square(mv.from);

        if mv.is_capture() {
            self.board.clear_square(mv.to);
        }

        self.board.put_piece(piece, mv.to);
        self.colour_to_move = self.colour_to_move.flip();
    }

    pub fn undo_move(&mut self, mv: &Move) {
        let piece = self.board.get_piece_at(mv.to).unwrap();

        self.board.clear_square(mv.to);

        if mv.is_capture() {
            self.board.put_piece(mv.captured.unwrap(), mv.to);
        }

        self.board.put_piece(piece, mv.from);
        self.colour_to_move = self.colour_to_move.flip();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::GameState;
    use crate::piece::Piece;

    #[test]
    fn test_do_move_non_capture() {
        let fen = "8/8/8/8/8/8/4P3/8 w - - 0 1";
        let mut state: GameState = fen.parse().unwrap();

        let mv = Move {
            from: "e2".parse::<Square>().unwrap(),
            to: "e4".parse::<Square>().unwrap(),
            captured: None,
            promoted: None,
        };

        state.do_move(&mv);

        assert_eq!(state.board.get_piece_at(mv.to), Some(Piece::WhitePawn));
        assert!(!state.board.has_piece_at(mv.from));
        assert_eq!(state.colour_to_move, Colour::Black);
    }

    #[test]
    fn test_do_move_capture() {
        let fen = "8/8/8/5p2/3N4/8/8/8 w - - 0 1";
        let mut state: GameState = fen.parse().unwrap();

        let mv = Move {
            from: "d4".parse::<Square>().unwrap(),
            to: "f5".parse::<Square>().unwrap(),
            captured: Some(Piece::BlackPawn),
            promoted: None,
        };

        state.do_move(&mv);

        assert_eq!(state.board.get_piece_at(mv.to), Some(Piece::WhiteKnight));
        assert!(!state.board.has_piece_at(mv.from));
    }

    #[test]
    fn test_undo_move_non_capture() {
        let fen = "8/8/8/8/4P3/8/8/8 b - - 0 1";
        let mut state: GameState = fen.parse().unwrap();

        let mv = Move {
            from: "e2".parse::<Square>().unwrap(),
            to: "e4".parse::<Square>().unwrap(),
            captured: None,
            promoted: None,
        };

        state.undo_move(&mv);

        assert_eq!(state.board.get_piece_at(mv.from), Some(Piece::WhitePawn));
        assert!(!state.board.has_piece_at(mv.to));
        assert_eq!(state.colour_to_move, Colour::White);
    }

    #[test]
    fn test_undo_move_capture() {
        let fen = "8/8/8/5N2/8/8/8/8 b - - 0 1";
        let mut state: GameState = fen.parse().unwrap();

        let mv = Move {
            from: "d4".parse::<Square>().unwrap(),
            to: "f5".parse::<Square>().unwrap(),
            captured: Some(Piece::BlackPawn),
            promoted: None,
        };

        state.undo_move(&mv);

        assert_eq!(state.board.get_piece_at(mv.from), Some(Piece::WhiteKnight));
        assert_eq!(state.board.get_piece_at(mv.to), Some(Piece::BlackPawn));
    }
}
