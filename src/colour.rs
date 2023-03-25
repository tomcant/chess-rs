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

impl<T> std::ops::Index<Colour> for [T; 2] {
    type Output = T;

    fn index(&self, colour: Colour) -> &Self::Output {
        &self[colour as usize]
    }
}

impl<T> std::ops::IndexMut<Colour> for [T; 2] {
    fn index_mut(&mut self, colour: Colour) -> &mut Self::Output {
        &mut self[colour as usize]
    }
}
