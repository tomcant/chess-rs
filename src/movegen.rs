use crate::attacks::get_attacks;
use crate::board::{BitBoard, Board};
use crate::colour::Colour;
use crate::game::GameState;
use crate::piece::{Piece, PieceType};
use crate::r#move::Move;
use crate::square::Square;

trait MoveGenerator {
    fn generate_moves(&self) -> Vec<Move>;
}

impl MoveGenerator for GameState {
    fn generate_moves(&self) -> Vec<Move> {
        let mut moves = vec![];

        for piece_type in PieceType::types() {
            let mut pieces = self.board.get_pieces(*piece_type, self.colour_to_move);

            while pieces > 0 {
                let from_square = Square::from_index(pieces.trailing_zeros() as u8);
                pieces ^= from_square.u64();

                let mut attacks =
                    get_attacks(from_square, &self.board) & !self.board.get_pieces_by_colour(self.colour_to_move);

                if piece_type.is_pawn() {
                    attacks |= get_pawn_advances(from_square, self.colour_to_move, &self.board);

                    if can_capture_en_passant(from_square, self.en_passant_square, self.colour_to_move) {
                        moves.push(Move {
                            from: from_square,
                            to: self.en_passant_square.unwrap(),
                            captured_piece: Some(Piece::from(PieceType::Pawn, self.colour_to_move.flip())),
                            promotion_piece: None,
                            is_en_passant: true,
                        });
                    }
                }

                while attacks > 0 {
                    let to_square = Square::from_index(attacks.trailing_zeros() as u8);
                    attacks ^= to_square.u64();

                    let captured_piece = self.board.get_piece_at(to_square);

                    // todo: generate castling

                    if piece_type.is_pawn() && to_square.is_promotion_rank() {
                        for piece in Piece::promotions(self.colour_to_move) {
                            moves.push(Move {
                                from: from_square,
                                to: to_square,
                                captured_piece,
                                promotion_piece: Some(*piece),
                                is_en_passant: false,
                            });
                        }

                        continue;
                    }

                    moves.push(Move {
                        from: from_square,
                        to: to_square,
                        captured_piece,
                        promotion_piece: None,
                        is_en_passant: false,
                    });
                }
            }
        }

        moves
    }
}

fn get_pawn_advances(square: Square, colour: Colour, board: &Board) -> BitBoard {
    let advanced_square = square.advance(colour);

    if board.has_piece_at(advanced_square) {
        return 0;
    }

    let mut advances = advanced_square.u64();

    let start_rank = match colour {
        Colour::White => 1,
        Colour::Black => 6,
    };

    if square.rank() == start_rank {
        let advanced_square = advanced_square.advance(colour);

        if !board.has_piece_at(advanced_square) {
            advances += advanced_square.u64();
        }
    }

    advances
}

fn can_capture_en_passant(pawn_square: Square, en_passant_square: Option<Square>, colour_to_move: Colour) -> bool {
    if let Some(square) = en_passant_square {
        return pawn_square.file().abs_diff(square.file()) == 1
            && pawn_square.rank() == square.advance(colour_to_move.flip()).rank();
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::attacks::is_in_check;
    use crate::game::GameState;

    #[test]
    fn legal_move_count_in_checkmate_is_zero() {
        assert_legal_move_count("rnb1kbnr/pppp1ppp/4p3/8/6Pq/5P2/PPPPP2P/RNBQKBNR w KQkq - 0 1", 0);
    }

    #[test]
    fn legal_move_count_in_check_is_limited() {
        assert_legal_move_count("rnbqkbnr/1pp1p1pp/p2p1p2/1B6/8/4P3/PPPP1PPP/RNBQK1NR b KQq - 0 1", 7);
    }

    #[test]
    fn white_pawn_moves() {
        assert_pseudo_legal_move_count("8/8/8/8/8/8/4P3/8 w - - 0 1", 2);
    }

    #[test]
    fn black_pawn_moves() {
        assert_pseudo_legal_move_count("8/4p3/8/8/8/8/8/8 b - - 0 1", 2);
    }

    #[test]
    fn single_pawn_advance() {
        assert_pseudo_legal_move_count("8/8/8/8/4p3/8/4P3/8 w - - 0 1", 1);
    }

    #[test]
    fn double_pawn_advance() {
        assert_pseudo_legal_move_count("8/8/8/8/8/4p3/4P3/8 w - - 0 1", 0);
    }

    #[test]
    fn knight_moves() {
        assert_pseudo_legal_move_count("8/8/8/8/3N4/8/8/8 w - - 0 1", 8);
    }

    #[test]
    fn bishop_moves() {
        assert_pseudo_legal_move_count("8/r7/5n2/8/3B4/8/8/8 w - - 0 1", 11);
    }

    #[test]
    fn rook_moves() {
        assert_pseudo_legal_move_count("8/3b4/8/8/1n1R4/8/8/8 w - - 0 1", 12);
    }

    #[test]
    fn king_moves() {
        assert_pseudo_legal_move_count("8/8/8/8/8/8/8/4K3 w - - 0 1", 5);
    }

    #[test]
    fn pawn_promotion_with_advance() {
        assert_pseudo_legal_move_count("8/4P3/8/8/8/8/8/8 w - - 0 1", 4);
    }

    #[test]
    fn pawn_promotion_with_capture() {
        assert_pseudo_legal_move_count("3qk3/4P3/8/8/8/8/8/8 w - - 0 1", 4);
    }

    #[test]
    fn pawn_promotion_with_advance_or_capture() {
        assert_pseudo_legal_move_count("3q4/4P3/8/8/8/8/8/8 w - - 0 1", 8);
    }

    #[test]
    fn en_passant_capture() {
        let state = parse_fen("8/8/8/3PpP2/8/8/8/8 w - e6 0 1");

        let moves = state.generate_moves();

        assert_eq!(moves.iter().filter(|mv| mv.is_en_passant).count(), 2);
    }

    #[test]
    fn ignore_friendly_piece_captures() {
        assert_pseudo_legal_move_count("8/8/5p2/5P2/3N4/8/8/8 w - - 0 1", 7);
    }

    mod perft {
        use super::*;

        #[test]
        fn perft_start_position_shallow() {
            assert_perft_for_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", 4, 197_281);
        }

        #[test]
        #[ignore]
        fn perft_start_position() {
            assert_perft_for_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", 5, 4_865_609);
        }

        #[test]
        #[ignore]
        fn perft_position_3() {
            assert_perft_for_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1", 5, 674_624);
        }

        #[test]
        #[ignore]
        fn perft_position_4() {
            assert_perft_for_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1", 1, 6);
        }

        #[test]
        #[ignore]
        fn perft_position_6() {
            assert_perft_for_fen(
                "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
                4,
                3_894_594,
            );
        }

        fn assert_perft_for_fen(fen: &str, depth: u8, expected_move_count: u64) {
            assert_eq!(perft(&mut parse_fen(fen), depth), expected_move_count);
        }

        fn perft(state: &mut GameState, depth: u8) -> u64 {
            if depth == 0 {
                return 1;
            }

            let mut nodes = 0;

            for mv in state.generate_moves() {
                state.do_move(&mv);

                if !is_in_check(&state.board, state.colour_to_move.flip()) {
                    nodes += perft(state, depth - 1);
                }

                state.undo_move(&mv);
            }

            nodes
        }
    }

    fn assert_pseudo_legal_move_count(fen: &str, count: usize) {
        assert_eq!(parse_fen(fen).generate_moves().len(), count);
    }

    fn assert_legal_move_count(fen: &str, count: usize) {
        let mut state = parse_fen(fen);
        let mut legal_move_count = 0;

        for mv in state.generate_moves() {
            state.do_move(&mv);

            if !is_in_check(&state.board, state.colour_to_move.flip()) {
                legal_move_count += 1;
            }

            state.undo_move(&mv);
        }

        assert_eq!(legal_move_count, count);
    }

    fn parse_fen(str: &str) -> GameState {
        let state = str.parse();
        assert!(state.is_ok());

        state.unwrap()
    }
}
