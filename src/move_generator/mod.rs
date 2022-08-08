mod attacks;
mod r#move;

use crate::board::{BitBoard, Board, Colour, Piece, PieceType, Square};
use crate::game_state::GameState;
use crate::move_generator::attacks::{
    get_attackers, get_attacks, get_king_attacks, get_knight_attacks, get_pawn_attacks,
};
use r#move::Move;

trait MoveGenerator {
    fn generate_moves(&self) -> Vec<Move>;
}

impl MoveGenerator for GameState {
    fn generate_moves(&self) -> Vec<Move> {
        let king_square = self.board.get_king_square(self.colour_to_move);
        let squares_giving_check = get_attackers(king_square, self.colour_to_move.flip(), &self.board);
        let num_checkers = squares_giving_check.count_ones();

        if num_checkers > 1 {
            // todo: generate all king moves
            return vec![Move {
                from: king_square,
                to: Square::from_index(0),
                captured: None,
                promoted: None,
            }];
        }

        if num_checkers == 1 {
            // todo: restrict moves to capturing the piece giving check or blocking the check if it's a sliding piece
        }

        let mut moves = vec![];

        for piece in Piece::iter_colour(self.colour_to_move) {
            println!("piece = {piece:?}");
            let mut pieces = self.board.get_pieces(*piece);

            while pieces > 0 {
                let from_square = Square::from_index(pieces.trailing_zeros() as u8);
                pieces ^= from_square.u64();

                let mut attacks = get_attacks(from_square, &self.board);
                // & !self.board.colours[self.colour_to_move];

                println!("from_square = {from_square:?} attacks = {attacks}");

                while attacks > 0 {
                    let to_square = Square::from_index(attacks.trailing_zeros() as u8);
                    attacks ^= to_square.u64();

                    let captured = self.board.get_piece_at(to_square);

                    println!("to_square = {to_square:?} captured = {captured:?}");

                    moves.push(Move {
                        from: from_square,
                        to: to_square,
                        captured,
                        promoted: None, // todo: generate promotions
                    });
                }

                // if piece.get_type() == PieceType::Pawn {
                //     let advances = get_pawn_advances(from_square, self.colour_to_move, &self.board);

                //     if advances > 0 {
                //         let to_square = from_square.up_for_colour(self.colour_to_move);

                //         moves.push(Move {
                //             from: from_square,
                //             to: to_square,
                //             captured: None,
                //             promoted: None, // todo
                //         });

                //         if advances ^ to_square.u64() > 0 {
                //             moves.push(Move {
                //                 from: from_square,
                //                 to: to_square.up_for_colour(self.colour_to_move),
                //                 captured: None,
                //                 promoted: None,
                //             });
                //         }
                //     }
                // }
            }
        }

        moves
    }
}

fn get_pawn_advances(square: Square, colour: Colour, board: &Board) -> BitBoard {
    let mut advances = 0;
    let up_square = square.up_for_colour(colour);

    if !board.has_piece_at(up_square) {
        advances += up_square.u64();
    }

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
    use crate::{game_state::GameState, move_generator::MoveGenerator};

    #[test]
    fn test_generate_white_pawn_moves() {
        let fen = "8/8/8/8/8/8/4P3/4K3 w - - 0 1";
        let state: GameState = fen.parse().unwrap();

        // todo: wrong 2 pawn moves are included, should be single/double advance, not the diagonal attacks
        assert_eq!(state.generate_moves().len(), 7);
    }

    #[test]
    fn test_generate_knight_moves() {
        let fen = "8/8/8/8/3N4/8/8/4K3 w - - 0 1";
        let state: GameState = fen.parse().unwrap();

        assert_eq!(state.generate_moves().len(), 13);
    }

    #[test]
    fn test_generate_king_moves() {
        let fen = "8/8/8/8/8/8/8/4K3 w - - 0 1";
        let state: GameState = fen.parse().unwrap();

        assert_eq!(state.generate_moves().len(), 5);
    }

    #[test]
    fn test_check_by_knight() {
        let fen = "4k3/8/8/8/8/3n4/2n5/4K3 w - - 0 1";
        let state: GameState = fen.parse().unwrap();

        assert_eq!(state.generate_moves().len(), 1);
    }
}
