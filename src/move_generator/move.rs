use crate::board::{Piece, Square};
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub captured: Option<Piece>,
    pub promoted: Option<Piece>,
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}",
            self.from,
            if self.captured.is_some() { "x" } else { "" },
            self.to
        )
    }
}
