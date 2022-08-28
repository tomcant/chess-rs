use crate::board::{BitBoard, Board};
use crate::colour::Colour;
use crate::piece::PieceType;
use crate::square::Square;
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

    static ref BISHOP_ATTACK_RAYS: [[BitBoard; 4]; 64] = {
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

    static ref ROOK_ATTACK_RAYS: [[BitBoard; 4]; 64] = {
        fn up_ray_from(square: Square) -> BitBoard {
            let mut ray = 0;
            let mut rank = square.rank() as i8 + 1;

            while rank < 8 {
                ray += Square::from_file_and_rank(square.file(), rank as u8).u64();
                rank += 1;
            }

            ray
        }

        fn right_ray_from(square: Square) -> BitBoard {
            let mut ray = 0;
            let mut file = square.file() as i8 + 1;

            while file < 8 {
                ray += Square::from_file_and_rank(file as u8, square.rank()).u64();
                file += 1;
            }

            ray
        }

        fn left_ray_from(square: Square) -> BitBoard {
            let mut ray = 0;
            let mut file = square.file() as i8 - 1;

            while file >= 0 {
                ray += Square::from_file_and_rank(file as u8, square.rank()).u64();
                file -= 1;
            }

            ray
        }

        fn down_ray_from(square: Square) -> BitBoard {
            let mut ray = 0;
            let mut rank = square.rank() as i8 - 1;

            while rank >= 0 {
                ray += Square::from_file_and_rank(square.file(), rank as u8).u64();
                rank -= 1;
            }

            ray
        }

        let mut rays = [[0; 4]; 64];

        for square in Square::iter() {
            rays[square.index()] = [
                up_ray_from(*square),
                right_ray_from(*square),
                left_ray_from(*square),
                down_ray_from(*square),
            ];
        }

        rays
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

pub fn is_in_check(board: &Board, colour: Colour) -> bool {
    let king_square = Square::from_u64(board.get_pieces(PieceType::King, colour));
    let attackers = get_attackers(king_square, colour.flip(), board);

    attackers.count_ones() > 0
}

pub fn get_attackers(square: Square, colour: Colour, board: &Board) -> BitBoard {
    let pawn_attacks = get_pawn_attacks(square, colour.flip(), board);
    let knight_attacks = get_knight_attacks(square);
    let bishop_attacks = get_bishop_attacks(square, board);
    let rook_attacks = get_rook_attacks(square, board);
    let queen_attacks = bishop_attacks | rook_attacks;
    let king_attacks = get_king_attacks(square);

    (board.get_pieces(PieceType::Pawn, colour) & pawn_attacks)
        | (board.get_pieces(PieceType::Knight, colour) & knight_attacks)
        | (board.get_pieces(PieceType::Bishop, colour) & bishop_attacks)
        | (board.get_pieces(PieceType::Rook, colour) & rook_attacks)
        | (board.get_pieces(PieceType::Queen, colour) & queen_attacks)
        | (board.get_pieces(PieceType::King, colour) & king_attacks)
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

fn get_pawn_attacks(square: Square, colour: Colour, board: &Board) -> BitBoard {
    PAWN_ATTACKS[colour as usize][square.index()] & board.occupancy()
}

fn get_knight_attacks(square: Square) -> BitBoard {
    KNIGHT_ATTACKS[square.index()]
}

fn get_bishop_attacks(square: Square, board: &Board) -> BitBoard {
    let mut attacks = 0;

    for direction in 0..4 {
        let attack_ray = BISHOP_ATTACK_RAYS[square.index()][direction];
        let blockers_on_ray = attack_ray & board.occupancy();

        if blockers_on_ray == 0 {
            attacks |= attack_ray;
            continue;
        }

        let blocker_square_index = if direction < 2 {
            blockers_on_ray.trailing_zeros()
        } else {
            63 - blockers_on_ray.leading_zeros()
        };

        attacks |= attack_ray ^ BISHOP_ATTACK_RAYS[blocker_square_index as usize][direction];
    }

    attacks
}

fn get_rook_attacks(square: Square, board: &Board) -> BitBoard {
    let mut attacks = 0;

    for direction in 0..4 {
        let attack_ray = ROOK_ATTACK_RAYS[square.index()][direction];
        let blockers_on_ray = attack_ray & board.occupancy();

        if blockers_on_ray == 0 {
            attacks |= attack_ray;
            continue;
        }

        let blocker_square_index = if direction < 2 {
            blockers_on_ray.trailing_zeros()
        } else {
            63 - blockers_on_ray.leading_zeros()
        };

        attacks |= attack_ray ^ ROOK_ATTACK_RAYS[blocker_square_index as usize][direction];
    }

    attacks
}

fn get_king_attacks(square: Square) -> BitBoard {
    KING_ATTACKS[square.index()]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::GameState;
    use crate::piece::Piece;

    #[test]
    fn test_it_can_detect_check() {
        let mut board = Board::empty();
        board.put_piece(Piece::BlackKing, "e8".parse::<Square>().unwrap());
        board.put_piece(Piece::WhiteKnight, "d6".parse::<Square>().unwrap());

        assert!(is_in_check(&board, Colour::Black));
    }

    #[test]
    fn test_attack_by_queen() {
        let fen = "4k3/8/8/8/4Q3/8/8/8 w - - 0 1";
        let state: GameState = fen.parse().unwrap();

        assert_eq!(
            get_attackers("e8".parse::<Square>().unwrap(), Colour::White, &state.board),
            "e4".parse::<Square>().unwrap().u64()
        );
    }

    #[test]
    fn test_attack_by_queen_diagonal() {
        let fen = "4k3/8/8/8/Q7/8/8/8 w - - 0 1";
        let state: GameState = fen.parse().unwrap();

        assert_eq!(
            get_attackers("e8".parse::<Square>().unwrap(), Colour::White, &state.board),
            "a4".parse::<Square>().unwrap().u64()
        );
    }

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

    #[test]
    fn test_bishop_attacks_with_up_left_blocker() {
        let fen = "8/8/2n5/8/8/8/6B1/8 w - - 0 1";
        let state: GameState = fen.parse().unwrap();

        assert_attacks_eq(&state, "g2", &["h1", "h3", "f1", "f3", "e4", "d5", "c6"]);
    }

    #[test]
    fn test_bishop_attacks_with_up_right_blocker() {
        let fen = "8/8/5n2/8/8/8/1B6/8 w - - 0 1";
        let state: GameState = fen.parse().unwrap();

        assert_attacks_eq(&state, "b2", &["a1", "a3", "c1", "c3", "d4", "e5", "f6"]);
    }

    #[test]
    fn test_bishop_attacks_with_down_left_blocker() {
        let fen = "8/8/8/4B3/3n4/8/8/8 w - - 0 1";
        let state: GameState = fen.parse().unwrap();

        assert_attacks_eq(
            &state,
            "e5",
            &["h8", "g7", "f6", "d4", "f4", "g3", "h2", "d6", "c7", "b8"],
        );
    }

    #[test]
    fn test_bishop_attacks_with_down_right_blocker() {
        let fen = "8/8/8/3b4/8/5N2/8/8 w - - 0 1";
        let state: GameState = fen.parse().unwrap();

        assert_attacks_eq(
            &state,
            "d5",
            &["a8", "b7", "c6", "e6", "f7", "g8", "c4", "b3", "a2", "e4", "f3"],
        );
    }

    #[test]
    fn test_rook_attacks_on_empty_board() {
        let fen = "8/8/8/8/3r4/8/8/8 b - - 0 1";
        let state: GameState = fen.parse().unwrap();

        assert_attacks_eq(
            &state,
            "d4",
            &[
                "d1", "d2", "d3", "d5", "d6", "d7", "d8", "a4", "b4", "c4", "e4", "f4", "g4", "h4",
            ],
        );
    }

    #[test]
    fn test_rook_attacks_with_up_blocker() {
        let fen = "8/8/8/3N4/8/8/8/3r4 b - - 0 1";
        let state: GameState = fen.parse().unwrap();

        assert_attacks_eq(
            &state,
            "d1",
            &["d2", "d3", "d4", "d5", "a1", "b1", "c1", "e1", "f1", "g1", "h1"],
        );
    }

    #[test]
    fn test_rook_attacks_with_right_blocker() {
        let fen = "8/8/8/r2N4/8/8/8/8 b - - 0 1";
        let state: GameState = fen.parse().unwrap();

        assert_attacks_eq(
            &state,
            "a5",
            &["b5", "c5", "d5", "a6", "a7", "a8", "a4", "a3", "a2", "a1"],
        );
    }

    #[test]
    fn test_rook_attacks_with_left_blocker() {
        let fen = "8/8/8/3N3r/8/8/8/8 b - - 0 1";
        let state: GameState = fen.parse().unwrap();

        assert_attacks_eq(
            &state,
            "h5",
            &["g5", "f5", "e5", "d5", "h6", "h7", "h8", "h4", "h3", "h2", "h1"],
        );
    }

    #[test]
    fn test_rook_attacks_with_down_blocker() {
        let fen = "3r4/8/8/3N4/8/8/8/8 b - - 0 1";
        let state: GameState = fen.parse().unwrap();

        assert_attacks_eq(
            &state,
            "d8",
            &["d7", "d6", "d5", "a8", "b8", "c8", "e8", "f8", "g8", "h8"],
        );
    }

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
}