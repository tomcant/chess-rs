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
        if mv.is_capture() {
            self.board.clear_square(mv.get_capture_square());
        }

        self.en_passant_square = None;
        let piece = self.board.get_piece_at(mv.from).unwrap();

        if piece.is_pawn() && mv.from.rank().abs_diff(mv.to.rank()) == 2 {
            self.en_passant_square = Some(mv.from.up_for_colour(self.colour_to_move));
        }

        self.board.put_piece(piece, mv.to);
        self.board.clear_square(mv.from);

        self.colour_to_move = self.colour_to_move.flip();
        self.full_move_counter += 1;
    }

    pub fn undo_move(&mut self, mv: &Move) {
        self.board.put_piece(self.board.get_piece_at(mv.to).unwrap(), mv.from);
        self.board.clear_square(mv.to);

        if mv.is_capture() {
            self.board
                .put_piece(mv.captured_piece.unwrap(), mv.get_capture_square());

            if mv.is_en_passant {
                self.en_passant_square = Some(mv.to);
            }
        }

        self.colour_to_move = self.colour_to_move.flip();
        self.full_move_counter -= 1;
    }

    pub fn can_capture_en_passant(&self, pawn_square: Square) -> bool {
        if let Some(square) = self.en_passant_square {
            return pawn_square.file().abs_diff(square.file()) == 1
                && pawn_square.rank() == square.up_for_colour(self.colour_to_move.flip()).rank();
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::GameState;
    use crate::piece::Piece;

    #[test]
    fn apply_a_non_capture_move() {
        let mut state = parse_fen("8/8/8/8/8/8/4P3/8 w - - 0 1");

        let mv = Move {
            from: parse_square("e2"),
            to: parse_square("e4"),
            captured_piece: None,
            promotion_piece: None,
            is_en_passant: false,
        };

        state.do_move(&mv);

        assert_eq!(state.board.get_piece_at(mv.to), Some(Piece::WhitePawn));
        assert!(!state.board.has_piece_at(mv.from));
        assert_eq!(state.colour_to_move, Colour::Black);
    }

    #[test]
    fn apply_a_capture_move() {
        let mut state = parse_fen("8/8/8/5p2/3N4/8/8/8 w - - 0 1");

        let mv = Move {
            from: parse_square("d4"),
            to: parse_square("f5"),
            captured_piece: Some(Piece::BlackPawn),
            promotion_piece: None,
            is_en_passant: false,
        };

        state.do_move(&mv);

        assert_eq!(state.board.get_piece_at(mv.to), Some(Piece::WhiteKnight));
        assert!(!state.board.has_piece_at(mv.from));
    }

    #[test]
    fn apply_an_en_passant_capture_move() {
        let mut state = parse_fen("8/8/8/3Pp3/8/8/8/8 w - e6 0 1");

        let mv = Move {
            from: parse_square("d5"),
            to: parse_square("e6"),
            captured_piece: Some(Piece::BlackPawn),
            promotion_piece: None,
            is_en_passant: true,
        };

        state.do_move(&mv);

        assert_eq!(state.board.get_piece_at(mv.to), Some(Piece::WhitePawn));
        assert!(!state.board.has_piece_at(parse_square("e5")));
        assert!(!state.board.has_piece_at(mv.from));
    }

    #[test]
    fn undo_a_non_capture_move() {
        let mut state = parse_fen("8/8/8/8/4P3/8/8/8 b - - 0 1");

        let mv = Move {
            from: parse_square("e2"),
            to: parse_square("e4"),
            captured_piece: None,
            promotion_piece: None,
            is_en_passant: false,
        };

        state.undo_move(&mv);

        assert_eq!(state.board.get_piece_at(mv.from), Some(Piece::WhitePawn));
        assert!(!state.board.has_piece_at(mv.to));
        assert_eq!(state.colour_to_move, Colour::White);
    }

    #[test]
    fn undo_a_capture_move() {
        let mut state = parse_fen("8/8/8/5N2/8/8/8/8 b - - 0 1");

        let mv = Move {
            from: parse_square("d4"),
            to: parse_square("f5"),
            captured_piece: Some(Piece::BlackPawn),
            promotion_piece: None,
            is_en_passant: false,
        };

        state.undo_move(&mv);

        assert_eq!(state.board.get_piece_at(mv.from), Some(Piece::WhiteKnight));
        assert_eq!(state.board.get_piece_at(mv.to), Some(Piece::BlackPawn));
    }

    #[test]
    fn undo_an_en_passant_capture_move() {
        let mut state = parse_fen("8/8/4P3/8/8/8/8/8 b - - 0 1");

        let mv = Move {
            from: parse_square("d5"),
            to: parse_square("e6"),
            captured_piece: Some(Piece::BlackPawn),
            promotion_piece: None,
            is_en_passant: true,
        };

        state.undo_move(&mv);

        assert_eq!(state.en_passant_square, Some(mv.to));
        assert_eq!(state.board.get_piece_at(mv.from), Some(Piece::WhitePawn));
        assert_eq!(state.board.get_piece_at(parse_square("e5")), Some(Piece::BlackPawn));
        assert!(!state.board.has_piece_at(mv.to));
    }

    #[test]
    fn set_the_en_passant_square_for_a_white_double_pawn_advance() {
        let mut state = parse_fen("8/8/8/8/8/8/4P3/8 w - - 0 1");

        let mv = Move {
            from: parse_square("e2"),
            to: parse_square("e4"),
            captured_piece: None,
            promotion_piece: None,
            is_en_passant: false,
        };

        state.do_move(&mv);

        assert_eq!(state.en_passant_square, Some(parse_square("e3")));
    }

    #[test]
    fn set_the_en_passant_square_for_a_black_double_pawn_advance() {
        let mut state = parse_fen("8/4p3/8/8/8/8/8/8 b - - 0 1");

        let mv = Move {
            from: parse_square("e7"),
            to: parse_square("e5"),
            captured_piece: None,
            promotion_piece: None,
            is_en_passant: false,
        };

        state.do_move(&mv);

        assert_eq!(state.en_passant_square, Some(parse_square("e6")));
    }

    #[test]
    fn reset_the_en_passant_square_when_undoing_a_double_pawn_advance() {
        let mut state = parse_fen("8/8/8/8/4P3/8/8/8 b - - 0 1");

        let mv = Move {
            from: parse_square("e2"),
            to: parse_square("e4"),
            captured_piece: None,
            promotion_piece: None,
            is_en_passant: false,
        };

        state.undo_move(&mv);

        assert_eq!(state.en_passant_square, None);
    }

    fn parse_fen(str: &str) -> GameState {
        let state = str.parse();
        assert!(state.is_ok());

        state.unwrap()
    }

    fn parse_square(str: &str) -> Square {
        let square = str.parse();
        assert!(square.is_ok());

        square.unwrap()
    }
}
