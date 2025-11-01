use super::{generate_all_moves, is_in_check};
use crate::position::Position;
use crate::uci::r#move::UciMove;

pub fn perft(pos: &mut Position, depth: u8) -> u32 {
    if depth == 0 {
        return 1;
    }

    let mut nodes = 0;

    for mv in generate_all_moves(pos) {
        pos.do_move(&mv);

        if !is_in_check(pos.opponent_colour(), &pos.board) {
            nodes += perft(pos, depth - 1);
        }

        pos.undo_move(&mv);
    }

    nodes
}

pub fn divide(pos: &mut Position, depth: u8) -> u32 {
    if depth == 0 {
        println!("\nTotal nodes: 1\n");
        return 1;
    }

    let mut nodes = 0;

    for mv in generate_all_moves(pos) {
        pos.do_move(&mv);

        if !is_in_check(pos.opponent_colour(), &pos.board) {
            let count = perft(pos, depth - 1);
            println!("{}: {}", UciMove::from(mv), count);
            nodes += count;
        }

        pos.undo_move(&mv);
    }

    println!("\nTotal nodes: {nodes}\n");

    nodes
}
