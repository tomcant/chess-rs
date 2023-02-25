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

use crate::info::{info_author, info_name};
use crate::uci::uci_main;

fn main() {
    println!("{}, {}", info_name(), info_author());
    uci_main();
}
