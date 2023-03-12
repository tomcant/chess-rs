mod attacks;
mod colour;
mod eval;
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
