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
use crate::position::Position;
use crate::uci::{uci_handle_command, UciCommand};
use std::{io, sync, thread};

fn main() -> io::Result<()> {
    println!("{}, {}", info_name(), info_author());

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

    let mut pos = Position::startpos();

    loop {
        let command = rx.recv().unwrap();

        if command == UciCommand::Quit {
            break;
        }

        uci_handle_command(&command, &mut pos);
    }

    Ok(())
}
