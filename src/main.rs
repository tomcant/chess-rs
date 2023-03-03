mod attacks;
mod board;
mod castling;
mod colour;
mod eval;
mod fen;
mod info;
mod r#move;
mod movegen;
mod piece;
mod position;
mod search;
mod square;
mod uci;

fn main() {
    println!("{}, {}", info::name(), info::author());
    uci::main();
}
