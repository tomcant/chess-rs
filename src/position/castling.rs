use crate::colour::Colour;
use crate::square::Square;

#[derive(Debug, Clone, Copy)]
pub enum CastlingRight {
    WhiteKing,
    WhiteQueen,
    BlackKing,
    BlackQueen,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CastlingRights(u8);

impl CastlingRights {
    pub fn none() -> Self {
        Self(0)
    }

    pub fn has(&self, right: CastlingRight) -> bool {
        self.0 & 1 << right as u8 != 0
    }

    pub fn add(&mut self, right: CastlingRight) {
        self.0 |= 1 << right as u8;
    }

    fn remove(&mut self, right: CastlingRight) {
        self.0 &= !(1 << right as u8);
    }

    pub fn remove_for_colour(&mut self, colour: Colour) {
        match colour {
            Colour::White => {
                self.remove(CastlingRight::WhiteKing);
                self.remove(CastlingRight::WhiteQueen);
            }
            _ => {
                self.remove(CastlingRight::BlackKing);
                self.remove(CastlingRight::BlackQueen);
            }
        };
    }

    pub fn remove_for_square(&mut self, square: Square) {
        self.remove(match square {
            Square::A1 => CastlingRight::WhiteQueen,
            Square::H1 => CastlingRight::WhiteKing,
            Square::A8 => CastlingRight::BlackQueen,
            Square::H8 => CastlingRight::BlackKing,
            _ => panic!("cannot remove castling rights for square"),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_a_castling_right() {
        let mut rights = CastlingRights::none();

        rights.add(CastlingRight::WhiteKing);

        assert_eq!(rights, CastlingRights::from(&[CastlingRight::WhiteKing]));
    }

    #[test]
    fn remove_a_castling_right() {
        let mut rights = CastlingRights::all();

        rights.remove(CastlingRight::WhiteKing);

        assert_eq!(
            rights,
            CastlingRights::from(&[
                CastlingRight::WhiteQueen,
                CastlingRight::BlackKing,
                CastlingRight::BlackQueen,
            ])
        );
    }

    #[test]
    fn remove_castling_rights_for_a_colour() {
        let mut rights = CastlingRights::all();

        rights.remove_for_colour(Colour::White);

        assert_eq!(
            rights,
            CastlingRights::from(&[CastlingRight::BlackKing, CastlingRight::BlackQueen])
        );
    }

    #[test]
    fn remove_castling_rights_for_a_corner_square() {
        let mut rights = CastlingRights::all();

        rights.remove_for_square(Square::H1);

        assert_eq!(
            rights,
            CastlingRights::from(&[
                CastlingRight::WhiteQueen,
                CastlingRight::BlackKing,
                CastlingRight::BlackQueen,
            ])
        );
    }

    #[test]
    fn check_for_presence_of_a_castling_right() {
        let rights = CastlingRights::from(&[CastlingRight::WhiteKing]);

        assert!(rights.has(CastlingRight::WhiteKing));

        let not_rights = [
            CastlingRight::WhiteQueen,
            CastlingRight::BlackKing,
            CastlingRight::BlackQueen,
        ];

        for right in not_rights {
            assert!(!rights.has(right));
        }
    }

    impl CastlingRights {
        pub fn all() -> Self {
            Self(15)
        }

        pub fn from(rights: &[CastlingRight]) -> Self {
            rights.iter().fold(Self::none(), |mut acc, right| {
                acc.add(*right);
                acc
            })
        }
    }
}
