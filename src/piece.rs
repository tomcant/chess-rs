use self::Piece::*;
use crate::colour::Colour;
use std::fmt::{Display, Formatter};

#[rustfmt::skip]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    WP, WN, WB, WR, WQ, WK,
    BP, BN, BB, BR, BQ, BK,
}

#[rustfmt::skip]
impl Piece {
    const PAWNS:   [Piece; 2] = [WP, BP];
    const KNIGHTS: [Piece; 2] = [WN, BN];
    const BISHOPS: [Piece; 2] = [WB, BB];
    const ROOKS:   [Piece; 2] = [WR, BR];
    const QUEENS:  [Piece; 2] = [WQ, BQ];
    const KINGS:   [Piece; 2] = [WK, BK];

    pub fn pawn   (colour: Colour) -> Self { Self::PAWNS   [colour as usize] }
    pub fn knight (colour: Colour) -> Self { Self::KNIGHTS [colour as usize] }
    pub fn bishop (colour: Colour) -> Self { Self::BISHOPS [colour as usize] }
    pub fn rook   (colour: Colour) -> Self { Self::ROOKS   [colour as usize] }
    pub fn queen  (colour: Colour) -> Self { Self::QUEENS  [colour as usize] }
    pub fn king   (colour: Colour) -> Self { Self::KINGS   [colour as usize] }
}

#[rustfmt::skip]
impl Piece {
    const PIECES: [Piece; 12] = [
        WP, WN, WB, WR, WQ, WK,
        BP, BN, BB, BR, BQ, BK,
    ];

    const PIECES_BY_COLOUR: [[Piece; 6]; 2] = [
        [WP, WN, WB, WR, WQ, WK],
        [BP, BN, BB, BR, BQ, BK],
    ];

    const PROMOTION_PIECES_BY_COLOUR: [[Piece; 4]; 2] = [
        [WN, WB, WR, WQ],
        [BN, BB, BR, BQ],
    ];

    pub fn pieces() -> &'static [Self] {
        &Self::PIECES
    }

    pub fn pieces_by_colour(colour: Colour) -> &'static [Self] {
        &Self::PIECES_BY_COLOUR[colour as usize]
    }

    pub fn promotions(colour: Colour) -> &'static [Self] {
        &Self::PROMOTION_PIECES_BY_COLOUR[colour as usize]
    }
}

impl Piece {
    pub fn index(&self) -> usize {
        *self as usize
    }

    pub fn is_pawn(&self) -> bool {
        matches!(self, WP | BP)
    }

    pub fn is_king(&self) -> bool {
        matches!(self, WK | BK)
    }

    pub fn colour(&self) -> Colour {
        match self {
            WP | WN | WB | WR | WQ | WK => Colour::White,
            _ => Colour::Black,
        }
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let piece = match self {
            WP => 'P',
            WN => 'N',
            WB => 'B',
            WR => 'R',
            WQ => 'Q',
            WK => 'K',
            BP => 'p',
            BN => 'n',
            BB => 'b',
            BR => 'r',
            BQ => 'q',
            BK => 'k',
        };
        write!(f, "{piece}")
    }
}
