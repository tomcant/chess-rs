use crate::colour::Colour;
use crate::piece::Piece;
use crate::position::Board;
use crate::square::Square;
use lazy_static::lazy_static;

pub fn eval(colour: Colour, board: &Board) -> i32 {
    Piece::pieces_by_colour(colour).iter().fold(0, |mut acc, piece| {
        let mut pieces = board.pieces(*piece);

        while pieces > 0 {
            let square = Square::from_index(pieces.trailing_zeros() as u8);
            acc += PSQT[*piece as usize][square.index()];
            pieces ^= square.u64();
        }

        acc
    })
}

lazy_static! {
    static ref PSQT: [[i32; 64]; 12] = {
        let mut psqt = [[0; 64]; 12];

        for piece in Piece::pieces() {
            let mut psqt_index = piece.index();

            if piece.colour() == Colour::Black {
                psqt_index -= 6;
            }

            for (square, mapped_square) in SQUARE_MAP[piece.colour() as usize].iter().enumerate() {
                psqt[piece.index()][square] = PSQT_WHITE[psqt_index][*mapped_square];
            }
        }

        psqt
    };
}

#[rustfmt::skip]
const PSQT_WHITE: [[i32; 64]; 6] = [
    // Pawn
    [
         0,   0,   0,   0,   0,   0,   0,   0,
        60,  60,  60,  60,  70,  60,  60,  60,
        40,  40,  40,  50,  60,  40,  40,  40,
        20,  20,  20,  40,  50,  20,  20,  20,
         5,   5,  15,  30,  40,  10,   5,   5,
         5,   5,  10,  20,  30,   5,   5,   5,
         5,   5,   5, -30, -30,   5,   5,   5,
         0,   0,   0,   0,   0,   0,   0,   0,
    ],
    // Knight
    [
        -20, -10, -10, -10, -10, -10, -10, -20,
        -10,  -5,  -5,  -5,  -5,  -5,  -5, -10,
        -10,  -5,  15,  15,  15,  15,  -5, -10,
        -10,  -5,  15,  15,  15,  15,  -5, -10,
        -10,  -5,  15,  15,  15,  15,  -5, -10,
        -10,  -5,  10,  15,  15,  15,  -5, -10,
        -10,  -5,  -5,  -5,  -5,  -5,  -5, -10,
        -20, -10, -10, -10, -10, -10, -10, -20,
    ],
    // Bishop
    [
        -20,   0,   0,   0,   0,   0,   0, -20,
        -15,   0,   0,   0,   0,   0,   0, -15,
        -10,   0,   0,   5,   5,   0,   0, -10,
        -10,  10,  10,  30,  30,  10,  10, -10,
          5,   5,  10,  25,  25,  10,   5,   5,
          5,   5,   5,  10,  10,   5,   5,   5,
        -10,   5,   5,  10,  10,   5,   5, -10,
        -20, -10, -10, -10, -10, -10, -10, -20,
    ],
    // Rook
    [
         0,   0,   0,   0,   0,   0,   0,   0,
        15,  15,  15,  20,  20,  15,  15,  15,
         0,   0,   0,   0,   0,   0,   0,   0,
         0,   0,   0,   0,   0,   0,   0,   0,
         0,   0,   0,   0,   0,   0,   0,   0,
         0,   0,   0,   0,   0,   0,   0,   0,
         0,   0,   0,   0,   0,   0,   0,   0,
         0,   0,   0,  10,  10,  10,   0,   0,
    ],
    // Queen
    [
        -30, -20, -10, -10, -10, -10, -20, -30,
        -20, -10,  -5,  -5,  -5,  -5, -10, -20,
        -10,  -5,  10,  10,  10,  10,  -5, -10,
        -10,  -5,  10,  20,  20,  10,  -5, -10,
        -10,  -5,  10,  20,  20,  10,  -5, -10,
        -10,  -5,  -5,  -5,  -5,  -5,  -5, -10,
        -20, -10,  -5,  -5,  -5,  -5, -10, -20,
        -30, -20, -10, -10, -10, -10, -20, -30,
    ],
    // King
    [
        0,   0,   0,   0,   0,   0,   0,   0,
        0,   0,   0,   0,   0,   0,   0,   0,
        0,   0,   0,   0,   0,   0,   0,   0,
        0,   0,   0,  20,  20,   0,   0,   0,
        0,   0,   0,  20,  20,   0,   0,   0,
        0,   0,   0,   0,   0,   0,   0,   0,
        0,   0,   0, -10, -10,   0,   0,   0,
        0,   0,  20, -10, -10,   0,  20,   0,
    ],
];

#[rustfmt::skip]
const SQUARE_MAP: [[usize; 64]; 2] = [
    // White
    [
        56, 57, 58, 59, 60, 61, 62, 63,
        48, 49, 50, 51, 52, 53, 54, 55,
        40, 41, 42, 43, 44, 45, 46, 47,
        32, 33, 34, 35, 36, 37, 38, 39,
        24, 25, 26, 27, 28, 29, 30, 31,
        16, 17, 18, 19, 20, 21, 22, 23,
         8,  9, 10, 11, 12, 13, 14, 15,
         0,  1,  2,  3,  4,  5,  6,  7,
    ],
    // Black
    [
         0,  1,  2,  3,  4,  5,  6,  7,
         8,  9, 10, 11, 12, 13, 14, 15,
        16, 17, 18, 19, 20, 21, 22, 23,
        24, 25, 26, 27, 28, 29, 30, 31,
        32, 33, 34, 35, 36, 37, 38, 39,
        40, 41, 42, 43, 44, 45, 46, 47,
        48, 49, 50, 51, 52, 53, 54, 55,
        56, 57, 58, 59, 60, 61, 62, 63,
    ],
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::position::Position;

    #[test]
    fn a_pawn_is_better_when_it_is_ready_to_promote() {
        let ready_to_promote = parse_fen("8/4P3/8/8/8/8/8/8 w - - 0 1");
        let unmoved = parse_fen("8/8/8/8/8/8/4P3/8 w - - 0 1");

        assert!(eval(Colour::White, &ready_to_promote.board) > eval(Colour::White, &unmoved.board));
    }

    #[test]
    fn a_knight_on_the_edge_is_better_than_a_knight_in_the_corner() {
        let on_the_edge = parse_fen("8/8/8/8/N7/8/8/8 w - - 0 1");
        let in_the_corner = parse_fen("8/8/8/8/8/8/8/N7 w - - 0 1");

        assert!(eval(Colour::White, &on_the_edge.board) > eval(Colour::White, &in_the_corner.board));
    }

    #[test]
    fn a_knight_in_the_centre_is_better_than_a_knight_on_the_edge() {
        let in_the_centre = parse_fen("8/8/8/8/3N4/8/8/8 w - - 0 1");
        let on_the_edge = parse_fen("8/8/8/8/N7/8/8/8 w - - 0 1");

        assert!(eval(Colour::White, &in_the_centre.board) > eval(Colour::White, &on_the_edge.board));
    }

    #[test]
    fn a_bishop_in_the_centre_is_better_than_a_bishop_in_the_corner() {
        let in_the_centre = parse_fen("8/8/8/8/3B4/8/8/8 w - - 0 1");
        let in_the_corner = parse_fen("8/8/8/8/8/8/8/B7 w - - 0 1");

        assert!(eval(Colour::White, &in_the_centre.board) > eval(Colour::White, &in_the_corner.board));
    }

    #[test]
    fn a_rook_on_the_7th_rank_is_better_than_a_rook_in_the_centre() {
        let on_the_7th_rank = parse_fen("8/3R4/8/8/8/8/8/8 w - - 0 1");
        let in_the_centre = parse_fen("8/8/8/8/3R4/8/8/8 w - - 0 1");

        assert!(eval(Colour::White, &on_the_7th_rank.board) > eval(Colour::White, &in_the_centre.board));
    }

    #[test]
    fn a_castled_king_is_better_than_an_uncastled_king() {
        let castled = parse_fen("8/8/8/8/8/8/8/6K1 w - - 0 1");
        let uncastled = parse_fen("8/8/8/8/8/8/8/4K3 w - - 0 1");

        assert!(eval(Colour::White, &castled.board) > eval(Colour::White, &uncastled.board));
    }

    fn parse_fen(str: &str) -> Position {
        let pos = str.parse();
        assert!(pos.is_ok());

        pos.unwrap()
    }
}
