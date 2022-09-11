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

use crate::fen::START_POS_FEN;
use crate::search::search;
use crate::uci::UciCommand;
use std::{io, sync, thread};

fn main() -> io::Result<()> {
    let name = format!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    let author = env!("CARGO_PKG_AUTHORS");

    println!("{name}");

    let (tx, rx) = sync::mpsc::channel::<UciCommand>();

    thread::spawn(move || loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        let command = buffer.trim();

        if command.is_empty() {
            continue;
        }

        match command.parse() {
            Ok(parsed) => tx.send(parsed).unwrap(),
            Err(_) => println!("unknown command '{command}'"),
        }
    });

    let mut pos = START_POS_FEN.parse().unwrap();

    loop {
        let command = rx.recv().unwrap();

        match command {
            UciCommand::Init => {
                println!("id name {name}");
                println!("id author {author}");
                println!("uciok");
            }
            UciCommand::Position(fen, _moves) => {
                if let Ok(parsed) = fen.parse() {
                    pos = parsed;
                }
                // todo: apply moves to position
            }
            UciCommand::Go(params) => {
                if let Some(mv) = search(&mut pos, params.depth) {
                    println!("bestmove {mv}");
                } else {
                    println!("bestmove (none)");
                }
            }
            UciCommand::IsReady => println!("readyok"),
            UciCommand::Quit => break,
            _ => println!("unhandled command: {command:?}"),
        }
    }

    Ok(())
}
