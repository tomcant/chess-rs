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
    println!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    let (tx, rx) = sync::mpsc::channel();

    thread::spawn(move || loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        let command = buffer.trim();

        if command.is_empty() {
            continue;
        }

        match command.parse::<UciCommand>() {
            Ok(parsed) => tx.send(parsed).unwrap(),
            Err(_) => println!("unknown command '{command}'"),
        }
    });

    let mut pos = START_POS_FEN.parse().unwrap();

    loop {
        let command = rx.recv().unwrap();

        match command {
            UciCommand::Init => {
                println!("id name chess-rs");
                println!("id author Tom Cant");
                println!("uciok");
            }
            UciCommand::Position(fen, _moves) => {
                pos = fen.parse().unwrap();
                // todo: apply moves to position
            }
            UciCommand::Go(params) => {
                let mv = search(&mut pos, params.depth).unwrap();
                println!("bestmove {mv}");
            }
            UciCommand::IsReady => println!("readyok"),
            UciCommand::Quit => break,
            _ => println!("unhandled command: {command:?}"),
        }
    }

    Ok(())
}
