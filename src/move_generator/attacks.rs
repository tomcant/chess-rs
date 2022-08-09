use crate::board::{BitBoard, Board, Colour, Piece, PieceType, Square};
use lazy_static::lazy_static;

const FILE_A: BitBoard = 0x0101_0101_0101_0101;
const FILE_B: BitBoard = 0x0202_0202_0202_0202;
const FILE_G: BitBoard = 0x4040_4040_4040_4040;
const FILE_H: BitBoard = 0x8080_8080_8080_8080;

lazy_static! {
    static ref PAWN_ATTACKS: [[BitBoard; 64]; 2] = {
        let mut attacks = [[0; 64]; 2];

        for square in Square::iter() {
            let square_u64 = square.u64();

            attacks[Colour::White as usize][square.index()] =
                  (square_u64 & !FILE_A) << 7 | (square_u64 & !FILE_H) << 9;

            attacks[Colour::Black as usize][square.index()] =
                  (square_u64 & !FILE_H) >> 7 | (square_u64 & !FILE_A) >> 9;
        }

        attacks
    };

    static ref KNIGHT_ATTACKS: [BitBoard; 64] = {
        let mut attacks = [0; 64];

        for square in Square::iter() {
            let square_u64 = square.u64();

            attacks[square.index()] =
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
            rays[square.index()] = [
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
            let square_index = square.index();
            attacks[square_index] = BISHOP_RAYS[square_index].iter().sum();
        }

        attacks
    };

    static ref KING_ATTACKS: [BitBoard; 64] = {
        let mut attacks = [0; 64];

        for square in Square::iter() {
            let square_u64 = square.u64();

            attacks[square.index()] =
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

pub fn get_attackers(square: Square, colour: Colour, board: &Board) -> BitBoard {
    (board.get_pieces(Piece::make(PieceType::Pawn, colour)) & get_pawn_attacks(square, colour.flip(), board))
        | (board.get_pieces(Piece::make(PieceType::Knight, colour)) & get_knight_attacks(square))
        | (board.get_pieces(Piece::make(PieceType::Bishop, colour)) & get_bishop_attacks(square, &board))
        | (board.get_pieces(Piece::make(PieceType::Rook, colour)) & get_rook_attacks(square, &board))
        | (board.get_pieces(Piece::make(PieceType::King, colour)) & get_king_attacks(square))
}

pub fn get_attacks(square: Square, board: &Board) -> BitBoard {
    let maybe_piece = board.get_piece_at(square);

    if maybe_piece.is_none() {
        return 0;
    }

    let piece = maybe_piece.unwrap();

    match piece.get_type() {
        PieceType::Pawn => get_pawn_attacks(square, piece.colour(), board),
        PieceType::Knight => get_knight_attacks(square),
        PieceType::Bishop => get_bishop_attacks(square, board),
        PieceType::Rook => get_rook_attacks(square, board),
        PieceType::Queen => get_bishop_attacks(square, board) | get_rook_attacks(square, board),
        PieceType::King => get_king_attacks(square),
    }
}

pub fn get_pawn_attacks(square: Square, colour: Colour, board: &Board) -> BitBoard {
    PAWN_ATTACKS[colour as usize][square.index()] & board.occupancy()
}

pub fn get_knight_attacks(square: Square) -> BitBoard {
    KNIGHT_ATTACKS[square.index()]
}

pub fn get_bishop_attacks(square: Square, board: &Board) -> BitBoard {
    // let b = board.occupancy() & BISHOP_RAYS[square.index()][0];
    //
    // println!(
    //     "b = {b:?} occ = {} ray = {}",
    //     board.occupancy(),
    //     BISHOP_RAYS[square.index()][0]
    // );

    BISHOP_ATTACKS[square.index()]
}

pub fn get_rook_attacks(square: Square, board: &Board) -> BitBoard {
    0
}

pub fn get_king_attacks(square: Square) -> BitBoard {
    KING_ATTACKS[square.index()]
}

#[cfg(test)]
mod tests {
    use crate::{
        board::{BitBoard, Square},
        game_state::GameState,
        move_generator::attacks::{get_attackers, get_attacks},
    };

    #[test]
    fn test_white_pawn_attacks_none() {
        let fen = "8/8/8/8/8/8/4P3/8 w - - 0 1";
        let state: GameState = fen.parse().unwrap();

        assert_attacks_eq(&state, "e2", &[]);
    }

    #[test]
    fn test_white_pawn_attacks_left() {
        let fen = "8/8/8/8/8/3p4/4P3/8 w - - 0 1";
        let state: GameState = fen.parse().unwrap();

        assert_attacks_eq(&state, "e2", &["d3"]);
    }

    #[test]
    fn test_white_pawn_attacks_right() {
        let fen = "8/8/8/8/8/5p2/4P3/8 w - - 0 1";
        let state: GameState = fen.parse().unwrap();

        assert_attacks_eq(&state, "e2", &["f3"]);
    }

    #[test]
    fn test_white_pawn_attacks_left_and_right() {
        let fen = "8/8/8/8/8/3p1p2/4P3/8 w - - 0 1";
        let state: GameState = fen.parse().unwrap();

        assert_attacks_eq(&state, "e2", &["d3", "f3"]);
    }

    #[test]
    fn test_black_pawn_attacks_none() {
        let fen = "8/4p3/8/8/8/8/8/8 b - - 0 1";
        let state: GameState = fen.parse().unwrap();

        assert_attacks_eq(&state, "e7", &[]);
    }

    #[test]
    fn test_black_pawn_attacks_left() {
        let fen = "8/4p3/3P4/8/8/8/8/8 b - - 0 1";
        let state: GameState = fen.parse().unwrap();

        assert_attacks_eq(&state, "e7", &["d6"]);
    }

    #[test]
    fn test_black_pawn_attacks_right() {
        let fen = "8/4p3/5P2/8/8/8/8/8 b - - 0 1";
        let state: GameState = fen.parse().unwrap();

        assert_attacks_eq(&state, "e7", &["f6"]);
    }

    #[test]
    fn test_black_pawn_attacks_left_and_right() {
        let fen = "8/4p3/3P1P2/8/8/8/8/8 b - - 0 1";
        let state: GameState = fen.parse().unwrap();

        assert_attacks_eq(&state, "e7", &["d6", "f6"]);
    }

    #[test]
    fn test_knight_attacks() {
        let fen = "8/8/8/8/3N4/8/8/8 w - - 0 1";
        let state: GameState = fen.parse().unwrap();

        assert_attacks_eq(&state, "d4", &["c2", "e2", "b3", "f3", "b5", "f5", "c6", "e6"]);
    }

    #[test]
    fn test_bishop_attacks_on_empty_board() {
        let fen = "8/8/8/8/3b4/8/8/8 b - - 0 1";
        let state: GameState = fen.parse().unwrap();

        assert_attacks_eq(
            &state,
            "d4",
            &[
                "a1", "g1", "b2", "f2", "c3", "e3", "c5", "e5", "b6", "f6", "a7", "g7", "h8",
            ],
        );
    }

    // #[test]
    // fn test_bishop_attacks_on_occupied_board() {
    //     let fen = "r7/8/2n5/8/8/8/6B1/8 w - - 0 1";
    //     let state: GameState = fen.parse().unwrap();

    //     let attacker = "g2".parse::<Square>().unwrap();

    //     let mut attacks = 0;
    //     for sq in ["f3", "e4", "d5", "c6"] {
    //         attacks += sq.parse::<Square>().unwrap().u64();
    //     }
    //     println!("attacks = {attacks:?}");

    //     assert_eq!(get_attacks(attacker, &state.board), attacks);

    //     // assert_attacks_eq(&state, "g2", &["f3", "e4", "d5", "c6"]);
    // }

    #[test]
    fn test_king_attacks() {
        let fen = "8/8/8/8/8/8/8/4K3 w - - 0 1";
        let state: GameState = fen.parse().unwrap();

        assert_attacks_eq(&state, "e1", &["d1", "f1", "d2", "e2", "f2"]);
    }

    fn assert_attacks_eq(state: &GameState, attacker: &str, squares: &[&str]) {
        let attacks: BitBoard = squares.iter().map(|sq| sq.parse::<Square>().unwrap().u64()).sum();

        assert_eq!(attacks, get_attacks(attacker.parse().unwrap(), &state.board));
    }

    // #[test]
    // fn test_attack_by_king() {
    //     let fen = "8/8/8/8/8/8/8/4K3 w - - 0 1";
    //     let state: GameState = fen.parse().unwrap();

    //     assert_attackers_eq(&state, "e1", &["d1", "f1", "d2", "e2", "f2"]);
    // }

    // fn assert_attackers_eq(state: &GameState, square: &str, attackers: &[&str]) {
    //     let square: Square = square.parse().unwrap();
    //     let attackers: Vec<Square> = attackers.iter().map(|sq| sq.parse().unwrap()).collect();

    //     for sq in Square::iter() {
    //         assert_eq!(
    //             square.u64() & get_attackers(*sq, state.colour_to_move, &state.board),
    //             if attackers.contains(sq) { square.u64() } else { 0 }
    //         );
    //     }
    // }
}
