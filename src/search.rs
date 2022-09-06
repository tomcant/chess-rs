use crate::attacks::is_in_check;
use crate::eval::Evaluator;
use crate::game::GameState;
use crate::movegen::MoveGenerator;
use crate::r#move::Move;

const EVAL_MIN: i32 = -9999;

pub fn search(state: &mut GameState, depth: u8) -> Option<Move> {
    let mut best_move = None;
    let mut best_eval = EVAL_MIN;

    for mv in state.generate_moves() {
        state.do_move(&mv);

        if !is_in_check(state.colour_to_move.flip(), &state.board) {
            let eval = -negamax(state, depth - 1);

            if eval > best_eval {
                best_move = Some(mv);
                best_eval = eval;

                println!("{mv} eval = {eval}");
            }
        }

        state.undo_move(&mv);
    }

    best_move
}

fn negamax(state: &mut GameState, depth: u8) -> i32 {
    if depth == 0 {
        return state.evaluate();
    }

    let mut best_eval = EVAL_MIN;

    for mv in state.generate_moves() {
        state.do_move(&mv);

        if !is_in_check(state.colour_to_move.flip(), &state.board) {
            let eval = -negamax(state, depth - 1);

            if eval > best_eval {
                best_eval = eval;
            }
        }

        state.undo_move(&mv);
    }

    best_eval
}
