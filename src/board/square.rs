use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Square(u8);

impl Square {
    pub fn from_index(index: u8) -> Self {
        Self(index)
    }

    pub fn from_file_and_rank(file: u8, rank: u8) -> Self {
        Self(rank << 3 | file)
    }

    pub fn index(&self) -> u8 {
        self.0
    }

    pub fn file(&self) -> u8 {
        self.0 & 7
    }

    pub fn rank(&self) -> u8 {
        self.0 >> 3
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

        Ok(Square::from_file_and_rank(file - b'a', rank - b'1'))
    }
}

#[cfg(test)]
mod tests {
    use super::Square;

    #[test]
    fn test_from_file_and_rank() {
        assert_eq!(Square::from_file_and_rank(0, 0), Square::from_index(0));
        assert_eq!(Square::from_file_and_rank(7, 7), Square::from_index(63));
        assert_eq!(Square::from_file_and_rank(1, 4), Square::from_index(33));
    }

    #[test]
    fn test_from_string_valid() {
        assert_eq!("a1".parse::<Square>(), Ok(Square::from_index(0)));
        assert_eq!("h8".parse::<Square>(), Ok(Square::from_index(63)));
        assert_eq!("b5".parse::<Square>(), Ok(Square::from_index(33)));
    }

    #[test]
    fn test_invalid_from_string() {
        assert!("a".parse::<Square>().is_err());
        assert!("i1".parse::<Square>().is_err());
        assert!("a9".parse::<Square>().is_err());
    }
}
