mod attacks;
mod r#move;

use crate::board::{BitBoard, Board, Colour, Piece, PieceType, Square};
use crate::game_state::GameState;
use crate::move_generator::attacks::{get_attackers, get_attacks};

pub use r#move::Move;

trait MoveGenerator {
    fn generate_moves(&self) -> Vec<Move>;
}

impl MoveGenerator for GameState {
    fn generate_moves(&self) -> Vec<Move> {
        let king_square = self.board.get_king_square(self.colour_to_move);
        let checking_squares = get_attackers(king_square, self.colour_to_move.flip(), &self.board);
        let num_checkers = checking_squares.count_ones();

        if num_checkers > 1 {
            // todo: generate all king moves
            return vec![Move {
                from: king_square,
                to: Square::from_index(0),
                captured: None,
                promoted: None,
            }];
        }

        let mut capture_mask = 0xFFFFFFFFFFFFFFFFu64;
        let mut non_capture_mask = 0xFFFFFFFFFFFFFFFFu64;

        if num_checkers == 1 {
            // todo: restrict moves to capturing the piece giving check or blocking the check if it's a sliding piece

            capture_mask = checking_squares;

            let checker_square = Square::from_u64(checking_squares);
            let checker_piece = self.board.get_piece_at(checker_square).unwrap();

            if checker_piece.is_sliding() {
                // todo: restrict normal moves to squares between checking piece and king
                // non_capture_mask = ...;
            } else {
                non_capture_mask = 0; // restrict to captures
            }
        }

        let mut moves = vec![];

        for piece in Piece::iter_colour(self.colour_to_move) {
            let mut pieces = self.board.get_pieces(*piece);

            while pieces > 0 {
                let from_square = Square::from_index(pieces.trailing_zeros() as u8);
                pieces ^= from_square.u64();

                let mut attacks =
                    get_attacks(from_square, &self.board) & !self.board.get_pieces_by_colour(self.colour_to_move);

                if piece.get_type() == PieceType::Pawn {
                    attacks |= get_pawn_advances(from_square, self.colour_to_move, &self.board);
                }

                while attacks > 0 {
                    let to_square = Square::from_index(attacks.trailing_zeros() as u8);
                    attacks ^= to_square.u64();

                    let captured = self.board.get_piece_at(to_square);

                    if captured.is_some() && capture_mask & to_square.u64() == 0 {
                        continue;
                    }

                    if captured.is_none() && non_capture_mask & to_square.u64() == 0 {
                        continue;
                    }

                    moves.push(Move {
                        from: from_square,
                        to: to_square,
                        captured,
                        promoted: None, // todo: generate promotions
                    });
                }
            }
        }

        moves
    }
}

fn get_pawn_advances(square: Square, colour: Colour, board: &Board) -> BitBoard {
    let up_square = square.up_for_colour(colour);

    if board.has_piece_at(up_square) {
        return 0;
    }

    let mut advances = up_square.u64();

    let start_rank = match colour {
        Colour::White => 1,
        Colour::Black => 6,
    };

    if square.rank() == start_rank {
        let up_up_square = up_square.up_for_colour(colour);

        if !board.has_piece_at(up_up_square) {
            advances += up_up_square.u64();
        }
    }

    advances
}

#[cfg(test)]
mod tests {
    use crate::{
        game_state::GameState,
        move_generator::{r#move::Move, MoveGenerator},
    };

    mod perft {
        use crate::{game_state::GameState, move_generator::MoveGenerator};

        #[test]
        fn test_perft_starting_position_depth_3() {
            let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
            let mut state: GameState = fen.parse().unwrap();

            assert_eq!(perft(&mut state, 3), 8902);
        }

        pub fn perft(state: &mut GameState, depth: u8) -> u64 {
            let moves = state.generate_moves();

            if depth == 1 {
                return moves.len() as u64;
            }

            let mut nodes = 0;

            for mv in moves {
                state.do_move(&mv);
                nodes += perft(state, depth - 1);
                state.undo_move(&mv);
            }

            nodes
        }
    }

    #[test]
    fn test_generate_moves_from_start_position() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let state: GameState = fen.parse().unwrap();

        assert_eq!(state.generate_moves().len(), 20);
    }

    #[test]
    fn test_generate_white_pawn_moves() {
        let fen = "8/8/8/8/8/8/4P3/4K3 w - - 0 1";
        let state: GameState = fen.parse().unwrap();

        assert_eq!(state.generate_moves().len(), 6);
    }

    #[test]
    fn test_generate_black_pawn_moves() {
        let fen = "4k3/4p3/8/8/8/8/8/8 b - - 0 1";
        let state: GameState = fen.parse().unwrap();

        assert_eq!(state.generate_moves().len(), 6);
    }

    #[test]
    fn test_generate_white_pawn_advance_single() {
        let fen = "8/8/8/8/4k3/8/4P3/4K3 w - - 0 1";
        let state: GameState = fen.parse().unwrap();

        assert_eq!(state.generate_moves().len(), 5);
    }

    #[test]
    fn test_generate_white_pawn_advance_double() {
        let fen = "8/8/8/8/8/4k3/4P3/4K3 w - - 0 1";
        let state: GameState = fen.parse().unwrap();

        assert_eq!(state.generate_moves().len(), 4);
    }

    #[test]
    fn test_generate_knight_moves() {
        let fen = "8/8/8/8/3N4/8/8/4K3 w - - 0 1";
        let state: GameState = fen.parse().unwrap();

        assert_eq!(state.generate_moves().len(), 13);
    }

    #[test]
    fn test_generate_bishop_moves() {
        let fen = "8/r7/5n2/8/3B4/8/8/4K3 w - - 0 1";
        let state: GameState = fen.parse().unwrap();

        let moves = state.generate_moves();
        let captures = moves.iter().filter(|mv| mv.captured.is_some()).collect::<Vec<&Move>>();

        assert_eq!(moves.len(), 16);
        assert_eq!(captures.len(), 2);
    }

    #[test]
    fn test_generate_rook_moves() {
        let fen = "8/3b4/8/8/1n1R4/8/8/4K3 w - - 0 1";
        let state: GameState = fen.parse().unwrap();

        let moves = state.generate_moves();
        let captures = moves.iter().filter(|mv| mv.captured.is_some()).collect::<Vec<&Move>>();

        assert_eq!(moves.len(), 17);
        assert_eq!(captures.len(), 2);
    }

    #[test]
    fn test_generate_king_moves() {
        let fen = "8/8/8/8/8/8/8/4K3 w - - 0 1";
        let state: GameState = fen.parse().unwrap();

        assert_eq!(state.generate_moves().len(), 5);
    }

    #[test]
    fn test_ignore_friendly_piece_captures() {
        let fen = "8/8/5p2/5P2/3N4/8/8/4K3 w - - 0 1";
        let state: GameState = fen.parse().unwrap();

        assert_eq!(state.generate_moves().len(), 12);
    }

    #[test]
    fn test_check_by_knight() {
        let fen = "4k3/8/8/8/8/3n4/2n5/4K3 w - - 0 1";
        let state: GameState = fen.parse().unwrap();

        assert_eq!(state.generate_moves().len(), 1);
    }
}
