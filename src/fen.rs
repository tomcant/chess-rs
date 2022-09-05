use crate::board::Board;
use crate::castling::{CastlingRight, CastlingRights};
use crate::colour::Colour;
use crate::game::GameState;
use crate::piece::Piece;
use crate::square::Square;
use std::str::FromStr;

impl FromStr for GameState {
    type Err = ();

    fn from_str(fen: &str) -> Result<Self, Self::Err> {
        Ok(parse_fen(fen))
    }
}

fn parse_fen(fen: &str) -> GameState {
    let parts: Vec<&str> = fen.split_whitespace().collect();
    const NUM_PARTS: usize = 6;

    if parts.len() != NUM_PARTS {
        panic!("error parsing fen: must contain {} parts", NUM_PARTS);
    }

    GameState {
        board: parse_board(parts[0]),
        colour_to_move: parse_colour_to_move(parts[1]),
        castling_rights: parse_castling_rights(parts[2]),
        en_passant_square: parse_en_passant_square(parts[3]),
        half_move_clock: parts[4].parse().unwrap(),
        full_move_counter: parts[5].parse().unwrap(),
    }
}

fn parse_board(str: &str) -> Board {
    if str.matches('/').count() != 7 {
        panic!("error parsing fen: board must contain 8 rows");
    }

    let mut board = Board::empty();
    let mut square_index = "a8".parse::<Square>().unwrap().index() as u8;

    for char in str.chars() {
        if char == '/' {
            square_index -= 16;
            continue;
        }

        if char.is_ascii_digit() {
            square_index += char as u8 - b'0';
            continue;
        }

        let piece = match char {
            'P' => Piece::WhitePawn,
            'N' => Piece::WhiteKnight,
            'B' => Piece::WhiteBishop,
            'R' => Piece::WhiteRook,
            'Q' => Piece::WhiteQueen,
            'K' => Piece::WhiteKing,

            'p' => Piece::BlackPawn,
            'n' => Piece::BlackKnight,
            'b' => Piece::BlackBishop,
            'r' => Piece::BlackRook,
            'q' => Piece::BlackQueen,
            'k' => Piece::BlackKing,

            _ => panic!("error parsing fen: invalid piece '{char}'"),
        };

        board.put_piece(piece, Square::from_index(square_index));
        square_index += 1;
    }

    if square_index != 8 {
        panic!("error parsing fen: board must contain 64 squares");
    }

    board
}

fn parse_colour_to_move(colour: &str) -> Colour {
    match colour {
        "w" => Colour::White,
        "b" => Colour::Black,
        _ => panic!("error parsing fen: invalid colour to move"),
    }
}

fn parse_castling_rights(rights: &str) -> CastlingRights {
    if rights == "-" {
        return CastlingRights::none();
    }

    rights.chars().fold(CastlingRights::none(), |mut acc, char| {
        acc.add(match char {
            'K' => CastlingRight::WhiteKing,
            'Q' => CastlingRight::WhiteQueen,
            'k' => CastlingRight::BlackKing,
            'q' => CastlingRight::BlackQueen,
            _ => panic!("error parsing fen: invalid castling right"),
        });
        acc
    })
}

fn parse_en_passant_square(square: &str) -> Option<Square> {
    if square == "-" {
        return None;
    }

    let result = square.parse::<Square>();

    if result.is_err() {
        panic!("error parsing fen: invalid en passant square");
    }

    let square = result.unwrap();

    if square.rank() != 2 && square.rank() != 5 {
        panic!("error parsing fen: invalid en passant square");
    }

    Some(square)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::GameState;

    #[test]
    fn board() {
        let state = parse_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1");

        assert_eq!(state.board.piece_at(parse_square("e4")), Some(Piece::WhitePawn));
        assert!(!state.board.has_piece_at(parse_square("e2")));
    }

    #[test]
    #[should_panic]
    fn invalid_piece() {
        parse_fen("8/8/8/8/8/8/8/4a3 w - - 0 1");
    }

    #[test]
    #[should_panic]
    fn too_few_rows() {
        parse_fen("8/8 w - - 0 1");
    }

    #[test]
    #[should_panic]
    fn too_many_rows() {
        parse_fen("8/8/8/8/8/8/8/8/1 w - - 0 1");
    }

    #[test]
    #[should_panic]
    fn too_few_squares() {
        parse_fen("8/8/8/8/8/8/8/7 w - - 0 1");
    }

    #[test]
    #[should_panic]
    fn too_many_squares() {
        parse_fen("8/8/8/8/8/8/8/9 w - - 0 1");
    }

    #[test]
    #[should_panic]
    fn too_few_parts() {
        parse_fen("w - - 0 1");
    }

    #[test]
    #[should_panic]
    fn too_many_parts() {
        parse_fen("8/8/8/8/8/8/8/8 w - - 0 1 extra");
    }

    #[test]
    fn white_to_move() {
        let state = parse_fen("8/8/8/8/8/8/8/8 w - - 0 1");

        assert_eq!(state.colour_to_move, Colour::White);
    }

    #[test]
    fn black_to_move() {
        let state = parse_fen("8/8/8/8/8/8/8/8 b - - 0 1");

        assert_eq!(state.colour_to_move, Colour::Black);
    }

    #[test]
    #[should_panic]
    fn invalid_colour_to_move() {
        parse_fen("8/8/8/8/8/8/8/8 W - - 0 1");
    }

    #[test]
    fn no_castling_rights() {
        let state = parse_fen("8/8/8/8/8/8/8/8 w - - 0 1");

        assert_eq!(state.castling_rights, CastlingRights::none());
    }

    #[test]
    fn partial_castling_rights() {
        let state = parse_fen("8/8/8/8/8/8/8/8 w Kq - 0 1");

        assert_eq!(
            state.castling_rights,
            CastlingRights::from(&[CastlingRight::WhiteKing, CastlingRight::BlackQueen])
        );
    }

    #[test]
    fn all_castling_rights() {
        let state = parse_fen("8/8/8/8/8/8/8/8 w KQkq - 0 1");

        assert_eq!(state.castling_rights, CastlingRights::all());
    }

    #[test]
    #[should_panic]
    fn invalid_castling_rights() {
        parse_fen("8/8/8/8/8/8/8/8 w K- - 0 1");
    }

    #[test]
    fn no_en_passant_square() {
        let state = parse_fen("8/8/8/8/8/8/8/8 w - - 0 1");

        assert_eq!(state.en_passant_square, None);
    }

    #[test]
    fn en_passant_square_3rd_rank() {
        let state = parse_fen("8/8/8/8/8/8/8/8 w - f3 0 1");

        assert_eq!(state.en_passant_square, Some(parse_square("f3")));
    }

    #[test]
    fn en_passant_square_6th_rank() {
        let state = parse_fen("8/8/8/8/8/8/8/8 w - f6 0 1");

        assert_eq!(state.en_passant_square, Some(parse_square("f6")));
    }

    #[test]
    #[should_panic]
    fn invalid_en_passant_square() {
        parse_fen("8/8/8/8/8/8/8/8 w - f4 0 1");
    }

    #[test]
    fn move_counters() {
        let state = parse_fen("8/8/8/8/8/8/8/8 w - - 10 20");

        assert_eq!(state.half_move_clock, 10);
        assert_eq!(state.full_move_counter, 20);
    }

    fn parse_fen(str: &str) -> GameState {
        let state = str.parse();
        assert!(state.is_ok());

        state.unwrap()
    }

    fn parse_square(str: &str) -> Square {
        let square = str.parse();
        assert!(square.is_ok());

        square.unwrap()
    }
}
