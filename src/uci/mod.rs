use crate::position::Position;
use crate::uci::{
    command::{handle, UciCommand},
    stopper::UciStopper,
};

use std::{
    io,
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub mod command;
pub mod stopper;

mod r#move;
mod reporter;

pub fn uci_main() {
    let (uci_tx, uci_rx) = mpsc::channel();
    let (stopper_tx, stopper_rx) = mpsc::channel();
    let stopper_rx = Arc::new(Mutex::new(stopper_rx));
    let pos = Arc::new(Mutex::new(Position::startpos()));

    thread::spawn(move || loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        let command = buffer.trim();

        if command.is_empty() {
            continue;
        }

        match command.parse() {
            Ok(parsed) => uci_tx.send(parsed).unwrap(),
            Err(_) => println!("unknown command '{command}'"),
        }
    });

    loop {
        match uci_rx.recv().unwrap() {
            UciCommand::Init => handle::init(),
            UciCommand::IsReady => handle::is_ready(),
            UciCommand::NewGame => handle::new_game(&mut pos.lock().unwrap()),
            UciCommand::Position(fen, moves) => handle::position(fen, moves, &mut pos.lock().unwrap()),
            UciCommand::Go(params) => {
                let stopper_rx = Arc::clone(&stopper_rx);
                let pos = Arc::clone(&pos);

                thread::spawn(move || {
                    let rx_lock = stopper_rx.lock().unwrap();
                    let mut stopper = UciStopper::new(&rx_lock)
                        .at_depth(params.depth)
                        .at_elapsed(params.movetime)
                        .at_nodes(params.nodes);

                    stopper.clear_stop_signal();
                    handle::go(&mut pos.lock().unwrap(), &mut stopper);
                });
            }
            UciCommand::Stop => stopper_tx.send(true).unwrap(),
            UciCommand::Quit => break,
        }
    }
}