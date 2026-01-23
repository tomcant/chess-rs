use super::EvalTerm;
use crate::colour::Colour;
use crate::piece::Piece;
use crate::position::Board;
use crate::square::{FILES, Square};
use lazy_static::lazy_static;

const DOUBLED_MG: i32 = -12;
const DOUBLED_EG: i32 = -8;

const ISOLATED_MG: i32 = -10;
const ISOLATED_EG: i32 = -8;

const PASSED_MG: [[i32; 8]; 2] = [
    [0, 0, 5, 10, 20, 35, 60, 0], // White
    [0, 60, 35, 20, 10, 5, 0, 0], // Black
];
const PASSED_EG: [[i32; 8]; 2] = [
    [0, 5, 10, 20, 35, 60, 90, 0], // White
    [0, 90, 60, 35, 20, 10, 5, 0], // Black
];

pub fn eval(colour: Colour, board: &Board) -> EvalTerm {
    doubled(colour, board) + isolated(colour, board) + passed(colour, board)
}

fn doubled(colour: Colour, board: &Board) -> EvalTerm {
    let (mut mg, mut eg) = (0, 0);
    let pawns = board.pieces(Piece::pawn(colour));

    for file in FILES {
        let pawns_on_file = (pawns & file).count_ones() as i32;

        if pawns_on_file > 1 {
            mg += (pawns_on_file - 1) * DOUBLED_MG;
            eg += (pawns_on_file - 1) * DOUBLED_EG;
        }
    }

    EvalTerm::new(mg, eg)
}

fn isolated(colour: Colour, board: &Board) -> EvalTerm {
    let (mut mg, mut eg) = (0, 0);
    let pawns = board.pieces(Piece::pawn(colour));

    for file in 0..8 {
        let pawns_on_file = pawns & FILES[file];

        if pawns_on_file == 0 {
            continue;
        }

        if pawns & ADJACENT_FILES[file] == 0 {
            let isolated_pawns = pawns_on_file.count_ones() as i32;
            mg += isolated_pawns * ISOLATED_MG;
            eg += isolated_pawns * ISOLATED_EG;
        }
    }

    EvalTerm::new(mg, eg)
}

fn passed(colour: Colour, board: &Board) -> EvalTerm {
    let (mut mg, mut eg) = (0, 0);
    let mut our_pawns = board.pieces(Piece::pawn(colour));
    let their_pawns = board.pieces(Piece::pawn(colour.flip()));

    while our_pawns != 0 {
        let square = Square::next(&mut our_pawns);
        let file = square.file() as usize;
        let blockers = their_pawns & (FILES[file] | ADJACENT_FILES[file]);

        if blockers & SQUARES_IN_FRONT[colour][square] == 0 {
            let rank = square.rank() as usize;
            mg += PASSED_MG[colour][rank];
            eg += PASSED_EG[colour][rank];
        }
    }

    EvalTerm::new(mg, eg)
}

const ADJACENT_FILES: [u64; 8] = [
    FILES[1],
    FILES[0] | FILES[2],
    FILES[1] | FILES[3],
    FILES[2] | FILES[4],
    FILES[3] | FILES[5],
    FILES[4] | FILES[6],
    FILES[5] | FILES[7],
    FILES[6],
];

lazy_static! {
    static ref SQUARES_IN_FRONT: [[u64; 64]; 2] = [
        build_squares_in_front(Colour::White),
        build_squares_in_front(Colour::Black),
    ];
}

fn build_squares_in_front(colour: Colour) -> [u64; 64] {
    let mut masks = [0; 64];
    let squares: [_; 64] = std::array::from_fn(|index| Square::from_index(index as u8));

    for square in squares {
        let rank = square.rank() as u32;

        masks[square] = match colour {
            Colour::White => {
                if rank < 7 {
                    !((1 << ((rank + 1) << 3)) - 1)
                } else {
                    0
                }
            }
            _ => {
                if rank > 0 {
                    (1 << (rank << 3)) - 1
                } else {
                    0
                }
            }
        };
    }

    masks
}
