use super::r#move::UciMove;
use std::time::Duration;

pub mod handle;
mod parse;

#[derive(Debug, PartialEq, Eq)]
pub enum UciCommand {
    Init,
    IsReady,
    NewGame,
    PrintBoard,
    Position(String, Vec<UciMove>),
    Go(GoParams),
    SetOption(String, Option<String>),
    Stop,
    Quit,
}

#[derive(Debug, PartialEq, Eq)]
pub struct GoParams {
    pub depth: Option<u8>,
    pub movetime: Option<Duration>,
    pub wtime: Option<Duration>,
    pub btime: Option<Duration>,
    pub winc: Option<Duration>,
    pub binc: Option<Duration>,
    pub nodes: Option<u128>,
}

impl GoParams {
    fn new() -> Self {
        Self {
            depth: None,
            movetime: None,
            wtime: None,
            btime: None,
            winc: None,
            binc: None,
            nodes: None,
        }
    }
}
