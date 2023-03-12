use super::*;
use crate::movegen::{is_in_check, generate_capture_moves};

pub fn search(pos: &mut Position, mut alpha: i32, beta: i32, pv: &mut Vec<Move>, report: &mut Report) -> i32 {
    report.nodes += 1;

    let eval = eval(pos);

    if eval >= beta {
        return beta;
    }

    if eval > alpha {
        alpha = eval;
    }

    let (pv_move, mut next_ply_pv) = split_pv(pv);
    let colour_to_move = pos.colour_to_move;

    for mv in order_moves(&generate_capture_moves(pos), pv_move) {
        pos.do_move(&mv);

        if is_in_check(colour_to_move, &pos.board) {
            pos.undo_move(&mv);
            continue;
        }

        let eval = -search(pos, -beta, -alpha, &mut next_ply_pv, report);

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

    alpha
}
