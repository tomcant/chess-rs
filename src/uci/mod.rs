use self::{
    command::{UciCommand::*, handle},
    stopper::UciStopper,
};
use crate::position::Position;
use std::{
    io,
    sync::{Arc, Mutex, mpsc},
    thread,
};

pub mod command;
pub mod stopper;

mod r#move;
mod reporter;

pub fn main() {
    let (uci_tx, uci_rx) = mpsc::channel();
    let (stopper_tx, stopper_rx) = mpsc::channel();
    let stopper_rx = Arc::new(Mutex::new(stopper_rx));
    let pos = Arc::new(Mutex::new(Position::startpos()));

    thread::spawn(move || {
        loop {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).unwrap();
            let command = buffer.trim();

            if command.is_empty() {
                continue;
            }

            match command.parse() {
                Ok(parsed) => uci_tx.send(parsed).unwrap(),
                Err(err) => println!("error: {err}"),
            }
        }
    });

    loop {
        match uci_rx.recv().unwrap() {
            Init => handle::init(),
            IsReady => handle::is_ready(),
            NewGame => {
                let pos = Arc::clone(&pos);

                thread::spawn(move || {
                    handle::new_game(&mut pos.lock().unwrap());
                });
            }
            Position(fen, moves) => {
                let pos = Arc::clone(&pos);

                thread::spawn(move || {
                    handle::position(fen, moves, &mut pos.lock().unwrap());
                });
            }
            Go(params) => {
                let stopper_rx = Arc::clone(&stopper_rx);
                let pos = Arc::clone(&pos);

                thread::spawn(move || {
                    let rx_lock = stopper_rx.lock().unwrap();
                    let mut stopper = UciStopper::new(&rx_lock);

                    stopper.at_depth(params.depth);
                    stopper.at_elapsed(params.movetime);
                    stopper.at_nodes(params.nodes);
                    stopper.clear_stop_signal();

                    handle::go(&mut pos.lock().unwrap(), &stopper);
                });
            }
            SetOption(name, value) => handle::set_option(name, value),
            Stop => stopper_tx.send(true).unwrap(),
            Quit => break,
        }
    }
}
