mod attacks;
mod board;
mod castling;
mod colour;
mod eval;
mod fen;
mod game;
mod r#move;
mod movegen;
mod piece;
mod search;
mod square;

use search::think;

fn main() {
    let fen = "rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 1";
    let mut state = fen.parse().unwrap();

    if let Some(best_move) = think(&mut state, 4) {
        println!("best move: {best_move}");
    }
}
