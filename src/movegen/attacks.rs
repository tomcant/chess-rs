use crate::colour::Colour;
use crate::piece::Piece::{self, *};
use crate::position::Board;
use crate::square::Square;
use lazy_static::lazy_static;

// Include build-generated magic tables
include!(concat!(env!("OUT_DIR"), "/magic.rs"));

const FILE_A: u64 = 0x0101_0101_0101_0101;
const FILE_B: u64 = 0x0202_0202_0202_0202;
const FILE_G: u64 = 0x4040_4040_4040_4040;
const FILE_H: u64 = 0x8080_8080_8080_8080;

#[inline]
pub fn is_in_check(colour: Colour, board: &Board) -> bool {
    let king_square = Square::first(board.pieces(Piece::king(colour)));

    is_attacked(king_square, colour.flip(), board)
}

#[inline]
pub fn is_attacked(square: Square, colour: Colour, board: &Board) -> bool {
    get_attackers(square, colour, board) != 0
}

#[inline]
pub fn get_attackers(square: Square, colour: Colour, board: &Board) -> u64 {
    let pawn_attacks = get_pawn_attacks(square, colour.flip(), board);
    let knight_attacks = get_knight_attacks(square);
    let bishop_attacks = get_bishop_attacks(square, board);
    let rook_attacks = get_rook_attacks(square, board);
    let queen_attacks = bishop_attacks | rook_attacks;
    let king_attacks = get_king_attacks(square);

    (board.pieces(Piece::pawn(colour)) & pawn_attacks)
        | (board.pieces(Piece::knight(colour)) & knight_attacks)
        | (board.pieces(Piece::bishop(colour)) & bishop_attacks)
        | (board.pieces(Piece::rook(colour)) & rook_attacks)
        | (board.pieces(Piece::queen(colour)) & queen_attacks)
        | (board.pieces(Piece::king(colour)) & king_attacks)
}

#[inline]
pub fn get_attacks(piece: Piece, square: Square, board: &Board) -> u64 {
    match piece {
        WP | BP => get_pawn_attacks(square, piece.colour(), board),
        WN | BN => get_knight_attacks(square),
        WB | BB => get_bishop_attacks(square, board),
        WR | BR => get_rook_attacks(square, board),
        WQ | BQ => get_bishop_attacks(square, board) | get_rook_attacks(square, board),
        WK | BK => get_king_attacks(square),
    }
}

#[inline]
pub fn get_en_passant_attacks(en_passant_square: Square, colour: Colour, board: &Board) -> u64 {
    PAWN_ATTACKS[colour.flip()][en_passant_square] & board.pieces(Piece::pawn(colour))
}

#[inline]
fn get_pawn_attacks(square: Square, colour: Colour, board: &Board) -> u64 {
    PAWN_ATTACKS[colour][square] & board.pieces_by_colour(colour.flip())
}

#[inline]
fn get_knight_attacks(square: Square) -> u64 {
    KNIGHT_ATTACKS[square]
}

#[inline]
fn get_bishop_attacks(square: Square, board: &Board) -> u64 {
    let magic = &BISHOP_MAGICS[square];
    let occupancy = board.occupancy() & magic.mask;
    let index = ((occupancy.wrapping_mul(magic.num)) >> magic.shift) as usize;

    BISHOP_ATTACKS[magic.offset + index]
}

#[inline]
fn get_rook_attacks(square: Square, board: &Board) -> u64 {
    let magic = &ROOK_MAGICS[square];
    let occupancy = board.occupancy() & magic.mask;
    let index = ((occupancy.wrapping_mul(magic.num)) >> magic.shift) as usize;

    ROOK_ATTACKS[magic.offset + index]
}

#[inline]
fn get_king_attacks(square: Square) -> u64 {
    KING_ATTACKS[square]
}

lazy_static! {
    static ref SQUARES: [Square; 64] = (0..64).map(Square::from_index).collect::<Vec<_>>().try_into().unwrap();

    static ref PAWN_ATTACKS: [[u64; 64]; 2] = {
        let mut attacks = [[0; 64]; 2];

        for square in SQUARES.iter() {
            let square_u64 = square.u64();

            attacks[Colour::White][*square] =
                  (square_u64 & !FILE_A) << 7 | (square_u64 & !FILE_H) << 9;

            attacks[Colour::Black][*square] =
                  (square_u64 & !FILE_H) >> 7 | (square_u64 & !FILE_A) >> 9;
        }

        attacks
    };

    static ref KNIGHT_ATTACKS: [u64; 64] = {
        let mut attacks = [0; 64];

        for square in SQUARES.iter() {
            let square_u64 = square.u64();

            attacks[*square] =
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

    static ref KING_ATTACKS: [u64; 64] = {
        let mut attacks = [0; 64];

        for square in SQUARES.iter() {
            let square_u64 = square.u64();

            attacks[*square] =
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::position::Position;
    use crate::testing::*;

    #[test]
    fn detect_check() {
        let mut board = Board::empty();
        board.put_piece(BK, Square::E8);
        board.put_piece(WN, Square::D6);

        assert!(is_in_check(Colour::Black, &board));
    }

    #[test]
    fn attack_by_queen_horizontal() {
        let pos = parse_fen("Q3k3/8/8/8/8/8/8/8 w - - 0 1");

        assert_eq!(get_attackers(Square::E8, Colour::White, &pos.board), Square::A8.u64());
    }

    #[test]
    fn attack_by_queen_vertical() {
        let pos = parse_fen("4k3/8/8/8/4Q3/8/8/8 w - - 0 1");

        assert_eq!(get_attackers(Square::E8, Colour::White, &pos.board), Square::E4.u64());
    }

    #[test]
    fn attack_by_queen_diagonal() {
        let pos = parse_fen("4k3/8/8/8/Q7/8/8/8 w - - 0 1");

        assert_eq!(get_attackers(Square::E8, Colour::White, &pos.board), Square::A4.u64());
    }

    #[test]
    fn white_pawn_attacks_none() {
        let pos = parse_fen("8/8/8/8/8/8/4P3/8 w - - 0 1");

        assert_attacks_eq(&pos, "e2", &[]);
    }

    #[test]
    fn white_pawn_attacks_left() {
        let pos = parse_fen("8/8/8/8/8/3p4/4P3/8 w - - 0 1");

        assert_attacks_eq(&pos, "e2", &["d3"]);
    }

    #[test]
    fn white_pawn_attacks_right() {
        let pos = parse_fen("8/8/8/8/8/5p2/4P3/8 w - - 0 1");

        assert_attacks_eq(&pos, "e2", &["f3"]);
    }

    #[test]
    fn white_pawn_attacks_left_and_right() {
        let pos = parse_fen("8/8/8/8/8/3p1p2/4P3/8 w - - 0 1");

        assert_attacks_eq(&pos, "e2", &["d3", "f3"]);
    }

    #[test]
    fn black_pawn_attacks_none() {
        let pos = parse_fen("8/4p3/8/8/8/8/8/8 b - - 0 1");

        assert_attacks_eq(&pos, "e7", &[]);
    }

    #[test]
    fn black_pawn_attacks_left() {
        let pos = parse_fen("8/4p3/3P4/8/8/8/8/8 b - - 0 1");

        assert_attacks_eq(&pos, "e7", &["d6"]);
    }

    #[test]
    fn black_pawn_attacks_right() {
        let pos = parse_fen("8/4p3/5P2/8/8/8/8/8 b - - 0 1");

        assert_attacks_eq(&pos, "e7", &["f6"]);
    }

    #[test]
    fn black_pawn_attacks_left_and_right() {
        let pos = parse_fen("8/4p3/3P1P2/8/8/8/8/8 b - - 0 1");

        assert_attacks_eq(&pos, "e7", &["d6", "f6"]);
    }

    #[test]
    fn knight_attacks() {
        let pos = parse_fen("8/8/8/8/3N4/8/8/8 w - - 0 1");

        assert_attacks_eq(&pos, "d4", &["c2", "e2", "b3", "f3", "b5", "f5", "c6", "e6"]);
    }

    #[test]
    fn bishop_attacks_on_empty_board() {
        let pos = parse_fen("8/8/8/8/3b4/8/8/8 b - - 0 1");

        assert_attacks_eq(
            &pos,
            "d4",
            &[
                "a1", "g1", "b2", "f2", "c3", "e3", "c5", "e5", "b6", "f6", "a7", "g7", "h8",
            ],
        );
    }

    #[test]
    fn bishop_attacks_with_up_left_blocker() {
        let pos = parse_fen("8/8/2n5/8/8/8/6B1/8 w - - 0 1");

        assert_attacks_eq(&pos, "g2", &["h1", "h3", "f1", "f3", "e4", "d5", "c6"]);
    }

    #[test]
    fn bishop_attacks_with_up_right_blocker() {
        let pos = parse_fen("8/8/5n2/8/8/8/1B6/8 w - - 0 1");

        assert_attacks_eq(&pos, "b2", &["a1", "a3", "c1", "c3", "d4", "e5", "f6"]);
    }

    #[test]
    fn bishop_attacks_with_down_left_blocker() {
        let pos = parse_fen("8/8/8/4B3/3n4/8/8/8 w - - 0 1");

        assert_attacks_eq(
            &pos,
            "e5",
            &["h8", "g7", "f6", "d4", "f4", "g3", "h2", "d6", "c7", "b8"],
        );
    }

    #[test]
    fn bishop_attacks_with_down_right_blocker() {
        let pos = parse_fen("8/8/8/3b4/8/5N2/8/8 w - - 0 1");

        assert_attacks_eq(
            &pos,
            "d5",
            &["a8", "b7", "c6", "e6", "f7", "g8", "c4", "b3", "a2", "e4", "f3"],
        );
    }

    #[test]
    fn rook_attacks_on_empty_board() {
        let pos = parse_fen("8/8/8/8/3r4/8/8/8 b - - 0 1");

        assert_attacks_eq(
            &pos,
            "d4",
            &[
                "d1", "d2", "d3", "d5", "d6", "d7", "d8", "a4", "b4", "c4", "e4", "f4", "g4", "h4",
            ],
        );
    }

    #[test]
    fn rook_attacks_with_up_blocker() {
        let pos = parse_fen("8/8/8/3N4/8/8/8/3r4 b - - 0 1");

        assert_attacks_eq(
            &pos,
            "d1",
            &["d2", "d3", "d4", "d5", "a1", "b1", "c1", "e1", "f1", "g1", "h1"],
        );
    }

    #[test]
    fn rook_attacks_with_right_blocker() {
        let pos = parse_fen("8/8/8/r2N4/8/8/8/8 b - - 0 1");

        assert_attacks_eq(
            &pos,
            "a5",
            &["b5", "c5", "d5", "a6", "a7", "a8", "a4", "a3", "a2", "a1"],
        );
    }

    #[test]
    fn rook_attacks_with_left_blocker() {
        let pos = parse_fen("8/8/8/3N3r/8/8/8/8 b - - 0 1");

        assert_attacks_eq(
            &pos,
            "h5",
            &["g5", "f5", "e5", "d5", "h6", "h7", "h8", "h4", "h3", "h2", "h1"],
        );
    }

    #[test]
    fn rook_attacks_with_down_blocker() {
        let pos = parse_fen("3r4/8/8/3N4/8/8/8/8 b - - 0 1");

        assert_attacks_eq(
            &pos,
            "d8",
            &["d7", "d6", "d5", "a8", "b8", "c8", "e8", "f8", "g8", "h8"],
        );
    }

    #[test]
    fn king_attacks() {
        let pos = parse_fen("8/8/8/8/8/8/8/4K3 w - - 0 1");

        assert_attacks_eq(&pos, "e1", &["d1", "f1", "d2", "e2", "f2"]);
    }

    fn assert_attacks_eq(pos: &Position, attacker: &str, squares: &[&str]) {
        let attacker = attacker.parse().unwrap();
        let attacks: u64 = squares
            .iter()
            .map(|square| square.parse::<Square>().unwrap().u64())
            .sum();

        assert_eq!(
            attacks,
            get_attacks(pos.board.piece_at(attacker).unwrap(), attacker, &pos.board)
        );
    }
}
