use super::{
    movepicker::{MovePicker, MovePickerMode},
    tt::Bound,
    *,
};
use crate::colour::Colour;
use crate::movegen::is_in_check;
use crate::piece::Piece;
use crate::position::Board;

#[rustfmt::skip]
#[allow(clippy::too_many_arguments)]
pub fn search(
    pos: &mut Position,
    mut depth: u8,
    mut alpha: i32,
    beta: i32,
    pv: &mut MoveList,
    tt: &mut TranspositionTable,
    killers: &mut KillerMoves,
    report: &mut Report,
    stopper: &Stopper,
) -> i32 {
    if stopper.should_stop(report) {
        return 0;
    }

    if pos.is_fifty_move_draw() || pos.is_repetition_draw(report.ply) {
        return EVAL_DRAW;
    }

    if depth == 0 {
        if !is_in_check(pos.colour_to_move, &pos.board) {
            return quiescence::search(pos, alpha, beta, report);
        }

        // Extend the search if we're in check so that quiescence doesn't need
        // to consider possible evasions and can remain focused on captures.
        depth = 1;
    }

    let mut tt_move = None;

    if let Some(entry) = tt.probe(pos.key) {
        if entry.depth >= depth {
            let eval = tt::eval_out(entry.eval, report.ply);

            match entry.bound {
                Bound::Exact => return eval,
                Bound::Lower if eval >= beta => return beta,
                Bound::Upper if eval <= alpha => return alpha,
                _ => (),
            };
        }

        tt_move = entry.mv;
    }

    report.nodes += 1;

    let colour_to_move = pos.colour_to_move;
    let in_check = is_in_check(colour_to_move, &pos.board);

    // Static eval used for futility pruning heuristics at non-PV nodes. This is
    // intentionally restricted to a PVS null-window so we don't prune PV nodes
    // where we need accurate scores.
    let futility_base_eval = if !in_check
        && report.ply > 0
        && depth <= 5
        && beta - alpha == 1 // PVS null-window
        && alpha > -EVAL_MATE_THRESHOLD
        && beta < EVAL_MATE_THRESHOLD
    {
        let eval = eval(pos);

        // Reverse futility pruning: if the static eval is already well above
        // beta at shallow depths, assume this node will fail high.
        let safe_to_prune = match tt_move {
            Some(mv) => mv.captured_piece.is_none(),
            None => true
        };
        if safe_to_prune && eval - depth as i32 * 100 >= beta {
            tt.store(pos.key, depth, tt::eval_in(beta, report.ply), Bound::Lower, tt_move);
            return beta;
        }

        if depth <= 3 {
            Some(eval)
        } else {
            None
        }
    } else {
        None
    };

    // Null-move pruning: if not in check and with sufficient depth/material, try
    // a null move to quickly detect beta cutoffs.
    if depth >= 3 && !in_check && has_non_pawn_material(&pos.board, colour_to_move) {
        pos.do_null_move();
        report.ply += 1;

        let r = if depth > 6 { 3 } else { 2 };
        let null_eval = -search(pos, depth - r - 1, -beta, -beta + 1, &mut MoveList::new(), tt, killers, report, stopper);

        report.ply -= 1;
        pos.undo_null_move();

        if null_eval >= beta {
            tt.store(pos.key, depth, tt::eval_in(null_eval, report.ply), Bound::Lower, None);
            return beta;
        }
    }

    let mut has_searched_one = false;
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
            tt.store(pos.key, depth, tt::eval_in(eval, report.ply), Bound::Lower, tt_move);
            return beta;
        }

        if eval > alpha {
            alpha = eval;
            tt_bound = Bound::Exact;

            pv.clear();
            pv.push(mv);
            pv.append(&mut child_pv);
        }

        has_searched_one = true;
        has_legal_move = true;
    }

    let mut move_picker = MovePicker::new(pos, MovePickerMode::AllMoves { killers, ply: report.ply });

    while let Some(mv) = move_picker.pick() {
        if tt_move.is_some() && mv.equals(&tt_move.unwrap()) {
            continue;
        }

        pos.do_move(&mv);

        if is_in_check(colour_to_move, &pos.board) {
            pos.undo_move(&mv);
            continue;
        }

        has_legal_move = true;

        // Futility pruning: if the static eval plus a margin is not enough to
        // improve alpha and the move is a quiet non-promotion then prune this
        // move. This helps skip hopeless quiet moves near leaf nodes.
        if let Some(eval) = futility_base_eval
            && mv.captured_piece.is_none()
            && mv.promotion_piece.is_none()
            && !is_in_check(pos.colour_to_move, &pos.board)
            && eval + depth as i32 * 100 <= alpha
        {
            pos.undo_move(&mv);
            continue;
        }

        report.ply += 1;

        // Principal variation search: after one move has been searched with the
        // full window (assumed best due to ordering), try the remaining moves
        // with a narrow window. This is cheaper because it's not searching for
        // the exact eval, just to show that it fails high or low. Only in the
        // rare case the eval lies between alpha and beta do we pay the cost of
        // a full window re-search to obtain the exact eval and PV.
        let mut eval;
        let mut child_pv = MoveList::new();

        if has_searched_one {
            eval = -search(pos, depth - 1, -alpha - 1, -alpha, &mut MoveList::new(), tt, killers, report, stopper);

            if eval > alpha && eval < beta {
                eval = -search(pos, depth - 1, -beta, -alpha, &mut child_pv, tt, killers, report, stopper);
            }
        } else {
            eval = -search(pos, depth - 1, -beta, -alpha, &mut child_pv, tt, killers, report, stopper);
        }

        report.ply -= 1;
        pos.undo_move(&mv);

        if eval >= beta {
            if mv.captured_piece.is_none() && mv.promotion_piece.is_none() {
                killers.store(report.ply, &mv);
            }

            tt.store(pos.key, depth, tt::eval_in(eval, report.ply), Bound::Lower, Some(mv));
            return beta;
        }

        if eval > alpha {
            alpha = eval;
            tt_bound = Bound::Exact;
            tt_move = Some(mv);

            pv.clear();
            pv.push(mv);
            pv.append(&mut child_pv);
        }

        has_searched_one = true;
    }

    if !has_legal_move {
        return if in_check { -EVAL_MATE + report.ply as i32 } else { EVAL_DRAW };
    }

    tt.store(pos.key, depth, tt::eval_in(alpha, report.ply), tt_bound, tt_move);

    alpha
}

fn has_non_pawn_material(board: &Board, colour: Colour) -> bool {
    let knights = board.count_pieces(Piece::knight(colour));
    let bishops = board.count_pieces(Piece::bishop(colour));
    let rooks = board.count_pieces(Piece::rook(colour));
    let queens = board.count_pieces(Piece::queen(colour));

    (knights + bishops + rooks + queens) > 0
}
