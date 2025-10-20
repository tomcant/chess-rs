use super::*;
use crate::movegen::{generate_capture_moves, is_in_check};

pub fn search(pos: &mut Position, mut alpha: i32, beta: i32, report: &mut Report) -> i32 {
    report.nodes += 1;

    let eval = eval(pos);

    if eval >= beta {
        return beta;
    }

    if eval > alpha {
        alpha = eval;
    }

    let colour_to_move = pos.colour_to_move;

    let mut moves = generate_capture_moves(pos);
    order_capture_moves(&mut moves);

    for mv in moves {
        pos.do_move(&mv);

        if is_in_check(colour_to_move, &pos.board) {
            pos.undo_move(&mv);
            continue;
        }

        let eval = -search(pos, -beta, -alpha, report);

        pos.undo_move(&mv);

        if eval >= beta {
            return beta;
        }

        if eval > alpha {
            alpha = eval;
        }
    }

    alpha
}

fn order_capture_moves(moves: &mut [Move]) {
    moves.sort_unstable_by_key(|mv| {
        let mvv = material::PIECE_WEIGHTS[mv.captured_piece.unwrap()];
        let lva = material::PIECE_WEIGHTS[mv.piece];
        -(mvv * 100 - lva)
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::piece::Piece;
    use crate::square::Square;
    use crate::testing::*;

    #[test]
    fn order_moves_by_mvv_lva() {
        let pawn_captures_pawn = make_move(Piece::WP, Square::C4, Square::B5, Some(Piece::BP));
        let pawn_captures_queen = make_move(Piece::WP, Square::C4, Square::D5, Some(Piece::BQ));
        let knight_captures_bishop = make_move(Piece::WN, Square::F4, Square::D3, Some(Piece::BB));
        let knight_captures_queen = make_move(Piece::WN, Square::F4, Square::D5, Some(Piece::BQ));
        let knight_captures_rook = make_move(Piece::WN, Square::F4, Square::G6, Some(Piece::BR));
        let knight_captures_knight = make_move(Piece::WN, Square::F4, Square::H3, Some(Piece::BN));

        let mut moves = [
            pawn_captures_pawn,
            pawn_captures_queen,
            knight_captures_bishop,
            knight_captures_queen,
            knight_captures_rook,
            knight_captures_knight,
        ];

        order_capture_moves(&mut moves);

        assert_eq!(
            moves,
            [
                pawn_captures_queen,
                knight_captures_queen,
                knight_captures_rook,
                knight_captures_bishop,
                knight_captures_knight,
                pawn_captures_pawn,
            ],
        );
    }
}
