use crate::attacks::is_in_check;
use crate::eval::Evaluator;
use crate::movegen::MoveGenerator;
use crate::position::Position;
use crate::r#move::Move;

const EVAL_MAX: i32 = 10_000;
const EVAL_MIN: i32 = -EVAL_MAX;
const EVAL_STALEMATE: i32 = 0;

pub fn search(pos: &mut Position, depth: u8) -> Option<Move> {
    let mut best_move = None;
    let mut best_eval = EVAL_MIN;

    for mv in pos.generate_moves() {
        pos.do_move(&mv);

        if !is_in_check(pos.opponent_colour(), &pos.board) {
            if best_move.is_none() {
                best_move = Some(mv);
            }

            let eval = -alpha_beta(pos, depth - 1, EVAL_MIN, EVAL_MAX);

            if eval > best_eval {
                best_move = Some(mv);
                best_eval = eval;

                println!("{mv} eval = {eval}");
            }
        }

        pos.undo_move(&mv);
    }

    best_move
}

fn alpha_beta(pos: &mut Position, depth: u8, mut alpha: i32, beta: i32) -> i32 {
    if depth == 0 {
        return pos.evaluate();
    }

    let mut has_legal_move = false;

    // todo: sort moves

    for mv in pos.generate_moves() {
        pos.do_move(&mv);

        if !is_in_check(pos.opponent_colour(), &pos.board) {
            has_legal_move = true;

            let eval = -alpha_beta(pos, depth - 1, -beta, -alpha);

            if eval >= beta {
                alpha = beta;
                pos.undo_move(&mv);
                break;
            }

            if eval > alpha {
                alpha = eval;
            }
        }

        pos.undo_move(&mv);
    }

    if !has_legal_move && !is_in_check(pos.colour_to_move, &pos.board) {
        return EVAL_STALEMATE;
    }

    alpha - depth as i32
}
