use crate::info;
use crate::movegen::Move;
use crate::position::Position;
use crate::search::search;
use crate::uci::{r#move::UciMove, reporter::UciReporter, stopper::UciStopper};

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
            half_move_clock: pos.half_move_clock,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::piece::Piece;
    use crate::square::Square;
    use crate::uci::command::UciCommand::{self, *};

    #[test]
    fn handle_position_command_with_moves() {
        let command = "position startpos moves e2e4 e7e5";
        let Position(fen, moves) = parse_command(command) else { panic!() };
        let Ok(mut pos) = fen.parse() else { panic!() };

        position(fen, moves, &mut pos);

        assert_eq!(pos.board.piece_at(parse_square("e4")), Some(Piece::WP));
        assert_eq!(pos.board.piece_at(parse_square("e5")), Some(Piece::BP));
    }

    #[test]
    fn handle_position_command_with_promotion_moves() {
        let command = format!("position fen 8/1P2k3/8/8/8/8/4K1p1/8 w - - 0 1 moves b7b8q g2g1r");
        let Position(fen, moves) = parse_command(&command) else { panic!() };
        let Ok(mut pos) = fen.parse() else { panic!() };

        position(fen, moves, &mut pos);

        assert_eq!(pos.board.piece_at(Square::B8), Some(Piece::WQ));
        assert_eq!(pos.board.piece_at(Square::G1), Some(Piece::BR));
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
