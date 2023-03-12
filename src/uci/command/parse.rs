use super::{
    GoParams,
    UciCommand::{self, *},
};
use crate::position::{Position, START_POS_FEN};
use std::str::FromStr;
use std::time::Duration;

impl FromStr for UciCommand {
    type Err = String;

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
            _ => Err(format!("unknown command '{}'", parts[0])),
        }
    }
}

fn parse_position(args: &[&str]) -> Result<UciCommand, String> {
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
                Token::Move => moves.push(arg.parse()?),
                _ => (),
            },
        }
    }

    fen.parse::<Position>()?;

    Ok(Position(fen.trim().to_string(), moves))
}

fn parse_go(args: &[&str]) -> Result<UciCommand, String> {
    let mut params = GoParams::new();
    let mut iter = args.iter();

    while let Some(attr) = iter.next() {
        if *attr == "infinite" {
            return Ok(Go(GoParams::new()));
        }

        let Some(value) = iter.next() else {
            return Err(format!("missing value for '{attr}' attribute"));
        };

        match *attr {
            "depth" => {
                let Ok(depth) = value.parse() else {
                    return Err("invalid value for 'depth' attribute".to_string());
                };
                params.depth = Some(depth);
            }
            "movetime" => {
                let Ok(movetime) = value.parse() else {
                    return Err("invalid value for 'movetime' attribute".to_string());
                };
                params.movetime = Some(Duration::from_millis(movetime));
            }
            "nodes" => {
                let Ok(nodes) = value.parse() else {
                    return Err("invalid value for 'nodes' attribute".to_string());
                };
                params.nodes = Some(nodes);
            }
            _ => return Err(format!("unknown attribute '{attr}'")),
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
    fn parse_init_command() {
        assert_eq!("uci".parse(), Ok(Init));
    }

    #[test]
    fn parse_isready_command() {
        assert_eq!("isready".parse(), Ok(IsReady));
    }

    #[test]
    fn parse_ucinewgame_command() {
        assert_eq!("ucinewgame".parse(), Ok(NewGame));
    }

    #[test]
    fn parse_position_command_with_start_pos() {
        assert_eq!(
            "position startpos".parse(),
            Ok(Position(START_POS_FEN.to_string(), vec![]))
        );
    }

    #[test]
    fn parse_position_command_with_fen() {
        assert_eq!(
            "position fen 4k3/8/8/8/8/8/8/4K3 w - - 0 1".parse(),
            Ok(Position("4k3/8/8/8/8/8/8/4K3 w - - 0 1".to_string(), vec![]))
        );
    }

    #[test]
    fn parse_position_command_with_moves() {
        assert_eq!(
            "position startpos moves e2e4".parse(),
            Ok(Position(
                START_POS_FEN.to_string(),
                vec![UciMove {
                    from: parse_square("e2"),
                    to: parse_square("e4"),
                    promotion_piece: None
                }]
            ))
        );

        assert_eq!(
            "position fen 4k3/8/8/8/8/8/8/4K3 w - - 0 1 moves d2d4 e7e6".parse(),
            Ok(Position(
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
            ))
        );

        assert_eq!(
            "position startpos moves e7e8q".parse(),
            Ok(Position(
                START_POS_FEN.to_string(),
                vec![UciMove {
                    from: parse_square("e7"),
                    to: parse_square("e8"),
                    promotion_piece: Some(Piece::WhiteQueen)
                }]
            ))
        );

        assert_eq!(
            "position startpos moves e2e1r".parse(),
            Ok(Position(
                START_POS_FEN.to_string(),
                vec![UciMove {
                    from: parse_square("e2"),
                    to: parse_square("e1"),
                    promotion_piece: Some(Piece::BlackRook)
                }]
            ))
        );
    }

    #[test]
    fn parse_go_command() {
        assert_eq!(
            "go depth 1 movetime 2 nodes 3".parse(),
            Ok(Go(GoParams {
                depth: Some(1),
                movetime: Some(Duration::from_millis(2)),
                nodes: Some(3),
            }))
        );
    }

    #[test]
    fn parse_go_command_with_infinite_attribute() {
        assert_eq!(
            "go infinite".parse(),
            Ok(Go(GoParams {
                depth: None,
                movetime: None,
                nodes: None,
            }))
        );
    }

    #[test]
    fn parse_stop_command() {
        assert_eq!("stop".parse(), Ok(Stop));
    }

    #[test]
    fn parse_quit_command() {
        assert_eq!("quit".parse(), Ok(Quit));
    }

    fn parse_square(str: &str) -> Square {
        let square = str.parse();
        assert!(square.is_ok());

        square.unwrap()
    }
}
