use crate::board::{Piece, Square};

pub struct Move {
    pub from: Square,
    pub to: Square,
    pub captured: Option<Piece>,
    pub promoted: Option<Piece>,
}
