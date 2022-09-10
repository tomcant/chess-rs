use crate::attacks::is_in_check;
use crate::eval::Evaluator;
use crate::movegen::MoveGenerator;
use crate::position::Position;
use crate::r#move::Move;

const EVAL_MIN: i32 = -9999;

pub fn search(pos: &mut Position, depth: u8) -> Option<Move> {
    let mut best_move = None;
    let mut best_eval = EVAL_MIN;

    for mv in pos.generate_moves() {
        pos.do_move(&mv);

        if !is_in_check(pos.colour_to_move.flip(), &pos.board) {
            let eval = -negamax(pos, depth - 1);

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

fn negamax(pos: &mut Position, depth: u8) -> i32 {
    if depth == 0 {
        return pos.evaluate();
    }

    let mut best_eval = EVAL_MIN;

    for mv in pos.generate_moves() {
        pos.do_move(&mv);

        if !is_in_check(pos.colour_to_move.flip(), &pos.board) {
            let eval = -negamax(pos, depth - 1);

            if eval > best_eval {
                best_eval = eval;
            }
        }

        pos.undo_move(&mv);
    }

    best_eval
}
