use crate::colour::Colour;
use crate::piece::{Piece, PieceType};
use crate::square::Square;

#[derive(Debug)]
pub struct Board {
    pieces: [u64; 12],
    colours: [u64; 2],
}

impl Board {
    pub fn empty() -> Self {
        Self {
            pieces: [0; 12],
            colours: [0; 2],
        }
    }

    pub fn pieces(&self, piece_type: PieceType, colour: Colour) -> u64 {
        self.pieces[Piece::from(piece_type, colour).index()]
    }

    pub fn pieces_by_colour(&self, colour: Colour) -> u64 {
        self.colours[colour as usize]
    }

    pub fn count_pieces(&self, piece_type: PieceType, colour: Colour) -> u32 {
        self.pieces(piece_type, colour).count_ones()
    }

    pub fn put_piece(&mut self, piece: Piece, square: Square) {
        let square_u64 = square.u64();
        self.pieces[piece.index()] |= square_u64;
        self.colours[piece.colour() as usize] |= square_u64;
    }

    pub fn piece_at(&self, square: Square) -> Option<Piece> {
        Piece::pieces()
            .iter()
            .find(|&&piece| self.pieces[piece.index()] & square.u64() != 0)
            .copied()
    }

    pub fn has_piece_at(&self, square: Square) -> bool {
        self.occupancy() & square.u64() != 0
    }

    pub fn remove_piece(&mut self, square: Square) {
        if let Some(piece) = self.piece_at(square) {
            self.pieces[piece.index()] ^= square.u64();
            self.colours[piece.colour() as usize] ^= square.u64();
        }
    }

    pub fn occupancy(&self) -> u64 {
        self.pieces_by_colour(Colour::White) + self.pieces_by_colour(Colour::Black)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn put_a_piece_on_the_board() {
        let mut board = Board::empty();
        let piece = Piece::WhiteKing;
        let square = "e1".parse::<Square>().unwrap();

        board.put_piece(piece, square);

        assert!(board.has_piece_at(square));
        assert_eq!(board.piece_at(square), Some(piece));
        assert_eq!(board.count_pieces(PieceType::King, Colour::White), 1);
        assert_eq!(board.pieces_by_colour(piece.colour()) & square.u64(), square.u64());
    }

    #[test]
    fn clear_a_square() {
        let mut board = Board::empty();
        let square = "e1".parse::<Square>().unwrap();
        board.put_piece(Piece::WhiteKing, square);

        assert!(board.has_piece_at(square));

        board.remove_piece(square);

        assert!(!board.has_piece_at(square));
    }
}
