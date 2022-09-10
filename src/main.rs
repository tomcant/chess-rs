mod attacks;
mod board;
mod castling;
mod colour;
mod eval;
mod fen;
mod r#move;
mod movegen;
mod piece;
mod position;
mod search;
mod square;
mod uci;

use search::search;

fn main() {
    let fen = "rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 1";
    let mut pos = fen.parse().unwrap();

    if let Some(best_move) = search(&mut pos, 5) {
        println!("{best_move}");
    }
}
