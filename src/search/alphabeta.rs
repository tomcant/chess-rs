use super::{
    tt::{Bound, Table},
    *,
};
use crate::movegen::{generate_all_moves, is_in_check};

#[allow(clippy::too_many_arguments)]
pub fn search(
    pos: &mut Position,
    depth: u8,
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
        return quiescence::search(pos, alpha, beta, &mut MoveList::new(), report);
    }

    let key = pos.key();
    let mut tt_move = None;
    let (_, mut pv_tail) = split_pv(pv);

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

    let colour_to_move = pos.colour_to_move;
    let mut has_legal_move = false;
    let mut tt_bound = Bound::Upper;

    let mut moves = generate_all_moves(pos);
    order_moves(&mut moves, &pos.board, tt_move);

    for mv in moves {
        pos.do_move(&mv);

        if is_in_check(colour_to_move, &pos.board) {
            pos.undo_move(&mv);
            continue;
        }

        has_legal_move = true;
        report.ply += 1;

        let eval = -search(pos, depth - 1, -beta, -alpha, &mut pv_tail, tt, report, stopper);

        report.ply -= 1;

        if eval >= beta {
            pos.undo_move(&mv);
            tt.store(key, depth, beta, Bound::Lower, Some(mv));
            return beta;
        }

        if eval > alpha {
            alpha = eval;

            tt_bound = Bound::Exact;
            tt_move = Some(mv);

            pv.clear();
            pv.push(mv);
            pv.append(&mut pv_tail);
        }

        pos.undo_move(&mv);
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
