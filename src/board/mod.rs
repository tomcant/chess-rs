pub mod colour;
pub mod piece;
pub mod square;

pub use colour::Colour;
pub use piece::{Piece, PieceType};
pub use square::Square;

use crate::attacks::get_attackers;

pub type BitBoard = u64;

#[derive(Debug)]
pub struct Board {
    pieces: [BitBoard; 12],
    colours: [BitBoard; 2],
}

impl Board {
    pub fn empty() -> Self {
        Self {
            pieces: [0; 12],
            colours: [0; 2],
        }
    }

    pub fn put_piece(&mut self, piece: Piece, square: Square) {
        let square_u64 = square.u64();
        self.pieces[piece.index()] |= square_u64;
        self.colours[piece.colour() as usize] |= square_u64;
    }

    pub fn get_piece_at(&self, square: Square) -> Option<Piece> {
        let square_u64 = square.u64();

        Piece::iter()
            .find(|&&piece| self.pieces[piece.index()] & square_u64 != 0)
            .copied()
    }

    pub fn has_piece_at(&self, square: Square) -> bool {
        self.occupancy() & square.u64() != 0
    }

    pub fn get_pieces(&self, piece_type: PieceType, colour: Colour) -> BitBoard {
        self.pieces[Piece::make(piece_type, colour).index()]
    }

    pub fn get_pieces_by_colour(&self, colour: Colour) -> BitBoard {
        self.colours[colour as usize]
    }

    pub fn clear_square(&mut self, square: Square) {
        if let Some(piece) = self.get_piece_at(square) {
            self.pieces[piece.index()] ^= square.u64();
            self.colours[piece.colour() as usize] ^= square.u64();
        }
    }

    pub fn is_in_check(&self, colour: Colour) -> bool {
        let king_square = Square::from_u64(self.get_pieces(PieceType::King, colour));
        let attackers = get_attackers(king_square, colour.flip(), self);

        attackers.count_ones() > 0
    }

    pub fn occupancy(&self) -> BitBoard {
        self.colours.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::{Board, Colour, Piece, Square};

    #[test]
    fn test_put_piece() {
        let mut board = Board::empty();
        let piece = Piece::WhiteKing;
        let square = "e1".parse::<Square>().unwrap();

        board.put_piece(piece, square);

        assert!(board.has_piece_at(square));
        assert_eq!(board.get_piece_at(square), Some(piece));
        assert_eq!(board.get_pieces_by_colour(piece.colour()) & square.u64(), square.u64());
    }

    #[test]
    fn test_clear_square() {
        let mut board = Board::empty();
        let square = "e1".parse::<Square>().unwrap();
        board.put_piece(Piece::WhiteKing, square);

        assert!(board.has_piece_at(square));

        board.clear_square(square);

        assert!(!board.has_piece_at(square));
    }

    #[test]
    fn test_it_can_detect_check() {
        let mut board = Board::empty();
        board.put_piece(Piece::BlackKing, "e8".parse::<Square>().unwrap());
        board.put_piece(Piece::WhiteKnight, "d6".parse::<Square>().unwrap());

        assert!(board.is_in_check(Colour::Black));
    }
}
