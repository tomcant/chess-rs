use self::UciCommand::*;
use crate::colour::Colour;
use crate::fen::START_POS_FEN;
use crate::info::{info_author, info_name};
use crate::piece::{Piece, PieceType};
use crate::position::Position;
use crate::r#move::Move;
use crate::search::{search, Report, Reporter, Stopper};
use crate::square::Square;
use std::str::FromStr;
use std::time::Duration;

const NANOS_PER_SEC: u128 = 1_000_000_000;

struct UciReporter {
    best_move: Option<Move>,
}

impl UciReporter {
    fn new() -> Self {
        Self { best_move: None }
    }
}

impl Reporter for UciReporter {
    fn send(&mut self, report: &Report) {
        let mut info = vec![
            format!("depth {}", report.depth),
            format!("nodes {}", report.nodes),
            format!("nps {}", report.nodes * NANOS_PER_SEC / report.elapsed().as_nanos()),
            format!("time {}", report.elapsed().as_millis()),
        ];

        if let Some((moves, score)) = &report.pv {
            info.push(format!(
                "score cp {} pv {}",
                score * 100,
                moves
                    .iter()
                    .map(|mv| format!("{mv}"))
                    .collect::<Vec<String>>()
                    .join(" ")
            ));

            self.best_move = Some(moves[0]);
        }

        println!("info {}", info.join(" "));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UciMove {
    pub from: Square,
    pub to: Square,
    pub promotion_piece: Option<Piece>,
}

impl FromStr for UciMove {
    type Err = ();

    fn from_str(mv: &str) -> Result<Self, Self::Err> {
        let from = mv[0..2].parse::<Square>()?;
        let to = mv[2..4].parse::<Square>()?;

        let promotion_piece = if mv.len() > 4 {
            Some(Piece::from(
                match mv.chars().nth(4).unwrap() {
                    'n' => PieceType::Knight,
                    'b' => PieceType::Bishop,
                    'r' => PieceType::Rook,
                    'q' => PieceType::Queen,
                    _ => return Err(()),
                },
                match to.rank() {
                    0 => Colour::Black,
                    _ => Colour::White,
                },
            ))
        } else {
            None
        };

        Ok(UciMove {
            from,
            to,
            promotion_piece,
        })
    }
}

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
                        promotion_piece: mv.promotion_piece,
                        castling_rights: pos.castling_rights,
                        is_en_passant: false,
                    });
                }
            }
        }
        UciCommand::Go(params) => {
            let mut reporter = UciReporter::new();

            let mut stopper = Stopper::new()
                .at_depth(params.depth)
                .at_elapsed(params.movetime)
                .at_nodes(params.nodes);

            search(pos, &mut reporter, &mut stopper);

            match reporter.best_move {
                Some(mv) => println!("bestmove {mv}"),
                None => println!("bestmove (none)"),
            }
        }
        UciCommand::IsReady => println!("readyok"),
        _ => (),
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
    let mut params = GoParams::new();
    let mut iter = args.iter();

    while let Some(control) = iter.next() {
        if *control == "infinite" {
            return Go(GoParams::new());
        }

        let arg = iter.next().unwrap();

        match *control {
            "depth" => params.depth = arg.parse().ok(),
            "movetime" => params.movetime = arg.parse().map(Duration::from_millis).ok(),
            "nodes" => params.nodes = arg.parse().ok(),
            _ => (),
        }
    }

    Go(params)
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
    fn handle_position_command_with_moves() {
        let command = parse_command("position startpos moves e2e4 e7e5");
        let mut pos = Position::startpos();

        uci_handle_command(&command, &mut pos);

        assert_eq!(pos.board.piece_at(parse_square("e4")), Some(Piece::WhitePawn));
        assert_eq!(pos.board.piece_at(parse_square("e5")), Some(Piece::BlackPawn));
    }

    #[test]
    fn handle_position_command_with_promotion_moves() {
        let fen = "8/1P2k3/8/8/8/8/4K1p1/8 w - - 0 1";
        let command = parse_command(&format!("position fen {fen} moves b7b8q g2g1r"));
        let mut pos = fen.parse().unwrap();

        uci_handle_command(&command, &mut pos);

        assert_eq!(pos.board.piece_at(parse_square("b8")), Some(Piece::WhiteQueen));
        assert_eq!(pos.board.piece_at(parse_square("g1")), Some(Piece::BlackRook));
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

    fn parse_command(command: &str) -> UciCommand {
        command.parse().unwrap()
    }

    fn parse_square(str: &str) -> Square {
        let square = str.parse();
        assert!(square.is_ok());

        square.unwrap()
    }
}
