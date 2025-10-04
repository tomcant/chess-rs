use super::r#move::UciMove;
use std::time::Duration;

pub mod handle;
mod parse;

#[derive(Debug, PartialEq, Eq)]
pub enum UciCommand {
    Init,
    IsReady,
    NewGame,
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
    pub nodes: Option<u128>,
}

impl GoParams {
    fn new() -> Self {
        Self {
            depth: None,
            movetime: None,
            nodes: None,
        }
    }
}
