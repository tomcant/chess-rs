use crate::colour::Colour;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Square(u8);

impl Square {
    pub const A1: Self = Self(0);
    pub const B1: Self = Self(1);
    pub const C1: Self = Self(2);
    pub const D1: Self = Self(3);
    pub const E1: Self = Self(4);
    pub const F1: Self = Self(5);
    pub const G1: Self = Self(6);
    pub const H1: Self = Self(7);

    pub const A8: Self = Self(56);
    pub const B8: Self = Self(57);
    pub const C8: Self = Self(58);
    pub const D8: Self = Self(59);
    pub const E8: Self = Self(60);
    pub const F8: Self = Self(61);
    pub const G8: Self = Self(62);
    pub const H8: Self = Self(63);

    pub fn from_index(index: u8) -> Self {
        Self(index)
    }

    pub fn from_file_and_rank(file: u8, rank: u8) -> Self {
        Self(rank << 3 | file)
    }

    pub fn from_u64(u64: u64) -> Self {
        debug_assert_eq!(u64.count_ones(), 1, "given u64 must be a power of 2");
        Self(63 - u64.leading_zeros() as u8)
    }

    pub fn index(&self) -> u8 {
        self.0
    }

    pub fn u64(&self) -> u64 {
        1 << self.0
    }

    pub fn file(&self) -> u8 {
        self.0 & 7
    }

    pub fn rank(&self) -> u8 {
        self.0 >> 3
    }

    pub fn file_diff(&self, other: Square) -> u8 {
        self.file().abs_diff(other.file())
    }

    pub fn rank_diff(&self, other: Square) -> u8 {
        self.rank().abs_diff(other.rank())
    }

    pub fn advance(&self, colour: Colour) -> Self {
        match colour {
            Colour::White => Self(self.0 + 8),
            _ => Self(self.0 - 8),
        }
    }

    pub fn is_back_rank(&self) -> bool {
        matches!(self.rank(), 0 | 7)
    }

    pub fn is_corner(&self) -> bool {
        matches!(*self, Self::A1 | Self::H1 | Self::A8 | Self::H8)
    }
}

impl<T> std::ops::Index<Square> for [T; 64] {
    type Output = T;

    fn index(&self, square: Square) -> &Self::Output {
        &self[square.0 as usize]
    }
}

impl<T> std::ops::IndexMut<Square> for [T; 64] {
    fn index_mut(&mut self, square: Square) -> &mut Self::Output {
        &mut self[square.0 as usize]
    }
}

impl std::fmt::Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", (b'a' + self.file()) as char, 1 + self.rank())
    }
}

impl std::str::FromStr for Square {
    type Err = String;

    fn from_str(square: &str) -> Result<Self, Self::Err> {
        if square.len() != 2 {
            return Err(format!("invalid square '{square}'"));
        }

        let chars: Vec<_> = square.chars().collect();

        let file = chars[0] as u8;
        let rank = chars[1] as u8;

        if !(b'a'..=b'h').contains(&file) || !(b'1'..=b'8').contains(&rank) {
            return Err(format!("invalid square '{square}'"));
        }

        Ok(Self::from_file_and_rank(file - b'a', rank - b'1'))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_from_a_file_and_a_rank() {
        assert_eq!(Square::from_file_and_rank(0, 0), Square::A1);
        assert_eq!(Square::from_file_and_rank(7, 7), Square::H8);
        assert_eq!(Square::from_file_and_rank(1, 4), Square::from_index(33));
    }

    #[test]
    fn create_from_a_square_value_in_a_64_bit_board_arrangement() {
        assert_eq!(Square::from_u64(1), Square::A1);
        assert_eq!(Square::from_u64(2u64.pow(63)), Square::H8);
        assert_eq!(Square::from_u64(2u64.pow(33)), Square::from_index(33));
    }

    #[test]
    fn create_from_algebraic_notation() {
        assert_eq!(parse_square("a1"), Square::A1);
        assert_eq!(parse_square("h8"), Square::H8);
        assert_eq!(parse_square("b5"), Square::from_index(33));
    }

    #[test]
    fn it_cannot_be_created_from_invalid_algebraic_notation() {
        for str in ["", "a", "a1b", "a9", "i1"] {
            assert!(str.parse::<Square>().is_err());
        }
    }

    #[test]
    fn advance_a_square_given_a_colour() {
        let square = parse_square("e4");

        assert_eq!(parse_square("e5"), square.advance(Colour::White));
        assert_eq!(parse_square("e3"), square.advance(Colour::Black));
    }

    fn parse_square(str: &str) -> Square {
        let square = str.parse();
        assert!(square.is_ok());

        square.unwrap()
    }
}
