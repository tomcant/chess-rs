mod colour;
mod eval;
mod info;
mod movegen;
mod piece;
mod position;
mod search;
mod square;
mod uci;

#[cfg(test)]
mod testing;

fn main() {
    println!("{}, {}", info::name(), info::author());
    uci::main();
}
