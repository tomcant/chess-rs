use crate::colour::Colour;
use crate::movegen::Move;
use crate::piece::Piece;
use crate::square::Square;
use smallvec::SmallVec;

mod board;
mod castling;
mod fen;
mod zobrist;

pub use board::Board;
pub use castling::{CastlingRight, CastlingRights};
pub use fen::START_POS_FEN;

const MAX_HISTORY: usize = 256;

#[derive(Debug, Clone)]
pub struct Position {
    pub board: Board,
    pub colour_to_move: Colour,
    pub castling_rights: CastlingRights,
    pub en_passant_square: Option<Square>,
    pub half_move_clock: u8,
    pub full_move_counter: u8,
    pub key_history: SmallVec<[u64; MAX_HISTORY]>,
}

impl Position {
    pub fn new(
        board: Board,
        colour_to_move: Colour,
        castling_rights: CastlingRights,
        en_passant_square: Option<Square>,
        half_move_clock: u8,
        full_move_counter: u8,
    ) -> Self {
        Self {
            board,
            colour_to_move,
            castling_rights,
            en_passant_square,
            half_move_clock,
            full_move_counter,
            key_history: SmallVec::new(),
        }
    }

    pub fn startpos() -> Self {
        START_POS_FEN.parse().unwrap()
    }

    pub fn do_move(&mut self, mv: &Move) {
        self.key_history.push(self.key());
        self.half_move_clock += 1;
        self.en_passant_square = None;

        if let Some(capture_square) = mv.capture_square() {
            self.board.remove_piece(capture_square);
            self.half_move_clock = 0;
        }

        let piece = mv.promotion_piece.unwrap_or(mv.piece);

        if piece.is_pawn() {
            self.half_move_clock = 0;

            if mv.rank_diff() == 2 {
                self.en_passant_square = Some(mv.from.advance(self.colour_to_move));
            }
        }

        if piece.is_king() {
            self.castling_rights.remove_for_colour(self.colour_to_move);

            if mv.is_castling() {
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
        if mv.is_castling() {
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

        let piece = match mv.promotion_piece {
            Some(piece) => Piece::pawn(piece.colour()),
            None => mv.piece,
        };
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

        self.key_history.pop();
    }

    pub fn is_threefold_repetition(&self) -> bool {
        if self.half_move_clock < 8 {
            return false;
        }

        let current_key = self.key();
        let len = self.key_history.len();
        let max_back = len.min(self.half_move_clock as usize);
        let mut found_first_repetition = false;
        let mut offset = 2;

        while offset <= max_back {
            if self.key_history[len - offset] == current_key {
                if found_first_repetition {
                    return true;
                }
                found_first_repetition = true;
            }
            offset += 2;
        }

        false
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
            piece: Piece::WR,
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
            piece: Piece::WR,
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
            piece: Piece::WN,
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
            piece: Piece::WN,
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
            piece: Piece::WK,
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
            piece: Piece::WK,
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
            piece: Piece::WR,
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
            piece: Piece::BB,
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
            piece: Piece::WP,
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
            piece: Piece::WP,
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
            piece: Piece::WP,
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
            piece: Piece::WP,
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
            piece: Piece::WP,
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
            piece: Piece::WP,
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
            piece: Piece::BP,
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
            piece: Piece::WP,
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
        let mut pos = parse_fen("8/4p3/8/8/8/8/4P3/4K3 w - - 0 1");

        let mv = Move {
            piece: Piece::WK,
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
            piece: Piece::WP,
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
            piece: Piece::WQ,
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
            piece: Piece::WP,
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
            piece: Piece::BP,
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
            piece: Piece::BP,
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
            piece: Piece::WP,
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

    #[test]
    fn detect_threefold_repetition() {
        let positions = [
            Position::startpos(),
            parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 60 1"),
        ];

        let moves = [
            (Piece::WN, "g1", "f3"), // Nf3
            (Piece::BN, "g8", "f6"), // Nf6
            (Piece::WN, "f3", "g1"), // Ng1
            (Piece::BN, "f6", "g8"), // Ng8, first repetition
            (Piece::WN, "g1", "f3"), // Nf3
            (Piece::BN, "g8", "f6"), // Nf6
            (Piece::WN, "f3", "g1"), // Ng1
            (Piece::BN, "f6", "g8"), // Ng8, second repetition, threefold
        ];

        for mut pos in positions {
            for (index, (piece, from, to)) in moves.iter().enumerate() {
                let mv = Move {
                    piece: *piece,
                    from: parse_square(*from),
                    to: parse_square(*to),
                    captured_piece: None,
                    promotion_piece: None,
                    castling_rights: pos.castling_rights,
                    half_move_clock: pos.half_move_clock,
                    is_en_passant: false,
                };

                pos.do_move(&mv);

                let expected_threefold_repetition = index == 7;
                assert_eq!(
                    pos.is_threefold_repetition(),
                    expected_threefold_repetition,
                    "Position should {} a threefold repetition at ply {}",
                    if expected_threefold_repetition { "be" } else { "not be" },
                    index + 1
                );
            }
        }
    }

    #[test]
    fn threefold_repetition_not_counted_when_castling_rights_differ() {
        let mut pos = Position::startpos();

        let moves = [
            (Piece::WN, "g1", "f3"), // Nf3, the pieces revisit these squares later
            (Piece::BN, "g8", "f6"), // Nf6
            (Piece::WR, "h1", "g1"), // Rg1, removes white king-side castling rights
            (Piece::BN, "f6", "g8"), // Ng8
            (Piece::WR, "g1", "h1"), // Rh1, first repetition of piece placement
            (Piece::BN, "g8", "f6"), // Nf6
            (Piece::WN, "f3", "g1"), // Ng1
            (Piece::BN, "f6", "g8"), // Ng8
            (Piece::WN, "g1", "f3"), // Nf3, second repetition but doesn't count due to castling rights
        ];

        for (index, (piece, from, to)) in moves.iter().enumerate() {
            let mv = Move {
                piece: *piece,
                from: parse_square(*from),
                to: parse_square(*to),
                captured_piece: None,
                promotion_piece: None,
                castling_rights: pos.castling_rights,
                half_move_clock: pos.half_move_clock,
                is_en_passant: false,
            };

            pos.do_move(&mv);

            assert!(
                !pos.is_threefold_repetition(),
                "Position should not be considered a threefold repetition at ply {}",
                index + 1
            );
        }
    }

    #[test]
    fn threefold_repetition_not_counted_when_en_passant_availability_differs() {
        let mut pos = parse_fen("4k3/4p3/8/3P4/8/8/8/4K1Nn w - - 0 1");

        let moves = [
            (Piece::WN, "g1", "f3"), // Nf3
            (Piece::BP, "e7", "e5"), // e5, the pieces revisit these squares later
            (Piece::WN, "f3", "g1"), // Ng1, en passant availability expires
            (Piece::BN, "h1", "g3"), // Ng3
            (Piece::WN, "g1", "f3"), // Nf3
            (Piece::BN, "g3", "h1"), // Nh1, first repetition of piece placement
            (Piece::WN, "f3", "g1"), // Ng1
            (Piece::BN, "h1", "g3"), // Ng3
            (Piece::WN, "g1", "f3"), // Nf3
            (Piece::BN, "g3", "h1"), // Nh1, second repetition but doesn't count due to en passant availability
        ];

        for (index, (piece, from, to)) in moves.iter().enumerate() {
            let mv = Move {
                piece: *piece,
                from: parse_square(*from),
                to: parse_square(*to),
                captured_piece: None,
                promotion_piece: None,
                castling_rights: pos.castling_rights,
                half_move_clock: pos.half_move_clock,
                is_en_passant: false,
            };

            pos.do_move(&mv);

            assert!(
                !pos.is_threefold_repetition(),
                "Position should not be considered a threefold repetition at ply {}",
                index + 1
            );
        }
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
