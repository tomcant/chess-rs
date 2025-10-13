use super::{
    tt::{Bound, Table},
    *,
};
use crate::movegen::{generate_all_moves, is_in_check};

#[allow(clippy::too_many_arguments)]
pub fn search(
    pos: &mut Position,
    mut depth: u8,
    mut alpha: i32,
    beta: i32,
    pv: &mut MoveList,
    tt: &mut Table,
    report: &mut Report,
    stopper: &impl Stopper,
) -> i32 {
    if stopper.should_stop(report) {
        return 0;
    }

    if depth == 0 {
        if !is_in_check(pos.colour_to_move, &pos.board) {
            return quiescence::search(pos, alpha, beta, report);
        }

        // Extend the search if we're in check so that quiescence doesn't need
        // to consider possible evasions and can remain focused on captures.
        depth = 1;
    }

    let key = pos.key();
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
        let eval = -search(pos, depth - 1, -beta, -alpha, &mut child_pv, tt, report, stopper);

        report.ply -= 1;
        pos.undo_move(&mv);

        if eval >= beta {
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
    order_moves(&mut moves, &pos.board, tt_move);
    let start_index = if tt_move.is_some() { 1 } else { 0 };

    for mv in &moves[start_index..] {
        pos.do_move(mv);

        if is_in_check(colour_to_move, &pos.board) {
            pos.undo_move(mv);
            continue;
        }

        has_legal_move = true;
        report.ply += 1;

        let mut child_pv = MoveList::new();
        let eval = -search(pos, depth - 1, -beta, -alpha, &mut child_pv, tt, report, stopper);

        report.ply -= 1;
        pos.undo_move(mv);

        if eval >= beta {
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
