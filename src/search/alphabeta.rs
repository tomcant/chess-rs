use super::*;
use crate::attacks::is_in_check;
use crate::movegen::generate_all_moves;

pub fn search(
    pos: &mut Position,
    depth: u8,
    mut alpha: i32,
    beta: i32,
    pv: &mut Vec<Move>,
    report: &mut Report,
    stopper: &impl Stopper,
) -> i32 {
    if stopper.should_stop(report) {
        return 0;
    }

    if depth == 0 {
        return quiescence::search(pos, alpha, beta, &mut vec![], report);
    }

    report.nodes += 1;

    let (pv_move, mut next_ply_pv) = split_pv(pv);
    let colour_to_move = pos.colour_to_move;
    let mut has_legal_move = false;

    for mv in order_moves(&generate_all_moves(pos), pv_move) {
        pos.do_move(&mv);

        if is_in_check(colour_to_move, &pos.board) {
            pos.undo_move(&mv);
            continue;
        }

        has_legal_move = true;
        report.ply += 1;

        let eval = -search(pos, depth - 1, -beta, -alpha, &mut next_ply_pv, report, stopper);

        report.ply -= 1;

        if eval >= beta {
            pos.undo_move(&mv);
            return beta;
        }

        if eval > alpha {
            alpha = eval;

            pv.clear();
            pv.push(*mv);
            pv.append(&mut next_ply_pv);
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

    alpha
}
