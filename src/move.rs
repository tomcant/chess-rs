use crate::piece::Piece;
use crate::square::Square;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub captured_piece: Option<Piece>,
    pub promotion_piece: Option<Piece>,
    pub is_en_passant: bool,
}

impl Move {
    pub fn is_capture(&self) -> bool {
        self.captured_piece.is_some()
    }

    pub fn get_capture_square(&self) -> Square {
        if self.is_en_passant {
            return self.to.up_for_colour(self.captured_piece.unwrap().colour());
        }

        self.to
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}",
            self.from,
            if self.is_capture() { "x" } else { "" },
            self.to
        )
    }
}
