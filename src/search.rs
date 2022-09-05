use crate::eval::Evaluator;
use crate::game::GameState;
use crate::movegen::MoveGenerator;
use crate::r#move::Move;

const EVAL_MIN: i32 = -1024;

pub fn think(state: &mut GameState, depth: u8) -> Option<Move> {
    let mut best_move = None;
    let mut best_eval = EVAL_MIN;

    for mv in state.generate_moves() {
        state.do_move(&mv);

        let eval = -search(state, depth - 1);

        if eval > best_eval {
            best_move = Some(mv);
            best_eval = eval;

            println!("current best move: {mv}");
        }

        state.undo_move(&mv);
    }

    best_move
}

fn search(state: &mut GameState, depth: u8) -> i32 {
    if depth == 0 {
        return state.evaluate();
    }

    let mut best_eval = EVAL_MIN;

    for mv in state.generate_moves() {
        state.do_move(&mv);

        let eval = -search(state, depth - 1);

        if eval > best_eval {
            best_eval = eval;
        }

        state.undo_move(&mv);
    }

    best_eval
}
