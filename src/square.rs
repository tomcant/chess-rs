use crate::colour::Colour;
use lazy_static::lazy_static;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Square(u8);

lazy_static! {
    static ref SQUARES: [Square; 64] = (0..64).map(Square).collect::<Vec<_>>().try_into().unwrap();
}

impl Square {
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

    pub fn index(&self) -> usize {
        self.0 as usize
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
        matches!(self.0, 0 | 7 | 56 | 63)
    }

    pub fn squares() -> &'static [Self; 64] {
        &SQUARES
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}{}", (b'a' + self.file()) as char, 1 + self.rank())
    }
}

impl FromStr for Square {
    type Err = ();

    fn from_str(square: &str) -> Result<Self, Self::Err> {
        if square.len() != 2 {
            return Err(());
        }

        let chars: Vec<_> = square.chars().collect();

        let file = chars[0] as u8;
        let rank = chars[1] as u8;

        if !(b'a'..=b'h').contains(&file) || !(b'1'..=b'8').contains(&rank) {
            return Err(());
        }

        Ok(Self::from_file_and_rank(file - b'a', rank - b'1'))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_from_a_file_and_a_rank() {
        assert_eq!(Square::from_file_and_rank(0, 0), Square::from_index(0));
        assert_eq!(Square::from_file_and_rank(7, 7), Square::from_index(63));
        assert_eq!(Square::from_file_and_rank(1, 4), Square::from_index(33));
    }

    #[test]
    fn create_from_a_square_value_in_a_64_bit_board_arrangement() {
        assert_eq!(Square::from_u64(1), Square::from_index(0));
        assert_eq!(Square::from_u64(2u64.pow(63)), Square::from_index(63));
        assert_eq!(Square::from_u64(2u64.pow(33)), Square::from_index(33));
    }

    #[test]
    fn create_from_algebraic_notation() {
        assert_eq!(parse_square("a1"), Square::from_index(0));
        assert_eq!(parse_square("h8"), Square::from_index(63));
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
