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

fn get_attacks(square: Square, colour: Colour, board: &Board) -> BitBoard {
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
    0
}

fn get_rook_attacks(square: Square, board: &Board) -> BitBoard {
    0
}

fn get_king_attacks(square: Square) -> BitBoard {
    KING_ATTACKS[square.index() as usize]
}

impl MoveGenerator for GameState {
    fn generate_moves(&self) -> Vec<Move> {
        let king_square = self.board.get_king_square(self.colour_to_move);
        let squares_giving_check = get_attacks(king_square, self.colour_to_move.flip(), &self.board);
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

        // todo: generate moves

        vec![]
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        board::{Board, Colour, Square},
        game_state::GameState,
        move_generator::{get_attacks, MoveGenerator},
    };

    #[test]
    fn test_check_by_knight() {
        let fen = "4k3/8/8/8/8/3n4/2n5/4K3 w - - 0 1";
        let state: GameState = fen.parse().unwrap();

        assert_eq!(state.generate_moves().len(), 1);
    }

    #[test]
    fn test_attack_by_white_pawn() {
        let fen = "8/8/8/8/8/8/4P3/8 w - - 0 1";
        let state: GameState = fen.parse().unwrap();

        assert_attacks_eq(&state.board, Colour::White, "e2", &["d3", "f3"]);
    }

    #[test]
    fn test_attack_by_black_pawn() {
        let fen = "8/4p3/8/8/8/8/8/8 w - - 0 1";
        let state: GameState = fen.parse().unwrap();

        assert_attacks_eq(&state.board, Colour::Black, "e7", &["d6", "f6"]);
    }

    #[test]
    fn test_attack_by_knight() {
        let fen = "8/8/8/8/3N4/8/8/8 w - - 0 1";
        let state: GameState = fen.parse().unwrap();

        assert_attacks_eq(
            &state.board,
            Colour::White,
            "d4",
            &["c2", "e2", "b3", "f3", "b5", "f5", "c6", "e6"],
        );
    }

    #[test]
    fn test_attack_by_king() {
        let fen = "8/8/8/8/8/8/8/4K3 w - - 0 1";
        let state: GameState = fen.parse().unwrap();

        assert_attacks_eq(&state.board, Colour::White, "e1", &["d1", "f1", "d2", "e2", "f2"]);
    }

    fn assert_attacks_eq(board: &Board, colour: Colour, attacker: &str, squares: &[&str]) {
        let attacker: Square = attacker.parse().unwrap();
        let squares: Vec<Square> = squares.iter().map(|sq| sq.parse().unwrap()).collect();

        for square in Square::iter() {
            assert_eq!(
                attacker.u64() & get_attacks(*square, colour, board),
                if squares.contains(square) { attacker.u64() } else { 0 }
            );
        }
    }
}
