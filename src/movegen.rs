use crate::attacks::{get_attacks, is_attacked};
use crate::board::{BitBoard, Board};
use crate::castling::{CastlingRight, CastlingRights};
use crate::colour::Colour;
use crate::piece::{Piece, PieceType};
use crate::position::Position;
use crate::r#move::Move;
use crate::square::Square;

const MAX_MOVES: usize = 256;

pub fn generate_all_moves(pos: &Position) -> Vec<Move> {
    let mut moves = Vec::with_capacity(MAX_MOVES);
    let colour_to_move = pos.colour_to_move;

    for piece_type in PieceType::types() {
        let mut pieces = pos.board.pieces(*piece_type, colour_to_move);

        while pieces > 0 {
            let from_square = Square::from_index(pieces.trailing_zeros() as u8);
            pieces ^= from_square.u64();

            let mut to_squares = !pos.board.pieces_by_colour(colour_to_move)
                & get_attacks(Piece::from(*piece_type, colour_to_move), from_square, &pos.board);

            if piece_type.is_pawn() {
                to_squares |= get_pawn_advances(from_square, colour_to_move, &pos.board);

                if can_capture_en_passant(from_square, pos.en_passant_square, colour_to_move) {
                    moves.push(Move {
                        from: from_square,
                        to: pos.en_passant_square.unwrap(),
                        captured_piece: Some(Piece::from(PieceType::Pawn, pos.opponent_colour())),
                        promotion_piece: None,
                        castling_rights: pos.castling_rights,
                        is_en_passant: true,
                    });
                }
            } else if piece_type.is_king() {
                to_squares |= get_castling(pos.castling_rights, colour_to_move, &pos.board);
            }

            while to_squares > 0 {
                let to_square = Square::from_index(to_squares.trailing_zeros() as u8);
                to_squares ^= to_square.u64();

                let captured_piece = pos.board.piece_at(to_square);

                if piece_type.is_pawn() && to_square.is_back_rank() {
                    for piece in Piece::promotions(colour_to_move) {
                        moves.push(Move {
                            from: from_square,
                            to: to_square,
                            captured_piece,
                            promotion_piece: Some(*piece),
                            castling_rights: pos.castling_rights,
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
                    castling_rights: pos.castling_rights,
                    is_en_passant: false,
                });
            }
        }
    }

    moves
}

pub fn generate_capture_moves(pos: &Position) -> Vec<Move> {
    let mut moves = Vec::with_capacity(MAX_MOVES);
    let colour_to_move = pos.colour_to_move;

    for piece_type in PieceType::types() {
        let mut pieces = pos.board.pieces(*piece_type, colour_to_move);

        while pieces > 0 {
            let from_square = Square::from_index(pieces.trailing_zeros() as u8);
            pieces ^= from_square.u64();

            if piece_type.is_pawn() && can_capture_en_passant(from_square, pos.en_passant_square, colour_to_move) {
                moves.push(Move {
                    from: from_square,
                    to: pos.en_passant_square.unwrap(),
                    captured_piece: Some(Piece::from(PieceType::Pawn, pos.opponent_colour())),
                    promotion_piece: None,
                    castling_rights: pos.castling_rights,
                    is_en_passant: true,
                });
            }

            let mut to_squares = pos.board.pieces_by_colour(pos.opponent_colour())
                & get_attacks(Piece::from(*piece_type, colour_to_move), from_square, &pos.board);

            while to_squares > 0 {
                let to_square = Square::from_index(to_squares.trailing_zeros() as u8);
                to_squares ^= to_square.u64();

                let captured_piece = pos.board.piece_at(to_square);

                if piece_type.is_pawn() && to_square.is_back_rank() {
                    for piece in Piece::promotions(colour_to_move) {
                        moves.push(Move {
                            from: from_square,
                            to: to_square,
                            captured_piece,
                            promotion_piece: Some(*piece),
                            castling_rights: pos.castling_rights,
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
                    castling_rights: pos.castling_rights,
                    is_en_passant: false,
                });
            }
        }
    }

    moves
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
        return pawn_square.file_diff(square) == 1
            && pawn_square.rank() == square.advance(colour_to_move.flip()).rank();
    }

    false
}

fn get_castling(castling_rights: CastlingRights, colour_to_move: Colour, board: &Board) -> BitBoard {
    let mut castling = 0;

    if colour_to_move == Colour::White {
        if castling_rights.has(CastlingRight::WhiteKing)
            && !board.has_piece_at(Square::from_index(5))
            && !board.has_piece_at(Square::from_index(6))
            && !is_attacked(Square::from_index(4), colour_to_move.flip(), board)
            && !is_attacked(Square::from_index(5), colour_to_move.flip(), board)
        {
            castling |= Square::from_index(6).u64();
        }

        if castling_rights.has(CastlingRight::WhiteQueen)
            && !board.has_piece_at(Square::from_index(1))
            && !board.has_piece_at(Square::from_index(2))
            && !board.has_piece_at(Square::from_index(3))
            && !is_attacked(Square::from_index(3), colour_to_move.flip(), board)
            && !is_attacked(Square::from_index(4), colour_to_move.flip(), board)
        {
            castling |= Square::from_index(2).u64();
        }
    } else {
        if castling_rights.has(CastlingRight::BlackKing)
            && !board.has_piece_at(Square::from_index(61))
            && !board.has_piece_at(Square::from_index(62))
            && !is_attacked(Square::from_index(60), colour_to_move.flip(), board)
            && !is_attacked(Square::from_index(61), colour_to_move.flip(), board)
        {
            castling |= Square::from_index(62).u64();
        }

        if castling_rights.has(CastlingRight::BlackQueen)
            && !board.has_piece_at(Square::from_index(57))
            && !board.has_piece_at(Square::from_index(58))
            && !board.has_piece_at(Square::from_index(59))
            && !is_attacked(Square::from_index(59), colour_to_move.flip(), board)
            && !is_attacked(Square::from_index(60), colour_to_move.flip(), board)
        {
            castling |= Square::from_index(58).u64();
        }
    }

    castling
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::attacks::is_in_check;

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
    fn castle_king_side_only() {
        let pos = parse_fen("8/8/8/8/8/8/8/R3K2R w K - 0 1");

        let moves = generate_all_moves(&pos);

        assert_castling_move_count(&moves, &pos.board, 1);
    }

    #[test]
    fn castle_queen_side_only() {
        let pos = parse_fen("8/8/8/8/8/8/8/R3K2R w Q - 0 1");

        let moves = generate_all_moves(&pos);

        assert_castling_move_count(&moves, &pos.board, 1);
    }

    #[test]
    fn castle_king_and_queen_side() {
        let pos = parse_fen("8/8/8/8/8/8/8/R3K2R w KQ - 0 1");

        let moves = generate_all_moves(&pos);

        assert_castling_move_count(&moves, &pos.board, 2);
    }

    #[test]
    fn no_castling_when_the_target_square_is_occupied_by_a_friendly_piece() {
        let pos = parse_fen("8/8/8/8/8/8/8/R1B1K1NR w KQ - 0 1");

        let moves = generate_all_moves(&pos);

        assert_castling_move_count(&moves, &pos.board, 0);
    }

    #[test]
    fn no_castling_when_the_target_square_is_occupied_by_an_opponent_piece() {
        let pos = parse_fen("8/8/8/8/8/8/8/R1b1K1nR w KQ - 0 1");

        let moves = generate_all_moves(&pos);

        assert_castling_move_count(&moves, &pos.board, 0);
    }

    #[test]
    fn no_castling_when_a_piece_blocks_the_path() {
        let pos = parse_fen("8/8/8/8/8/8/8/RN2KB1R w KQ - 0 1");

        let moves = generate_all_moves(&pos);

        assert_castling_move_count(&moves, &pos.board, 0);
    }

    #[test]
    fn no_castling_when_the_king_path_is_attacked() {
        let pos = parse_fen("8/8/8/8/8/4n3/8/R3K2R w KQ - 0 1");

        let moves = generate_all_moves(&pos);

        assert_castling_move_count(&moves, &pos.board, 0);
    }

    #[test]
    fn no_castling_when_the_right_was_previously_lost() {
        let pos = parse_fen("8/8/8/8/8/8/8/R3K2R w Q - 0 1");

        let moves = generate_all_moves(&pos);

        assert_castling_move_count(&moves, &pos.board, 1);

        let castling_move = moves
            .iter()
            .filter(|mv| pos.board.piece_at(mv.from).unwrap().is_king() && mv.file_diff() > 1)
            .next()
            .unwrap();

        assert_eq!(castling_move.to, "c1".parse::<Square>().unwrap());
    }

    #[test]
    fn no_castling_out_of_check() {
        let pos = parse_fen("8/8/8/8/8/3n4/8/R3K2R w KQ - 0 1");

        let moves = generate_all_moves(&pos);

        assert_castling_move_count(&moves, &pos.board, 0);
    }

    #[test]
    fn en_passant_capture() {
        let pos = parse_fen("8/8/8/3PpP2/8/8/8/8 w - e6 0 1");

        let moves = generate_all_moves(&pos);

        assert_eq!(moves.iter().filter(|mv| mv.is_en_passant).count(), 2);
    }

    #[test]
    fn ignore_friendly_piece_captures() {
        assert_pseudo_legal_move_count("8/8/5p2/5P2/3N4/8/8/8 w - - 0 1", 7);
    }

    mod perft {
        use super::*;
        use crate::fen::START_POS_FEN;

        #[test]
        fn perft_start_position_shallow() {
            assert_perft_for_fen(START_POS_FEN, 4, 197_281);
        }

        #[test]
        #[ignore]
        fn perft_start_position() {
            assert_perft_for_fen(START_POS_FEN, 6, 119_060_324);
        }

        #[test]
        #[ignore]
        fn perft_position_2() {
            assert_perft_for_fen(
                "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
                5,
                193_690_690,
            );
        }

        #[test]
        #[ignore]
        fn perft_position_3() {
            assert_perft_for_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1", 7, 178_633_661);
        }

        #[test]
        #[ignore]
        fn perft_position_4() {
            assert_perft_for_fen(
                "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
                6,
                706_045_033,
            );
        }

        #[test]
        #[ignore]
        fn perft_position_4_flipped() {
            assert_perft_for_fen(
                "r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ - 0 1",
                6,
                706_045_033,
            );
        }

        #[test]
        #[ignore]
        fn perft_position_5() {
            assert_perft_for_fen(
                "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
                5,
                89_941_194,
            );
        }

        #[test]
        #[ignore]
        fn perft_position_6() {
            assert_perft_for_fen(
                "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
                5,
                164_075_551,
            );
        }

        fn assert_perft_for_fen(fen: &str, depth: u8, expected_move_count: u64) {
            assert_eq!(perft(&mut parse_fen(fen), depth, true), expected_move_count);
        }

        fn perft(pos: &mut Position, depth: u8, divide: bool) -> u64 {
            if depth == 0 {
                return 1;
            }

            let mut nodes = 0;

            for mv in generate_all_moves(&pos) {
                pos.do_move(&mv);

                if !is_in_check(pos.opponent_colour(), &pos.board) {
                    let nodes_divide = perft(pos, depth - 1, false);

                    if divide {
                        println!("{mv}: {nodes_divide}");
                    }

                    nodes += nodes_divide;
                }

                pos.undo_move(&mv);
            }

            if divide {
                println!("Nodes: {nodes}");
            }

            nodes
        }
    }

    fn assert_pseudo_legal_move_count(fen: &str, count: usize) {
        assert_eq!(generate_all_moves(&parse_fen(fen)).len(), count);
    }

    fn assert_legal_move_count(fen: &str, count: usize) {
        let mut pos = parse_fen(fen);
        let mut legal_move_count = 0;

        for mv in generate_all_moves(&pos) {
            pos.do_move(&mv);

            if !is_in_check(pos.opponent_colour(), &pos.board) {
                legal_move_count += 1;
            }

            pos.undo_move(&mv);
        }

        assert_eq!(legal_move_count, count);
    }

    fn assert_castling_move_count(moves: &Vec<Move>, board: &Board, count: usize) {
        assert_eq!(
            moves
                .iter()
                .filter(|mv| board.piece_at(mv.from).unwrap().is_king() && mv.file_diff() > 1)
                .count(),
            count
        );
    }

    fn parse_fen(str: &str) -> Position {
        let pos = str.parse();
        assert!(pos.is_ok());

        pos.unwrap()
    }
}
