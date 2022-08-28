use crate::colour::Colour;
use crate::piece::{Piece, PieceType};
use crate::square::Square;

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
        Piece::pieces()
            .iter()
            .find(|&&piece| self.pieces[piece.index()] & square.u64() != 0)
            .copied()
    }

    pub fn has_piece_at(&self, square: Square) -> bool {
        self.occupancy() & square.u64() != 0
    }

    pub fn get_pieces(&self, piece_type: PieceType, colour: Colour) -> BitBoard {
        self.pieces[Piece::from(piece_type, colour).index()]
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

    pub fn occupancy(&self) -> BitBoard {
        self.colours.iter().sum()
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
        assert_eq!(board.get_piece_at(square), Some(piece));
        assert_eq!(board.get_pieces_by_colour(piece.colour()) & square.u64(), square.u64());
    }

    #[test]
    fn clear_a_square() {
        let mut board = Board::empty();
        let square = "e1".parse::<Square>().unwrap();
        board.put_piece(Piece::WhiteKing, square);

        assert!(board.has_piece_at(square));

        board.clear_square(square);

        assert!(!board.has_piece_at(square));
    }
}
