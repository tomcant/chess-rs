mod r#move;

use crate::board::{BitBoard, Board, Colour, Piece, PieceType, Square};
use crate::game_state::GameState;
use lazy_static::lazy_static;
use r#move::Move;

trait MoveGenerator {
    fn generate_moves(&self) -> Vec<Move>;
}

const FILE_A: BitBoard = 0x0101_0101_0101_0101;
const FILE_B: BitBoard = 0x0202_0202_0202_0202;
const FILE_G: BitBoard = 0x4040_4040_4040_4040;
const FILE_H: BitBoard = 0x8080_8080_8080_8080;

lazy_static! {
    static ref PAWN_ATTACKS: [[BitBoard; 64]; 2] = {
        let mut attacks = [[0; 64]; 2];

        for square_index in 0..64 {
            let square_u64 = 1 << square_index;

            attacks[Colour::White as usize][square_index as usize] =
                  (square_u64 & !FILE_A) << 7 | (square_u64 & !FILE_H) << 9;

            attacks[Colour::Black as usize][square_index as usize] =
                  (square_u64 & !FILE_H) >> 7 | (square_u64 & !FILE_A) >> 9;
        }

        attacks
    };

    static ref KNIGHT_ATTACKS: [BitBoard; 64] = {
        let mut attacks = [0; 64];

        for square_index in 0..64 {
            let square_u64 = 1 << square_index;

            attacks[square_index as usize] =
                  (square_u64 & !FILE_A & !FILE_B) << 6  // up 1, left 2
                | (square_u64 & !FILE_G & !FILE_H) << 10 // up 1, right 2
                | (square_u64 & !FILE_A) << 15           // up 2, left 1
                | (square_u64 & !FILE_H) << 17           // up 2, right 1

                | (square_u64 & !FILE_G & !FILE_H) >> 6  // down 1, right 2
                | (square_u64 & !FILE_A & !FILE_B) >> 10 // down 1, left 2
                | (square_u64 & !FILE_H) >> 15           // down 2, right 1
                | (square_u64 & !FILE_A) >> 17;          // down 2, left 1
        }

        attacks
    };

    static ref BISHOP_RAYS: [[BitBoard; 4]; 64] = {
        fn up_right_ray_from(square: Square) -> BitBoard {
            let mut ray = 0;
            let mut file = square.file() as i8 + 1;
            let mut rank = square.rank() as i8 + 1;

            while file < 8 && rank < 8 {
                ray += Square::from_file_and_rank(file as u8, rank as u8).u64();
                file += 1;
                rank += 1;
            }

            ray
        }

        fn up_left_ray_from(square: Square) -> BitBoard {
            let mut ray = 0;
            let mut file = square.file() as i8 - 1;
            let mut rank = square.rank() as i8 + 1;

            while file >= 0 && rank < 8 {
                ray += Square::from_file_and_rank(file as u8, rank as u8).u64();
                file -= 1;
                rank += 1;
            }

            ray
        }

        fn down_right_ray_from(square: Square) -> BitBoard {
            let mut ray = 0;
            let mut file = square.file() as i8 + 1;
            let mut rank = square.rank() as i8 - 1;

            while file < 8 && rank >= 0 {
                ray += Square::from_file_and_rank(file as u8, rank as u8).u64();
                file += 1;
                rank -= 1;
            }

            ray
        }

        fn down_left_ray_from(square: Square) -> BitBoard {
            let mut ray = 0;
            let mut file = square.file() as i8 - 1;
            let mut rank = square.rank() as i8 - 1;

            while file >= 0 && rank >= 0 {
                ray += Square::from_file_and_rank(file as u8, rank as u8).u64();
                file -= 1;
                rank -= 1;
            }

            ray
        }

        let mut rays = [[0; 4]; 64];

        for square in Square::iter() {
            rays[square.index() as usize] = [
                up_left_ray_from(*square),
                up_right_ray_from(*square),
                down_left_ray_from(*square),
                down_right_ray_from(*square),
            ];
        }

        rays
    };

    static ref BISHOP_ATTACKS: [BitBoard; 64] = {
        let mut attacks = [0; 64];

        for square in Square::iter() {
            let square_index = square.index() as usize;
            attacks[square_index] = BISHOP_RAYS[square_index].iter().sum();
        }

        attacks
    };

    static ref KING_ATTACKS: [BitBoard; 64] = {
        let mut attacks = [0; 64];

        for square_index in 0..64 {
            let square_u64 = 1 << square_index;

            attacks[square_index as usize] =
                  (square_u64 & !FILE_H) << 1
                | (square_u64 & !FILE_A) >> 1

                | square_u64 << 8
                | (square_u64 & !FILE_A) << 7
                | (square_u64 & !FILE_H) << 9

                | square_u64 >> 8
                | (square_u64 & !FILE_H) >> 7
                | (square_u64 & !FILE_A) >> 9;
        }

        attacks
    };
}

fn get_attacks(square: Square, board: &Board) -> BitBoard {
    0
}

fn get_attackers(square: Square, colour: Colour, board: &Board) -> BitBoard {
    (board.get_pieces(Piece::make(PieceType::Pawn, colour)) & get_pawn_attacks(square, colour.flip()))
        | (board.get_pieces(Piece::make(PieceType::Knight, colour)) & get_knight_attacks(square))
        | (board.get_pieces(Piece::make(PieceType::Bishop, colour)) & get_bishop_attacks(square, &board))
        | (board.get_pieces(Piece::make(PieceType::Rook, colour)) & get_rook_attacks(square, &board))
        | (board.get_pieces(Piece::make(PieceType::King, colour)) & get_king_attacks(square))
}

fn get_pawn_attacks(square: Square, colour: Colour) -> BitBoard {
    PAWN_ATTACKS[colour as usize][square.index() as usize]
}

fn get_knight_attacks(square: Square) -> BitBoard {
    KNIGHT_ATTACKS[square.index() as usize]
}

fn get_bishop_attacks(square: Square, board: &Board) -> BitBoard {
    let b = board.occupancy() & BISHOP_RAYS[square.index() as usize][0];

    // println!(
    //     "b = {b:?} occ = {} ray = {}",
    //     board.occupancy(),
    //     BISHOP_RAYS[square.index() as usize][0]
    // );

    BISHOP_ATTACKS[square.index() as usize]
}

fn get_rook_attacks(square: Square, board: &Board) -> BitBoard {
    0
}

fn get_king_attacks(square: Square) -> BitBoard {
    KING_ATTACKS[square.index() as usize]
}

impl MoveGenerator for GameState {
    fn generate_moves(&self) -> Vec<Move> {
        // let king_square = self.board.get_king_square(self.colour_to_move);
        // let squares_giving_check = get_attackers(king_square, self.colour_to_move.flip(), &self.board);
        // let num_checkers = squares_giving_check.count_ones();

        // if num_checkers > 1 {
        //     // todo: generate all king moves
        //     return vec![Move {
        //         from: king_square,
        //         to: Square::from_index(0),
        //         captured: None,
        //         promoted: None,
        //     }];
        // }

        // if num_checkers == 1 {
        //     // todo: restrict moves to capturing the piece giving check or blocking the check if it's a sliding piece
        // }

        let mut moves = vec![];

        for piece in Piece::iter_colour(self.colour_to_move) {
            println!("piece = {piece:?}");
            let mut pieces_u64 = self.board.get_pieces(*piece);

            while pieces_u64 > 0 {
                let from_square = Square::from_index(pieces_u64.trailing_zeros() as u8);
                pieces_u64 ^= from_square.u64();

                let mut attacks_u64 = match piece.get_type() {
                    PieceType::Pawn => get_pawn_attacks(from_square, self.colour_to_move),
                    PieceType::Knight => get_knight_attacks(from_square),
                    PieceType::King => get_king_attacks(from_square),
                    _ => 0,
                };

                // remove attacks on friendly squares
                // attacks_u64 &= !self.board.colours[self.colour_to_move];

                println!("from_square = {from_square:?} attacks_u64 = {attacks_u64}");

                while attacks_u64 > 0 {
                    let to_square = Square::from_index(attacks_u64.trailing_zeros() as u8);
                    attacks_u64 ^= to_square.u64();

                    let captured = self.board.get_piece_at(to_square);

                    println!("to_square = {to_square:?} captured = {captured:?}");

                    moves.push(Move {
                        from: from_square,
                        to: to_square,
                        captured,
                        promoted: None,
                    });
                }
            }
        }

        moves
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        board::{BitBoard, Board, Square},
        game_state::GameState,
        move_generator::{get_attacks, MoveGenerator},
    };

    #[test]
    fn test_generate_white_pawn_moves() {
        let fen = "8/8/8/8/8/8/4P3/8 w - - 0 1";
        let state: GameState = fen.parse().unwrap();

        assert_eq!(state.generate_moves().len(), 2);
    }

    #[test]
    fn test_generate_knight_moves() {
        let fen = "8/8/8/8/3N4/8/8/8 w - - 0 1";
        let state: GameState = fen.parse().unwrap();

        assert_eq!(state.generate_moves().len(), 8);
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

    #[test]
    fn test_white_pawn_attacks() {
        let fen = "8/8/8/8/8/8/4P3/8 w - - 0 1";
        let state: GameState = fen.parse().unwrap();

        assert_attack_squares_eq(&state.board, "e2", &["d3", "f3"]);
    }

    #[test]
    fn test_black_pawn_attacks() {
        let fen = "8/4p3/8/8/8/8/8/8 w - - 0 1";
        let state: GameState = fen.parse().unwrap();

        assert_attack_squares_eq(&state.board, "e7", &["d6", "f6"]);
    }

    #[test]
    fn test_knight_attacks() {
        let fen = "8/8/8/8/3N4/8/8/8 w - - 0 1";
        let state: GameState = fen.parse().unwrap();

        assert_attack_squares_eq(&state.board, "d4", &["c2", "e2", "b3", "f3", "b5", "f5", "c6", "e6"]);
    }

    #[test]
    fn test_bishop_on_empty_board_attacks() {
        let fen = "8/8/8/8/3b4/8/8/8 w - - 0 1";
        let state: GameState = fen.parse().unwrap();

        assert_attack_squares_eq(
            &state.board,
            "d4",
            &[
                "a1", "g1", "b2", "f2", "c3", "e3", "c5", "e5", "b6", "f6", "a7", "g7", "h8",
            ],
        );
    }

    #[test]
    fn test_bishop_on_occupied_board_attacks() {
        let fen = "r7/8/2n5/8/8/8/6B1/8 w - - 0 1";
        let state: GameState = fen.parse().unwrap();

        let attacker = "g2".parse::<Square>().unwrap();

        let mut attacks = 0;
        for sq in ["f3", "e4", "d5", "c6"] {
            attacks += sq.parse::<Square>().unwrap().u64();
        }
        println!("attacks = {attacks:?}");

        assert_eq!(get_attacks(attacker, &state.board), attacks);

        // assert_attack_squares_eq(&state.board, "g2", &["f3", "e4", "d5", "c6"]);
    }

    #[test]
    fn test_king_attacks() {
        let fen = "8/8/8/8/8/8/8/4K3 w - - 0 1";
        let state: GameState = fen.parse().unwrap();

        assert_attack_squares_eq(&state.board, "e1", &["d1", "f1", "d2", "e2", "f2"]);
    }

    fn assert_attack_squares_eq(board: &Board, attacker: &str, squares: &[&str]) {
        let attacks = squares
            .iter()
            .map(|square| square.parse::<Square>().unwrap().u64())
            .sum::<BitBoard>();

        assert_eq!(attacks, get_attacks(attacker.parse().unwrap(), board));

        // for square in Square::iter() {
        //     assert_eq!(
        //         attacker.u64() & get_attacks(*square, colour, board),
        //         if squares.contains(square) { attacker.u64() } else { 0 }
        //     );
        // }
    }
}
