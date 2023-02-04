use crate::board::Board;
use crate::colour::Colour;
use crate::piece::PieceType;
use crate::position::Position;

pub const EVAL_MAX: i32 = 10_000;
pub const EVAL_MIN: i32 = -EVAL_MAX;
pub const EVAL_STALEMATE: i32 = 0;
pub const EVAL_CHECKMATE: i32 = EVAL_MIN;

pub trait Evaluator {
    fn evaluate(&self) -> i32;
}

impl Evaluator for Position {
    fn evaluate(&self) -> i32 {
        let eval = (material::evaluate(Colour::White, &self.board) - material::evaluate(Colour::Black, &self.board))
            + (psqt::evaluate(Colour::White, &self.board) - psqt::evaluate(Colour::Black, &self.board));

        match self.colour_to_move {
            Colour::White => eval,
            _ => -eval,
        }
    }
}

mod material {
    use super::*;

    const PIECE_WEIGHTS: [i32; 6] = [100, 300, 350, 500, 900, 0];

    pub fn evaluate(colour: Colour, board: &Board) -> i32 {
        PieceType::types().iter().fold(0, |acc, piece_type| {
            acc + PIECE_WEIGHTS[*piece_type as usize] * board.count_pieces(*piece_type, colour) as i32
        })
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn minor_pieces_are_worth_more_than_pawns() {
            let white_knight_black_pawn = parse_fen("8/4p3/8/8/8/8/8/6N1 w - - 0 1");
            let black_bishop_white_pawn = parse_fen("5b2/8/8/8/8/8/4P3/8 w - - 0 1");

            assert!(
                evaluate(Colour::White, &white_knight_black_pawn.board)
                    > evaluate(Colour::Black, &white_knight_black_pawn.board)
            );
            assert!(
                evaluate(Colour::Black, &black_bishop_white_pawn.board)
                    > evaluate(Colour::White, &black_bishop_white_pawn.board)
            );
        }

        #[test]
        fn rooks_are_worth_more_than_bishops() {
            let pos = parse_fen("5b2/8/8/8/8/8/8/7R w - - 0 1");

            assert!(evaluate(Colour::White, &pos.board) > evaluate(Colour::Black, &pos.board));
        }

        #[test]
        fn queens_are_worth_more_than_rooks() {
            let pos = parse_fen("7r/8/8/8/8/8/8/3Q4 w - - 0 1");

            assert!(evaluate(Colour::White, &pos.board) > evaluate(Colour::Black, &pos.board));
        }

        fn parse_fen(str: &str) -> Position {
            let pos = str.parse();
            assert!(pos.is_ok());

            pos.unwrap()
        }
    }
}

mod psqt {
    use super::*;
    use crate::square::Square;

    pub fn evaluate(colour: Colour, board: &Board) -> i32 {
        PieceType::types().iter().fold(0, |mut acc, piece_type| {
            let mut pieces = board.pieces(*piece_type, colour);

            while pieces > 0 {
                let square = Square::from_index(pieces.trailing_zeros() as u8);
                acc += PSQT[*piece_type as usize][SQUARE_MAP[colour as usize][square.index()]];
                pieces ^= square.u64();
            }

            acc
        })
    }

    #[rustfmt::skip]
    const PSQT: [[i32; 64]; 6] = [
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

        #[test]
        fn a_pawn_is_better_when_it_is_ready_to_promote() {
            let ready_to_promote = parse_fen("8/4P3/8/8/8/8/8/8 w - - 0 1");
            let unmoved = parse_fen("8/8/8/8/8/8/4P3/8 w - - 0 1");

            assert!(evaluate(Colour::White, &ready_to_promote.board) > evaluate(Colour::White, &unmoved.board));
        }

        #[test]
        fn a_knight_on_the_rim_is_dim() {
            let on_the_rim = parse_fen("8/8/8/8/N7/8/8/8 w - - 0 1");

            assert!(evaluate(Colour::White, &on_the_rim.board) < 0);
        }

        #[test]
        fn a_knight_on_the_rim_is_better_than_a_knight_in_the_corner() {
            let on_the_rim = parse_fen("8/8/8/8/N7/8/8/8 w - - 0 1");
            let in_the_corner = parse_fen("8/8/8/8/8/8/8/N7 w - - 0 1");

            assert!(evaluate(Colour::White, &on_the_rim.board) > evaluate(Colour::White, &in_the_corner.board));
        }

        #[test]
        fn a_knight_in_the_centre_is_better() {
            let in_the_centre = parse_fen("8/8/8/8/3N4/8/8/8 w - - 0 1");
            let on_the_rim = parse_fen("8/8/8/8/N7/8/8/8 w - - 0 1");

            assert!(evaluate(Colour::White, &in_the_centre.board) > evaluate(Colour::White, &on_the_rim.board));
        }

        #[test]
        fn a_bishop_in_the_centre_is_better() {
            let in_the_centre = parse_fen("8/8/8/8/3B4/8/8/8 w - - 0 1");
            let in_the_corner = parse_fen("8/8/8/8/8/8/8/B7 w - - 0 1");

            assert!(evaluate(Colour::White, &in_the_centre.board) > evaluate(Colour::White, &in_the_corner.board));
        }

        #[test]
        fn a_rook_on_the_7th_rank_is_better() {
            let on_the_7th_rank = parse_fen("8/3R4/8/8/8/8/8/8 w - - 0 1");
            let in_the_centre = parse_fen("8/8/8/8/3R4/8/8/8 w - - 0 1");

            assert!(evaluate(Colour::White, &on_the_7th_rank.board) > evaluate(Colour::White, &in_the_centre.board));
        }

        #[test]
        fn a_castled_king_is_better() {
            let castled = parse_fen("8/8/8/8/8/8/8/6K1 w - - 0 1");
            let uncastled = parse_fen("8/8/8/8/8/8/8/4K3 w - - 0 1");

            assert!(evaluate(Colour::White, &castled.board) > evaluate(Colour::White, &uncastled.board));
        }

        fn parse_fen(str: &str) -> Position {
            let pos = str.parse();
            assert!(pos.is_ok());

            pos.unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_position_is_even() {
        let pos = Position::startpos();

        assert_eq!(pos.evaluate(), 0);
    }

    #[test]
    fn more_material_is_good() {
        let pos = parse_fen("4kbnr/8/8/8/8/8/4P3/4KBNR w - - 0 1");

        assert!(pos.evaluate() > 0);
    }

    #[test]
    fn less_material_is_bad() {
        let pos = parse_fen("1nbqk3/4p3/8/8/8/8/8/1NBQK3 w - - 0 1");

        assert!(pos.evaluate() < 0);
    }

    fn parse_fen(str: &str) -> Position {
        let pos = str.parse();
        assert!(pos.is_ok());

        pos.unwrap()
    }
}
