pub mod colour;
pub mod piece;
pub mod square;

pub use colour::Colour;
pub use piece::Piece;
pub use square::Square;

type BitBoard = u64;

#[derive(Debug)]
pub struct Board {
    pieces: [BitBoard; 12],
}

impl Board {
    pub fn empty() -> Self {
        Self { pieces: [0; 12] }
    }

    pub fn put_piece(&mut self, piece: Piece, square: Square) {
        self.pieces[piece as usize] |= 1 << square.index();
    }

    pub fn has_piece_at(&self, square: Square) -> bool {
        self.occupancy() & 1 << square.index() != 0
    }

    pub fn piece_at(&self, square: Square) -> Option<Piece> {
        let square_index = 1 << square.index();

        Piece::iter()
            .find(|&&piece| self.pieces[piece as usize] & square_index != 0)
            .copied()
    }

    pub fn clear_square(&mut self, square: Square) {
        if let Some(piece) = self.piece_at(square) {
            self.pieces[piece as usize] ^= 1 << square.index();
        }
    }

    fn occupancy(&self) -> BitBoard {
        self.pieces.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::{Board, Piece, Square};

    #[test]
    fn test_put_piece() {
        let mut board = Board::empty();
        let piece = Piece::WhiteKing;
        let square = "e1".parse::<Square>().unwrap();

        board.put_piece(piece, square);

        assert!(board.has_piece_at(square));
        assert_eq!(board.piece_at(square), Some(piece));
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
}
