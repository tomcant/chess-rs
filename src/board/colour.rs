#[derive(Debug, Clone, Copy, PartialEq)]
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
