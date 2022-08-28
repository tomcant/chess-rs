#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Colour {
    White,
    Black,
}

impl Colour {
    pub fn flip(&self) -> Self {
        match self {
            Self::White => Self::Black,
            _ => Self::White,
        }
    }
}
