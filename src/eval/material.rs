use crate::colour::Colour;
use crate::piece::Piece;
use crate::position::Board;

const PIECE_WEIGHTS: [i32; 12] = [100, 300, 350, 500, 900, 0, 100, 300, 350, 500, 900, 0];

pub fn eval(colour: Colour, board: &Board) -> i32 {
    Piece::pieces_by_colour(colour).iter().fold(0, |acc, piece| {
        acc + PIECE_WEIGHTS[piece.index()] * board.count_pieces(*piece) as i32
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::position::Position;

    #[test]
    fn more_material_is_good() {
        let more_white_material = parse_fen("4kbnr/8/8/8/8/8/4P3/4KBNR w - - 0 1");

        assert!(eval(Colour::White, &more_white_material.board) > eval(Colour::Black, &more_white_material.board));
    }

    #[test]
    fn minor_pieces_are_worth_more_than_pawns() {
        let white_knight_black_pawn = parse_fen("8/4p3/8/8/8/8/8/6N1 w - - 0 1");
        let black_bishop_white_pawn = parse_fen("5b2/8/8/8/8/8/4P3/8 w - - 0 1");

        assert!(
            eval(Colour::White, &white_knight_black_pawn.board) > eval(Colour::Black, &white_knight_black_pawn.board)
        );
        assert!(
            eval(Colour::Black, &black_bishop_white_pawn.board) > eval(Colour::White, &black_bishop_white_pawn.board)
        );
    }

    #[test]
    fn rooks_are_worth_more_than_bishops() {
        let pos = parse_fen("5b2/8/8/8/8/8/8/7R w - - 0 1");

        assert!(eval(Colour::White, &pos.board) > eval(Colour::Black, &pos.board));
    }

    #[test]
    fn queens_are_worth_more_than_rooks() {
        let pos = parse_fen("7r/8/8/8/8/8/8/3Q4 w - - 0 1");

        assert!(eval(Colour::White, &pos.board) > eval(Colour::Black, &pos.board));
    }

    fn parse_fen(str: &str) -> Position {
        let pos = str.parse();
        assert!(pos.is_ok());

        pos.unwrap()
    }
}
