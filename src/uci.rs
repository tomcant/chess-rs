use self::UciCommand::*;
use crate::fen::START_POS_FEN;
use crate::info::{info_author, info_name};
use crate::position::Position;
use crate::r#move::Move;
use crate::search::{search, Report};
use crate::square::Square;
use std::str::FromStr;
use std::time::Duration;

#[derive(Debug)]
struct UciReport {
    pub best_move: Option<Move>,
    pub elapsed_ms: u128,
}

impl UciReport {
    pub fn new() -> Self {
        Self {
            best_move: None,
            elapsed_ms: 0,
        }
    }
}

impl Report for UciReport {
    fn principal_variation(&mut self, moves: Vec<Move>, eval: i32) {
        self.best_move = Some(moves[0]);

        println!(
            "info depth {} score cp {} time {} pv {}",
            moves.len(),
            eval * 100,
            self.elapsed_ms,
            moves
                .iter()
                .map(|mv| format!("{mv}"))
                .collect::<Vec<String>>()
                .join(" "),
        );
    }

    fn elapsed_time(&mut self, time: Duration) {
        self.elapsed_ms = time.as_millis();
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct SearchParams {
    pub depth: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UciMove {
    pub from: Square,
    pub to: Square,
}

impl FromStr for UciMove {
    type Err = ();

    fn from_str(mv: &str) -> Result<Self, Self::Err> {
        let from = mv[0..2].parse()?;
        let to = mv[2..4].parse()?;

        Ok(UciMove { from, to })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum UciCommand {
    Init,
    IsReady,
    NewGame,
    Position(String, Vec<UciMove>),
    Go(SearchParams),
    Stop,
    Quit,
}

impl FromStr for UciCommand {
    type Err = ();

    fn from_str(command: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = command.split_whitespace().collect();
        let args = &parts[1..];

        match parts[0] {
            "uci" => Ok(Init),
            "isready" => Ok(IsReady),
            "ucinewgame" => Ok(NewGame),
            "position" => Ok(parse_position(args)?),
            "go" => Ok(parse_go(args)),
            "stop" => Ok(Stop),
            "quit" => Ok(Quit),
            _ => Err(()),
        }
    }
}

pub fn uci_handle_command(command: &UciCommand, pos: &mut Position) {
    match command {
        UciCommand::Init => {
            println!("id name {}", info_name());
            println!("id author {}", info_author());
            println!("uciok");
        }
        UciCommand::NewGame => {
            *pos = Position::startpos();
        }
        UciCommand::Position(fen, moves) => {
            if let Ok(parsed) = fen.parse() {
                *pos = parsed;

                for mv in moves {
                    pos.do_move(&Move {
                        from: mv.from,
                        to: mv.to,
                        captured_piece: pos.board.piece_at(mv.to),
                        promotion_piece: None,
                        castling_rights: pos.castling_rights,
                        is_en_passant: false,
                    });
                }
            }
        }
        UciCommand::Go(params) => {
            let mut report = UciReport::new();
            search(pos, params.depth, &mut report);

            match report.best_move {
                Some(mv) => println!("bestmove {mv}"),
                None => println!("bestmove (none)"),
            }
        }
        UciCommand::IsReady => println!("readyok"),
        UciCommand::Stop | UciCommand::Quit => unimplemented!(),
    }
}

fn parse_position(args: &[&str]) -> Result<UciCommand, ()> {
    enum Token {
        None,
        Fen,
        Move,
    }

    let mut token = Token::None;
    let mut fen = String::from("");
    let mut moves = vec![];

    for arg in args {
        match *arg {
            "fen" => token = Token::Fen,
            "moves" => token = Token::Move,
            "startpos" => fen = START_POS_FEN.to_string(),

            _ => match token {
                Token::Fen => fen = format!("{fen} {arg}"),
                Token::Move => moves.push(arg.parse().unwrap()),
                _ => (),
            },
        }
    }

    if fen.is_empty() {
        return Err(());
    }

    Ok(Position(fen.trim().to_string(), moves))
}

fn parse_go(args: &[&str]) -> UciCommand {
    let mut params = SearchParams { depth: 1 };
    let mut iter = args.iter();

    while let Some(arg) = iter.next() {
        match *arg {
            "depth" => params.depth = iter.next().unwrap().parse().unwrap(),
            _ => (),
        }
    }

    Go(params)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_position_command_with_start_pos() {
        assert_eq!(
            parse_command("position startpos"),
            Position(START_POS_FEN.to_string(), vec![])
        );
    }

    #[test]
    fn parse_position_command_with_fen() {
        let fen = "4k3/8/8/8/8/8/8/4K3 w - - 0 1".to_string();

        assert_eq!(parse_command(&format!("position fen {fen}")), Position(fen, vec![]));
    }

    #[test]
    fn parse_position_command_with_moves() {
        let uci_moves = vec!["e2e4".parse().unwrap(), "e7e5".parse().unwrap()];

        assert_eq!(
            parse_command("position startpos moves e2e4 e7e5"),
            Position(START_POS_FEN.to_string(), uci_moves.clone())
        );

        let fen = "4k3/8/8/8/8/8/8/4K3 w - - 0 1";

        assert_eq!(
            parse_command(&format!("position fen {fen} moves e2e4 e7e5")),
            Position(fen.to_string(), uci_moves.clone())
        );
    }

    fn parse_command(command: &str) -> UciCommand {
        command.parse().unwrap()
    }
}
