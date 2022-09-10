use self::UciCommand::*;
use crate::fen::START_POS_FEN;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct SearchParams {
    pub depth: u8,
}

#[derive(Debug, PartialEq, Eq)]
pub enum UciCommand {
    Init,
    IsReady,
    NewGame,
    Position(String, Vec<String>),
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
            "position" => Ok(parse_position(args)),
            "go" => Ok(parse_go(args)),
            "stop" => Ok(Stop),
            "quit" => Ok(Quit),
            _ => Err(()),
        }
    }
}

fn parse_position(args: &[&str]) -> UciCommand {
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
                Token::Move => moves.push(arg.to_string()),
                _ => (),
            },
        }
    }

    Position(fen.trim().to_string(), moves)
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
        let fen = "4k3/8/8/8/8/8/8/4K3 w - - 0 1".to_string();
        let moves = vec!["e2e4".to_string(), "e7e5".to_string()];

        assert_eq!(
            parse_command(&format!("position startpos moves {}", moves.join(" "))),
            Position(START_POS_FEN.to_string(), moves.clone())
        );

        assert_eq!(
            parse_command(&format!("position fen {fen} moves {}", moves.join(" "))),
            Position(fen, moves.clone())
        );
    }

    fn parse_command(command: &str) -> UciCommand {
        command.parse().unwrap()
    }
}
