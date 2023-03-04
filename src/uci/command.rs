use super::r#move::UciMove;
use std::time::Duration;

#[derive(Debug, PartialEq, Eq)]
pub enum UciCommand {
    Init,
    IsReady,
    NewGame,
    Position(String, Vec<UciMove>),
    Go(GoParams),
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
    pub fn new() -> Self {
        Self {
            depth: None,
            movetime: None,
            nodes: None,
        }
    }
}

mod parse {
    use super::*;
    use crate::fen::START_POS_FEN;
    use std::str::FromStr;

    impl FromStr for UciCommand {
        type Err = ();

        fn from_str(command: &str) -> Result<Self, Self::Err> {
            let parts: Vec<_> = command.split_whitespace().collect();
            let args = &parts[1..];

            match parts[0] {
                "uci" => Ok(UciCommand::Init),
                "isready" => Ok(UciCommand::IsReady),
                "ucinewgame" => Ok(UciCommand::NewGame),
                "position" => Ok(parse_position(args)?),
                "go" => Ok(parse_go(args)?),
                "stop" => Ok(UciCommand::Stop),
                "quit" => Ok(UciCommand::Quit),
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

        Ok(UciCommand::Position(fen.trim().to_string(), moves))
    }

    fn parse_go(args: &[&str]) -> Result<UciCommand, ()> {
        let mut params = GoParams::new();
        let mut iter = args.iter();

        while let Some(control) = iter.next() {
            if *control == "infinite" {
                return Ok(UciCommand::Go(GoParams::new()));
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

        Ok(UciCommand::Go(params))
    }
}

pub mod handle {
    use super::*;
    use crate::info;
    use crate::position::Position;
    use crate::r#move::Move;
    use crate::search::search;
    use crate::uci::{reporter::UciReporter, stopper::UciStopper};

    pub fn init() {
        println!("id name {}", info::name());
        println!("id author {}", info::author());
        println!("uciok");
    }

    pub fn is_ready() {
        println!("readyok");
    }

    pub fn new_game(pos: &mut Position) {
        *pos = Position::startpos();
    }

    pub fn position(fen: String, moves: Vec<UciMove>, pos: &mut Position) {
        let Ok(parsed) = fen.parse() else {
            return;
        };

        *pos = parsed;

        for mv in moves {
            pos.do_move(&Move {
                from: mv.from,
                to: mv.to,
                captured_piece: pos.board.piece_at(mv.to),
                promotion_piece: mv.promotion_piece,
                castling_rights: pos.castling_rights,
                is_en_passant: false,
            });
        }
    }

    pub fn go(pos: &mut Position, stopper: &UciStopper) {
        let reporter = UciReporter::new();
        search(pos, &reporter, stopper);

        match reporter.best_move() {
            Some(mv) => println!("bestmove {mv}"),
            None => println!("bestmove (none)"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fen::START_POS_FEN;
    use crate::piece::Piece;
    use crate::position::Position;
    use crate::square::Square;

    #[test]
    fn parse_position_command_with_start_pos() {
        assert_eq!(
            parse_command("position startpos"),
            UciCommand::Position(START_POS_FEN.to_string(), vec![])
        );
    }

    #[test]
    fn parse_position_command_with_fen() {
        assert_eq!(
            parse_command("position fen 4k3/8/8/8/8/8/8/4K3 w - - 0 1"),
            UciCommand::Position("4k3/8/8/8/8/8/8/4K3 w - - 0 1".to_string(), vec![])
        );
    }

    #[test]
    fn parse_position_command_with_moves() {
        assert_eq!(
            parse_command("position startpos moves e2e4"),
            UciCommand::Position(
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
            UciCommand::Position(
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
            UciCommand::Position(
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
            UciCommand::Position(
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
    fn handle_position_command_with_moves() {
        let command = "position startpos moves e2e4 e7e5";
        let UciCommand::Position(fen, moves) = parse_command(command) else {
            panic!("Could not parse position command")
        };
        let mut pos = Position::startpos();

        handle::position(fen, moves, &mut pos);

        assert_eq!(pos.board.piece_at(parse_square("e4")), Some(Piece::WhitePawn));
        assert_eq!(pos.board.piece_at(parse_square("e5")), Some(Piece::BlackPawn));
    }

    #[test]
    fn handle_position_command_with_promotion_moves() {
        let fen = "8/1P2k3/8/8/8/8/4K1p1/8 w - - 0 1";
        let command = format!("position fen {fen} moves b7b8q g2g1r");
        let UciCommand::Position(fen, moves) = parse_command(&command) else {
            panic!("Could not parse position command")
        };
        let Ok(mut pos) = fen.parse() else { panic!("Could not parse FEN") };

        handle::position(fen, moves, &mut pos);

        assert_eq!(pos.board.piece_at(parse_square("b8")), Some(Piece::WhiteQueen));
        assert_eq!(pos.board.piece_at(parse_square("g1")), Some(Piece::BlackRook));
    }

    #[test]
    fn parse_go_command() {
        assert_eq!(
            parse_command("go depth 1 movetime 2 nodes 3"),
            UciCommand::Go(GoParams {
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
            UciCommand::Go(GoParams {
                depth: None,
                movetime: None,
                nodes: None,
            })
        );
    }

    fn parse_command(command: &str) -> UciCommand {
        command.parse().unwrap()
    }

    fn parse_square(str: &str) -> Square {
        let square = str.parse();
        assert!(square.is_ok());

        square.unwrap()
    }
}
