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
    pub half_move_clock: u8,
    pub is_en_passant: bool,
}

impl Move {
    pub fn capture_square(&self) -> Option<Square> {
        let captured_piece = self.captured_piece?;

        if self.is_en_passant {
            Some(self.to.advance(captured_piece.colour()))
        } else {
            Some(self.to)
        }
    }

    pub fn file_diff(&self) -> u8 {
        self.from.file_diff(self.to)
    }

    pub fn rank_diff(&self) -> u8 {
        self.from.rank_diff(self.to)
    }
}
