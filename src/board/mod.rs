pub mod colour;
pub mod piece;
pub mod square;

pub use colour::Colour;
pub use piece::{Piece, PieceType};
pub use square::Square;

pub type BitBoard = u64;

#[derive(Debug)]
pub struct Board {
    pieces: [BitBoard; 12],
    // colours: [BitBoard; 2],
}

impl Board {
    pub fn empty() -> Self {
        Self { pieces: [0; 12] }
    }

    pub fn put_piece(&mut self, piece: Piece, square: Square) {
        self.pieces[piece.index()] |= square.u64();
    }

    pub fn get_piece_at(&self, square: Square) -> Option<Piece> {
        if !self.has_piece_at(square) {
            return None;
        }

        let square_u64 = square.u64();

        Piece::iter()
            .find(|&&piece| self.pieces[piece.index()] & square_u64 != 0)
            .copied()
    }

    pub fn has_piece_at(&self, square: Square) -> bool {
        self.occupancy() & square.u64() != 0
    }

    pub fn get_pieces(&self, piece: Piece) -> BitBoard {
        self.pieces[piece.index()]
    }

    pub fn clear_square(&mut self, square: Square) {
        if let Some(piece) = self.get_piece_at(square) {
            self.pieces[piece.index()] ^= square.u64();
        }
    }

    pub fn get_king_square(&self, colour: Colour) -> Square {
        Square::from_u64(self.get_pieces(Piece::make(PieceType::King, colour)))
    }

    pub fn occupancy(&self) -> BitBoard {
        self.pieces.iter().sum()
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
    fn test_get_king_square() {
        let mut board = Board::empty();
        let white_king_square = "e1".parse::<Square>().unwrap();
        let black_king_square = "e8".parse::<Square>().unwrap();
        board.put_piece(Piece::WhiteKing, white_king_square);
        board.put_piece(Piece::BlackKing, black_king_square);

        assert_eq!(board.get_king_square(Colour::White), white_king_square);
        assert_eq!(board.get_king_square(Colour::Black), black_king_square);
    }
}
