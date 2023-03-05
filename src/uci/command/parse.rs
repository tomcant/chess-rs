use super::{
    GoParams,
    UciCommand::{self, *},
};
use crate::fen::START_POS_FEN;
use std::str::FromStr;
use std::time::Duration;

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
            "go" => Ok(parse_go(args)?),
            "stop" => Ok(Stop),
            "quit" => Ok(Quit),
            _ => Err(()),
        }
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

fn parse_go(args: &[&str]) -> Result<UciCommand, ()> {
    let mut params = GoParams::new();
    let mut iter = args.iter();

    while let Some(control) = iter.next() {
        if *control == "infinite" {
            return Ok(Go(GoParams::new()));
        }

        let Some(arg) = iter.next() else {
            return Err(());
        };

        match *control {
            "depth" => params.depth = arg.parse().ok(),
            "movetime" => params.movetime = arg.parse().map(Duration::from_millis).ok(),
            "nodes" => params.nodes = arg.parse().ok(),
            _ => return Err(()),
        }
    }

    Ok(Go(params))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::piece::Piece;
    use crate::square::Square;
    use crate::uci::r#move::UciMove;

    #[test]
    fn parse_position_command_with_start_pos() {
        assert_eq!(
            parse_command("position startpos"),
            Position(START_POS_FEN.to_string(), vec![])
        );
    }

    #[test]
    fn parse_position_command_with_fen() {
        assert_eq!(
            parse_command("position fen 4k3/8/8/8/8/8/8/4K3 w - - 0 1"),
            Position("4k3/8/8/8/8/8/8/4K3 w - - 0 1".to_string(), vec![])
        );
    }

    #[test]
    fn parse_position_command_with_moves() {
        assert_eq!(
            parse_command("position startpos moves e2e4"),
            Position(
                START_POS_FEN.to_string(),
                vec![UciMove {
                    from: parse_square("e2"),
                    to: parse_square("e4"),
                    promotion_piece: None
                }]
            )
        );

        assert_eq!(
            parse_command("position fen 4k3/8/8/8/8/8/8/4K3 w - - 0 1 moves d2d4 e7e6"),
            Position(
                "4k3/8/8/8/8/8/8/4K3 w - - 0 1".to_string(),
                vec![
                    UciMove {
                        from: parse_square("d2"),
                        to: parse_square("d4"),
                        promotion_piece: None
                    },
                    UciMove {
                        from: parse_square("e7"),
                        to: parse_square("e6"),
                        promotion_piece: None
                    }
                ]
            )
        );

        assert_eq!(
            parse_command("position startpos moves e7e8q"),
            Position(
                START_POS_FEN.to_string(),
                vec![UciMove {
                    from: parse_square("e7"),
                    to: parse_square("e8"),
                    promotion_piece: Some(Piece::WhiteQueen)
                }]
            )
        );

        assert_eq!(
            parse_command("position startpos moves e2e1r"),
            Position(
                START_POS_FEN.to_string(),
                vec![UciMove {
                    from: parse_square("e2"),
                    to: parse_square("e1"),
                    promotion_piece: Some(Piece::BlackRook)
                }]
            )
        );
    }

    #[test]
    fn parse_go_command() {
        assert_eq!(
            parse_command("go depth 1 movetime 2 nodes 3"),
            Go(GoParams {
                depth: Some(1),
                movetime: Some(Duration::from_millis(2)),
                nodes: Some(3),
            })
        );
    }

    #[test]
    fn parse_go_command_with_infinite_attribute() {
        assert_eq!(
            parse_command("go infinite"),
            Go(GoParams {
                depth: None,
                movetime: None,
                nodes: None,
            })
        );
    }

    fn parse_command(str: &str) -> UciCommand {
        let command = str.parse();
        assert!(command.is_ok());

        command.unwrap()
    }

    fn parse_square(str: &str) -> Square {
        let square = str.parse();
        assert!(square.is_ok());

        square.unwrap()
    }
}
