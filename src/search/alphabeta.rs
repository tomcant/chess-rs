use super::{
    killers::KillerMoves,
    tt::{Bound, Table},
    *,
};
use crate::movegen::{generate_all_moves, is_in_check};

#[rustfmt::skip]
#[allow(clippy::too_many_arguments)]
pub fn search(
    pos: &mut Position,
    mut depth: u8,
    mut alpha: i32,
    beta: i32,
    pv: &mut MoveList,
    tt: &mut Table,
    killers: &mut KillerMoves,
    report: &mut Report,
    stopper: &impl Stopper,
) -> i32 {
    if stopper.should_stop(report) {
        return 0;
    }

    if pos.is_threefold_repetition() {
        return EVAL_STALEMATE;
    }

    if depth == 0 {
        if !is_in_check(pos.colour_to_move, &pos.board) {
            return quiescence::search(pos, alpha, beta, report);
        }

        // Extend the search if we're in check so that quiescence doesn't need
        // to consider possible evasions and can remain focused on captures.
        depth = 1;
    }

    let key = pos.key;
    let mut tt_move = None;

    if let Some(entry) = tt.probe(key) {
        if let Some(mv) = entry.mv {
            tt_move = entry.mv;
            pv.clear();
            pv.push(mv);
        }

        if entry.depth >= depth {
            match entry.bound {
                Bound::Exact => return entry.eval,
                Bound::Lower if entry.eval >= beta => return beta,
                Bound::Upper if entry.eval <= alpha => return alpha,
                _ => (),
            };
        }
    }

    report.nodes += 1;

    let mut has_legal_move = false;
    let mut tt_bound = Bound::Upper;

    // Search the TT move before generating other moves because there's a good
    // chance it leads to a cutoff
    if let Some(mv) = tt_move {
        pos.do_move(&mv);
        report.ply += 1;

        let mut child_pv = MoveList::new();
        let eval = -search(pos, depth - 1, -beta, -alpha, &mut child_pv, tt, killers, report, stopper);

        report.ply -= 1;
        pos.undo_move(&mv);

        if eval >= beta {
            if mv.captured_piece.is_none() {
                killers.store(report.ply, mv);
            }

            tt.store(key, depth, beta, Bound::Lower, tt_move);
            return beta;
        }

        if eval > alpha {
            alpha = eval;
            tt_bound = Bound::Exact;

            pv.clear();
            pv.push(mv);
            pv.append(&mut child_pv);
        }

        has_legal_move = true;
    }

    let colour_to_move = pos.colour_to_move;

    let mut moves = generate_all_moves(pos);
    order_moves(&mut moves, killers, report.ply);

    for mv in &moves {
        if tt_move.is_some() && mv.equals(&tt_move.unwrap()) {
            continue;
        }

        pos.do_move(mv);

        if is_in_check(colour_to_move, &pos.board) {
            pos.undo_move(mv);
            continue;
        }

        has_legal_move = true;
        report.ply += 1;

        let mut child_pv = MoveList::new();
        let eval = -search(pos, depth - 1, -beta, -alpha, &mut child_pv, tt, killers, report, stopper);

        report.ply -= 1;
        pos.undo_move(mv);

        if eval >= beta {
            if mv.captured_piece.is_none() {
                killers.store(report.ply, *mv);
            }

            tt.store(key, depth, beta, Bound::Lower, Some(*mv));
            return beta;
        }

        if eval > alpha {
            alpha = eval;
            tt_bound = Bound::Exact;
            tt_move = Some(*mv);

            pv.clear();
            pv.push(*mv);
            pv.append(&mut child_pv);
        }
    }

    if !has_legal_move {
        return if is_in_check(colour_to_move, &pos.board) {
            -EVAL_CHECKMATE + report.ply as i32
        } else {
            EVAL_STALEMATE
        };
    }

    tt.store(key, depth, alpha, tt_bound, tt_move);

    alpha
}

fn order_moves(moves: &mut [Move], killers: &KillerMoves, ply: u8) {
    moves.sort_unstable_by_key(|mv| {
        if let Some(victim) = mv.captured_piece {
            let mvv = material::PIECE_WEIGHTS[victim];
            let lva = material::PIECE_WEIGHTS[mv.piece];
            return -(mvv * 100 - lva);
        }

        for i in 0..2 {
            if let Some(killer) = killers.probe(ply, i)
                && mv.equals(&killer)
            {
                return i as i32;
            }
        }

        3
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::piece::Piece;
    use crate::square::Square;
    use crate::testing::*;

    #[test]
    fn order_moves_by_mvv_lva_and_killers() {
        let quiet_move = make_move(Piece::WP, Square::C4, Square::C5, None);
        let killer_move1 = make_move(Piece::WP, Square::A2, Square::A3, None);
        let killer_move2 = make_move(Piece::WP, Square::B2, Square::B3, None);
        let pawn_captures_pawn = make_move(Piece::WP, Square::C4, Square::B5, Some(Piece::BP));
        let pawn_captures_queen = make_move(Piece::WP, Square::C4, Square::D5, Some(Piece::BQ));
        let knight_captures_bishop = make_move(Piece::WN, Square::F4, Square::D3, Some(Piece::BB));
        let knight_captures_queen = make_move(Piece::WN, Square::F4, Square::D5, Some(Piece::BQ));
        let knight_captures_rook = make_move(Piece::WN, Square::F4, Square::G6, Some(Piece::BR));
        let knight_captures_knight = make_move(Piece::WN, Square::F4, Square::H3, Some(Piece::BN));

        let mut moves = [
            quiet_move,
            killer_move1,
            killer_move2,
            pawn_captures_pawn,
            pawn_captures_queen,
            knight_captures_bishop,
            knight_captures_queen,
            knight_captures_rook,
            knight_captures_knight,
        ];

        let killer_ply = 0;
        let mut killers = KillerMoves::new();
        killers.store(killer_ply, killer_move2);
        killers.store(killer_ply, killer_move1);

        order_moves(&mut moves, &killers, killer_ply);

        assert_eq!(
            moves,
            [
                pawn_captures_queen,
                knight_captures_queen,
                knight_captures_rook,
                knight_captures_bishop,
                knight_captures_knight,
                pawn_captures_pawn,
                killer_move1,
                killer_move2,
                quiet_move,
            ],
        );
    }
}
