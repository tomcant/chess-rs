use super::{castling::CastlingAbility, GameState};
use crate::board::Board;
use crate::colour::Colour;
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
        castling_ability: parse_castling_ability(parts[2]),
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

fn parse_castling_ability(ability: &str) -> CastlingAbility {
    if ability == "-" {
        return CastlingAbility::NONE;
    }

    ability.chars().fold(CastlingAbility::NONE, |acc, char| {
        acc | match char {
            'K' => CastlingAbility::WHITE_KING,
            'Q' => CastlingAbility::WHITE_QUEEN,
            'k' => CastlingAbility::BLACK_KING,
            'q' => CastlingAbility::BLACK_QUEEN,
            _ => panic!("error parsing fen: invalid castling ability"),
        }
    })
}

fn parse_en_passant_square(str: &str) -> Option<Square> {
    if str == "-" {
        return None;
    }

    let result = str.parse::<Square>();

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
    use crate::game_state::{CastlingAbility, GameState};

    #[test]
    fn test_board() {
        let fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1";
        let state: GameState = fen.parse().unwrap();

        assert_eq!(
            state.board.get_piece_at("e4".parse::<Square>().unwrap()),
            Some(Piece::WhitePawn)
        );
        assert!(!state.board.has_piece_at("e2".parse::<Square>().unwrap()));
    }

    #[test]
    #[should_panic]
    fn test_invalid_board_too_few_rows() {
        let fen = "8/8 w - - 0 1";
        let _ = fen.parse::<GameState>();
    }

    #[test]
    #[should_panic]
    fn test_invalid_board_too_many_rows() {
        let fen = "8/8/8/8/8/8/8/8/1 w - - 0 1";
        let _ = fen.parse::<GameState>();
    }

    #[test]
    #[should_panic]
    fn test_invalid_board_too_few_squares() {
        let fen = "8/8/8/8/8/8/8/7 w - - 0 1";
        let _ = fen.parse::<GameState>();
    }

    #[test]
    #[should_panic]
    fn test_invalid_board_too_many_squares() {
        let fen = "8/8/8/8/8/8/8/9 w - - 0 1";
        let _ = fen.parse::<GameState>();
    }

    #[test]
    #[should_panic]
    fn test_invalid_board_invalid_piece() {
        let fen = "8/8/8/8/8/8/8/4a3 w - - 0 1";
        let _ = fen.parse::<GameState>();
    }

    #[test]
    fn test_colour_to_move_white() {
        let fen = "8/8/8/8/8/8/8/8 w - - 0 1";
        let state: GameState = fen.parse().unwrap();

        assert_eq!(state.colour_to_move, Colour::White);
    }

    #[test]
    fn test_colour_to_move_black() {
        let fen = "8/8/8/8/8/8/8/8 b - - 0 1";
        let state: GameState = fen.parse().unwrap();

        assert_eq!(state.colour_to_move, Colour::Black);
    }

    #[test]
    #[should_panic]
    fn test_invalid_colour_to_move() {
        let fen = "8/8/8/8/8/8/8/8 W - - 0 1";
        let _ = fen.parse::<GameState>();
    }

    #[test]
    fn test_castling_ability_none() {
        let fen = "8/8/8/8/8/8/8/8 w - - 0 1";
        let state: GameState = fen.parse().unwrap();

        assert_eq!(state.castling_ability, CastlingAbility::NONE);
    }

    #[test]
    fn test_castling_ability_partial() {
        let fen = "8/8/8/8/8/8/8/8 w Kq - 0 1";
        let state: GameState = fen.parse().unwrap();

        assert_eq!(
            state.castling_ability,
            CastlingAbility::WHITE_KING | CastlingAbility::BLACK_QUEEN
        );
    }

    #[test]
    fn test_castling_ability_all() {
        let fen = "8/8/8/8/8/8/8/8 w KQkq - 0 1";
        let state: GameState = fen.parse().unwrap();

        assert_eq!(state.castling_ability, CastlingAbility::ALL);
    }

    #[test]
    #[should_panic]
    fn test_invalid_castling_ability() {
        let fen = "8/8/8/8/8/8/8/8 w K- - 0 1";
        let _ = fen.parse::<GameState>();
    }

    #[test]
    fn test_en_passant_square_none() {
        let fen = "8/8/8/8/8/8/8/8 w - - 0 1";
        let state: GameState = fen.parse().unwrap();

        assert_eq!(state.en_passant_square, None);
    }

    #[test]
    fn test_en_passant_square_3rd_rank() {
        let fen = "8/8/8/8/8/8/8/8 w - f3 0 1";
        let state: GameState = fen.parse().unwrap();

        assert_eq!(state.en_passant_square, "f3".parse::<Square>().ok());
    }

    #[test]
    fn test_en_passant_square_6th_rank() {
        let fen = "8/8/8/8/8/8/8/8 w - f6 0 1";
        let state: GameState = fen.parse().unwrap();

        assert_eq!(state.en_passant_square, "f6".parse::<Square>().ok());
    }

    #[test]
    #[should_panic]
    fn test_invalid_en_passant_square() {
        let fen = "8/8/8/8/8/8/8/8 w - f4 0 1";
        let _ = fen.parse::<GameState>();
    }

    #[test]
    fn test_move_counters() {
        let fen = "8/8/8/8/8/8/8/8 w - - 10 20";
        let state: GameState = fen.parse().unwrap();

        assert_eq!(state.half_move_clock, 10);
        assert_eq!(state.full_move_counter, 20);
    }

    #[test]
    #[should_panic]
    fn test_invalid_too_few_parts() {
        let fen = "w - - 0 1";
        let _ = fen.parse::<GameState>();
    }

    #[test]
    #[should_panic]
    fn test_invalid_too_many_parts() {
        let fen = "8/8/8/8/8/8/8/8 w - - 0 1 extra";
        let _ = fen.parse::<GameState>();
    }
}
