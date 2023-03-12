use crate::piece::Piece;
use crate::position::CastlingRights;
use crate::square::Square;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub captured_piece: Option<Piece>,
    pub promotion_piece: Option<Piece>,
    pub castling_rights: CastlingRights,
    pub is_en_passant: bool,
}

impl Move {
    pub fn is_capture(&self) -> bool {
        self.captured_piece.is_some()
    }

    pub fn capture_square(&self) -> Square {
        if self.is_en_passant {
            return self.to.advance(self.captured_piece.unwrap().colour());
        }

        self.to
    }

    pub fn file_diff(&self) -> u8 {
        self.from.file_diff(self.to)
    }

    pub fn rank_diff(&self) -> u8 {
        self.from.rank_diff(self.to)
    }
}
