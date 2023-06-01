use crate::colour::Colour;
use crate::movegen::Move;
use crate::piece::Piece;
use crate::square::Square;

mod board;
mod castling;
mod fen;

pub use board::Board;
pub use castling::{CastlingRight, CastlingRights};
pub use fen::START_POS_FEN;

#[derive(Debug)]
pub struct Position {
    pub board: Board,
    pub colour_to_move: Colour,
    pub castling_rights: CastlingRights,
    pub en_passant_square: Option<Square>,
    pub half_move_clock: u8,
    pub full_move_counter: u8,
}

impl Position {
    pub fn startpos() -> Self {
        START_POS_FEN.parse().unwrap()
    }

    pub fn do_move(&mut self, mv: &Move) {
        self.half_move_clock += 1;
        self.en_passant_square = None;

        if let Some(capture_square) = mv.capture_square() {
            self.board.remove_piece(capture_square);
            self.half_move_clock = 0;
        }

        let piece = mv
            .promotion_piece
            .unwrap_or_else(|| self.board.piece_at(mv.from).unwrap());

        if piece.is_pawn() {
            self.half_move_clock = 0;

            if mv.rank_diff() == 2 {
                self.en_passant_square = Some(mv.from.advance(self.colour_to_move));
            }
        }

        if piece.is_king() {
            self.castling_rights.remove_for_colour(self.colour_to_move);

            if mv.file_diff() > 1 {
                let rook = Piece::rook(self.colour_to_move);

                match mv.to.file() {
                    2 => {
                        self.board.put_piece(rook, Square::from_file_and_rank(3, mv.to.rank()));
                        self.board.remove_piece(Square::from_file_and_rank(0, mv.to.rank()));
                    }
                    6 => {
                        self.board.put_piece(rook, Square::from_file_and_rank(5, mv.to.rank()));
                        self.board.remove_piece(Square::from_file_and_rank(7, mv.to.rank()));
                    }
                    _ => unreachable!(),
                };
            }
        }

        if mv.from.is_corner() {
            self.castling_rights.remove_for_square(mv.from);
        }

        if mv.to.is_corner() {
            self.castling_rights.remove_for_square(mv.to);
        }

        self.board.put_piece(piece, mv.to);
        self.board.remove_piece(mv.from);

        if self.colour_to_move == Colour::Black {
            self.full_move_counter += 1;
        }

        self.colour_to_move = self.opponent_colour();
    }

    pub fn undo_move(&mut self, mv: &Move) {
        let piece = match mv.promotion_piece {
            Some(piece) => Piece::pawn(piece.colour()),
            None => self.board.piece_at(mv.to).unwrap(),
        };

        if piece.is_king() && mv.file_diff() > 1 {
            let rook = Piece::rook(self.opponent_colour());

            match mv.to.file() {
                2 => {
                    self.board.put_piece(rook, Square::from_file_and_rank(0, mv.to.rank()));
                    self.board.remove_piece(Square::from_file_and_rank(3, mv.to.rank()));
                }
                6 => {
                    self.board.put_piece(rook, Square::from_file_and_rank(7, mv.to.rank()));
                    self.board.remove_piece(Square::from_file_and_rank(5, mv.to.rank()));
                }
                _ => unreachable!(),
            };
        }

        self.board.put_piece(piece, mv.from);
        self.board.remove_piece(mv.to);

        self.castling_rights = mv.castling_rights;
        self.half_move_clock = mv.half_move_clock;
        self.en_passant_square = None;

        if let Some(capture_square) = mv.capture_square() {
            self.board.put_piece(mv.captured_piece.unwrap(), capture_square);

            if mv.is_en_passant {
                self.en_passant_square = Some(mv.to);
            }
        }

        self.colour_to_move = self.opponent_colour();

        if self.colour_to_move == Colour::Black {
            self.full_move_counter -= 1;
        }
    }

    pub fn opponent_colour(&self) -> Colour {
        self.colour_to_move.flip()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::piece::Piece;

    #[test]
    fn move_a_piece() {
        let mut pos = parse_fen("8/8/8/8/8/8/8/5R2 w - - 0 1");

        let mv = Move {
            from: Square::F1,
            to: parse_square("f4"),
            captured_piece: None,
            promotion_piece: None,
            castling_rights: pos.castling_rights,
            half_move_clock: pos.half_move_clock,
            is_en_passant: false,
        };

        pos.do_move(&mv);

        assert_eq!(pos.board.piece_at(mv.to), Some(Piece::WR));
        assert!(!pos.board.has_piece_at(mv.from));
        assert_eq!(pos.colour_to_move, Colour::Black);
    }

    #[test]
    fn undo_moving_a_piece() {
        let mut pos = parse_fen("8/8/8/8/5R2/8/8/8 b - - 1 1");

        let mv = Move {
            from: Square::F1,
            to: parse_square("f4"),
            captured_piece: None,
            promotion_piece: None,
            castling_rights: CastlingRights::none(),
            half_move_clock: 0,
            is_en_passant: false,
        };

        pos.undo_move(&mv);

        assert_eq!(pos.board.piece_at(mv.from), Some(Piece::WR));
        assert!(!pos.board.has_piece_at(mv.to));
        assert_eq!(pos.colour_to_move, Colour::White);
    }

    #[test]
    fn capture_a_piece() {
        let mut pos = parse_fen("8/8/8/5p2/3N4/8/8/8 w - - 0 1");

        let mv = Move {
            from: parse_square("d4"),
            to: parse_square("f5"),
            captured_piece: Some(Piece::BP),
            promotion_piece: None,
            castling_rights: pos.castling_rights,
            half_move_clock: pos.half_move_clock,
            is_en_passant: false,
        };

        pos.do_move(&mv);

        assert_eq!(pos.board.piece_at(mv.to), Some(Piece::WN));
        assert!(!pos.board.has_piece_at(mv.from));
    }

    #[test]
    fn undo_capturing_a_piece() {
        let mut pos = parse_fen("8/8/8/5N2/8/8/8/8 b - - 1 1");

        let mv = Move {
            from: parse_square("d4"),
            to: parse_square("f5"),
            captured_piece: Some(Piece::BP),
            promotion_piece: None,
            castling_rights: CastlingRights::none(),
            half_move_clock: 0,
            is_en_passant: false,
        };

        pos.undo_move(&mv);

        assert_eq!(pos.board.piece_at(mv.from), Some(Piece::WN));
        assert_eq!(pos.board.piece_at(mv.to), Some(Piece::BP));
    }

    #[test]
    fn castle_king_side() {
        let mut pos = parse_fen("8/8/8/8/8/8/8/4K2R w K - 0 1");

        let mv = Move {
            from: Square::E1,
            to: Square::G1,
            captured_piece: None,
            promotion_piece: None,
            castling_rights: pos.castling_rights,
            half_move_clock: pos.half_move_clock,
            is_en_passant: false,
        };

        pos.do_move(&mv);

        assert_eq!(pos.castling_rights, CastlingRights::none());

        assert_eq!(pos.board.piece_at(mv.to), Some(Piece::WK));
        assert_eq!(pos.board.piece_at(Square::F1), Some(Piece::WR));

        assert!(!pos.board.has_piece_at(mv.from));
        assert!(!pos.board.has_piece_at(Square::H1));
    }

    #[test]
    fn undo_castle_king_side() {
        let mut pos = parse_fen("8/8/8/8/8/8/8/5RK1 b - - 1 1");

        let mv = Move {
            from: Square::E1,
            to: Square::G1,
            captured_piece: None,
            promotion_piece: None,
            castling_rights: CastlingRights::from(&[CastlingRight::WhiteKing]),
            half_move_clock: 0,
            is_en_passant: false,
        };

        pos.undo_move(&mv);

        assert_eq!(pos.castling_rights, CastlingRights::from(&[CastlingRight::WhiteKing]));

        assert_eq!(pos.board.piece_at(mv.from), Some(Piece::WK));
        assert_eq!(pos.board.piece_at(Square::H1), Some(Piece::WR));

        assert!(!pos.board.has_piece_at(mv.to));
        assert!(!pos.board.has_piece_at(Square::F1));
    }

    #[test]
    fn moving_a_rook_removes_the_relevant_castling_rights() {
        let mut pos = parse_fen("8/8/8/8/8/8/8/R3K2R w KQ - 0 1");

        let mv = Move {
            from: Square::H1,
            to: Square::G1,
            captured_piece: None,
            promotion_piece: None,
            castling_rights: CastlingRights::from(&[CastlingRight::WhiteKing, CastlingRight::WhiteQueen]),
            half_move_clock: pos.half_move_clock,
            is_en_passant: false,
        };

        pos.do_move(&mv);

        assert_eq!(pos.castling_rights, CastlingRights::from(&[CastlingRight::WhiteQueen]));
    }

    #[test]
    fn capturing_a_rook_removes_the_relevant_castling_rights() {
        let mut pos = parse_fen("8/8/8/8/3b4/8/8/R3K2R b KQ - 0 1");

        let mv = Move {
            from: parse_square("d4"),
            to: Square::A1,
            captured_piece: Some(Piece::WR),
            promotion_piece: None,
            castling_rights: CastlingRights::from(&[CastlingRight::WhiteKing, CastlingRight::WhiteQueen]),
            half_move_clock: pos.half_move_clock,
            is_en_passant: false,
        };

        pos.do_move(&mv);

        assert_eq!(pos.castling_rights, CastlingRights::from(&[CastlingRight::WhiteKing]));
    }

    #[test]
    fn promote_a_pawn() {
        let mut pos = parse_fen("8/4P3/8/8/8/8/8/8 w - - 0 1");

        let mv = Move {
            from: parse_square("e7"),
            to: Square::E8,
            captured_piece: None,
            promotion_piece: Some(Piece::WN),
            castling_rights: pos.castling_rights,
            half_move_clock: pos.half_move_clock,
            is_en_passant: false,
        };

        pos.do_move(&mv);

        assert_eq!(pos.board.piece_at(mv.to), mv.promotion_piece);
        assert!(!pos.board.has_piece_at(mv.from));
    }

    #[test]
    fn undo_promoting_a_pawn() {
        let mut pos = parse_fen("4N3/8/8/8/8/8/8/8 b - - 0 1");

        let mv = Move {
            from: parse_square("e7"),
            to: Square::E8,
            captured_piece: None,
            promotion_piece: Some(Piece::WN),
            castling_rights: CastlingRights::none(),
            half_move_clock: 1,
            is_en_passant: false,
        };

        pos.undo_move(&mv);

        assert_eq!(pos.board.piece_at(mv.from), Some(Piece::WP));
        assert!(!pos.board.has_piece_at(mv.to));
    }

    #[test]
    fn undo_promoting_a_pawn_with_capture() {
        let mut pos = parse_fen("3B4/8/8/8/8/8/8/8 b - - 0 1");

        let mv = Move {
            from: parse_square("e7"),
            to: Square::D8,
            captured_piece: Some(Piece::BQ),
            promotion_piece: Some(Piece::WB),
            castling_rights: CastlingRights::none(),
            half_move_clock: 1,
            is_en_passant: false,
        };

        pos.undo_move(&mv);

        assert_eq!(pos.board.piece_at(mv.from), Some(Piece::WP));
        assert_eq!(pos.board.piece_at(mv.to), mv.captured_piece);
    }

    #[test]
    fn capture_a_pawn_en_passant() {
        let mut pos = parse_fen("8/8/8/3Pp3/8/8/8/8 w - e6 0 1");

        let mv = Move {
            from: parse_square("d5"),
            to: parse_square("e6"),
            captured_piece: Some(Piece::BP),
            promotion_piece: None,
            castling_rights: pos.castling_rights,
            half_move_clock: pos.half_move_clock,
            is_en_passant: true,
        };

        pos.do_move(&mv);

        assert_eq!(pos.board.piece_at(mv.to), Some(Piece::WP));
        assert!(!pos.board.has_piece_at(parse_square("e5")));
        assert!(!pos.board.has_piece_at(mv.from));
    }

    #[test]
    fn undo_capturing_a_pawn_en_passant() {
        let mut pos = parse_fen("8/8/4P3/8/8/8/8/8 b - - 0 1");

        let mv = Move {
            from: parse_square("d5"),
            to: parse_square("e6"),
            captured_piece: Some(Piece::BP),
            promotion_piece: None,
            castling_rights: CastlingRights::none(),
            half_move_clock: 1,
            is_en_passant: true,
        };

        pos.undo_move(&mv);

        assert_eq!(pos.en_passant_square, Some(mv.to));
        assert_eq!(pos.board.piece_at(mv.from), Some(Piece::WP));
        assert_eq!(pos.board.piece_at(parse_square("e5")), Some(Piece::BP));
        assert!(!pos.board.has_piece_at(mv.to));
    }

    #[test]
    fn set_the_en_passant_square_for_a_white_double_pawn_advance() {
        let mut pos = parse_fen("8/8/8/8/8/8/4P3/8 w - - 0 1");

        let mv = Move {
            from: parse_square("e2"),
            to: parse_square("e4"),
            captured_piece: None,
            promotion_piece: None,
            castling_rights: pos.castling_rights,
            half_move_clock: pos.half_move_clock,
            is_en_passant: false,
        };

        pos.do_move(&mv);

        assert_eq!(pos.en_passant_square, Some(parse_square("e3")));
    }

    #[test]
    fn set_the_en_passant_square_for_a_black_double_pawn_advance() {
        let mut pos = parse_fen("8/4p3/8/8/8/8/8/8 b - - 0 1");

        let mv = Move {
            from: parse_square("e7"),
            to: parse_square("e5"),
            captured_piece: None,
            promotion_piece: None,
            castling_rights: pos.castling_rights,
            half_move_clock: pos.half_move_clock,
            is_en_passant: false,
        };

        pos.do_move(&mv);

        assert_eq!(pos.en_passant_square, Some(parse_square("e6")));
    }

    #[test]
    fn reset_the_en_passant_square_when_undoing_a_double_pawn_advance() {
        let mut pos = parse_fen("8/8/8/8/4P3/8/8/8 b - e3 0 1");

        let mv = Move {
            from: parse_square("e2"),
            to: parse_square("e4"),
            captured_piece: None,
            promotion_piece: None,
            castling_rights: CastlingRights::none(),
            half_move_clock: 1,
            is_en_passant: false,
        };

        pos.undo_move(&mv);

        assert_eq!(pos.en_passant_square, None);
    }

    #[test]
    fn increment_the_half_move_clock_for_non_pawn_or_non_capture_moves() {
        let mut pos = parse_fen("8/4p3/8/8/8/8/4P3/4k3 w - - 0 1");

        let mv = Move {
            from: parse_square("e1"),
            to: parse_square("f2"),
            captured_piece: None,
            promotion_piece: None,
            castling_rights: pos.castling_rights,
            half_move_clock: pos.half_move_clock,
            is_en_passant: false,
        };

        pos.do_move(&mv);

        assert_eq!(pos.half_move_clock, 1);
    }

    #[test]
    fn reset_the_half_move_clock_when_a_pawn_moves() {
        let mut pos = parse_fen("8/4p3/8/8/8/8/4P3/8 w - - 1 1");

        let mv = Move {
            from: parse_square("e2"),
            to: parse_square("e4"),
            captured_piece: None,
            promotion_piece: None,
            castling_rights: pos.castling_rights,
            half_move_clock: pos.half_move_clock,
            is_en_passant: false,
        };

        pos.do_move(&mv);

        assert_eq!(pos.half_move_clock, 0);
    }

    #[test]
    fn reset_the_half_move_clock_when_a_piece_is_captured() {
        let mut pos = parse_fen("8/4p3/8/8/8/8/4Q3/8 w - - 1 1");

        let mv = Move {
            from: parse_square("e2"),
            to: parse_square("e7"),
            captured_piece: Some(Piece::BP),
            promotion_piece: None,
            castling_rights: pos.castling_rights,
            half_move_clock: pos.half_move_clock,
            is_en_passant: false,
        };

        pos.do_move(&mv);

        assert_eq!(pos.half_move_clock, 0);
    }

    #[test]
    fn increment_the_full_move_counter_when_black_moves() {
        let mut pos = parse_fen("8/4p3/8/8/8/8/4P3/8 w - - 0 1");

        assert_eq!(pos.full_move_counter, 1);

        let mv = Move {
            from: parse_square("e2"),
            to: parse_square("e4"),
            captured_piece: None,
            promotion_piece: None,
            castling_rights: pos.castling_rights,
            half_move_clock: pos.half_move_clock,
            is_en_passant: false,
        };

        pos.do_move(&mv);

        assert_eq!(pos.full_move_counter, 1);

        let mv = Move {
            from: parse_square("e7"),
            to: parse_square("e5"),
            captured_piece: None,
            promotion_piece: None,
            castling_rights: pos.castling_rights,
            half_move_clock: pos.half_move_clock,
            is_en_passant: false,
        };

        pos.do_move(&mv);

        assert_eq!(pos.full_move_counter, 2);
    }

    #[test]
    fn decrement_the_full_move_counter_when_undoing_black_moves() {
        let mut pos = parse_fen("8/8/8/4p3/4P3/8/8/8 w - - 0 2");

        assert_eq!(pos.full_move_counter, 2);

        let mv = Move {
            from: parse_square("e7"),
            to: parse_square("e5"),
            captured_piece: None,
            promotion_piece: None,
            castling_rights: CastlingRights::none(),
            half_move_clock: 0,
            is_en_passant: false,
        };

        pos.undo_move(&mv);

        assert_eq!(pos.full_move_counter, 1);

        let mv = Move {
            from: parse_square("e2"),
            to: parse_square("e4"),
            captured_piece: None,
            promotion_piece: None,
            castling_rights: CastlingRights::none(),
            half_move_clock: 0,
            is_en_passant: false,
        };

        pos.undo_move(&mv);

        assert_eq!(pos.full_move_counter, 1);
    }

    fn parse_fen(str: &str) -> Position {
        let pos = str.parse();
        assert!(pos.is_ok());

        pos.unwrap()
    }

    fn parse_square(str: &str) -> Square {
        let square = str.parse();
        assert!(square.is_ok());

        square.unwrap()
    }
}
