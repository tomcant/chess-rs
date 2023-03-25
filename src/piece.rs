use self::Piece::*;
use crate::colour::Colour;
use std::fmt::{Display, Formatter};

#[rustfmt::skip]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    WhitePawn, WhiteKnight, WhiteBishop, WhiteRook, WhiteQueen, WhiteKing,
    BlackPawn, BlackKnight, BlackBishop, BlackRook, BlackQueen, BlackKing,
}

#[rustfmt::skip]
impl Piece {
    const PAWNS:   [Piece; 2] = [WhitePawn,   BlackPawn  ];
    const KNIGHTS: [Piece; 2] = [WhiteKnight, BlackKnight];
    const BISHOPS: [Piece; 2] = [WhiteBishop, BlackBishop];
    const ROOKS:   [Piece; 2] = [WhiteRook,   BlackRook  ];
    const QUEENS:  [Piece; 2] = [WhiteQueen,  BlackQueen ];
    const KINGS:   [Piece; 2] = [WhiteKing,   BlackKing  ];

    pub fn pawn   (colour: Colour) -> Self { Self::PAWNS   [colour as usize] }
    pub fn knight (colour: Colour) -> Self { Self::KNIGHTS [colour as usize] }
    pub fn bishop (colour: Colour) -> Self { Self::BISHOPS [colour as usize] }
    pub fn rook   (colour: Colour) -> Self { Self::ROOKS   [colour as usize] }
    pub fn queen  (colour: Colour) -> Self { Self::QUEENS  [colour as usize] }
    pub fn king   (colour: Colour) -> Self { Self::KINGS   [colour as usize] }
}

impl Piece {
    #[rustfmt::skip]
    const PIECES: [Piece; 12] = [
        WhitePawn, WhiteKnight, WhiteBishop, WhiteRook, WhiteQueen, WhiteKing,
        BlackPawn, BlackKnight, BlackBishop, BlackRook, BlackQueen, BlackKing,
    ];

    const PIECES_BY_COLOUR: [[Piece; 6]; 2] = [
        [WhitePawn, WhiteKnight, WhiteBishop, WhiteRook, WhiteQueen, WhiteKing],
        [BlackPawn, BlackKnight, BlackBishop, BlackRook, BlackQueen, BlackKing],
    ];

    const PROMOTION_PIECES_BY_COLOUR: [[Piece; 4]; 2] = [
        [WhiteKnight, WhiteBishop, WhiteRook, WhiteQueen],
        [BlackKnight, BlackBishop, BlackRook, BlackQueen],
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
        matches!(self, WhitePawn | BlackPawn)
    }

    pub fn is_king(&self) -> bool {
        matches!(self, WhiteKing | BlackKing)
    }

    pub fn colour(&self) -> Colour {
        match self {
            WhitePawn | WhiteKnight | WhiteBishop | WhiteRook | WhiteQueen | WhiteKing => Colour::White,
            _ => Colour::Black,
        }
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let piece = match self {
            WhitePawn => 'P',
            WhiteKnight => 'N',
            WhiteBishop => 'B',
            WhiteRook => 'R',
            WhiteQueen => 'Q',
            WhiteKing => 'K',
            BlackPawn => 'p',
            BlackKnight => 'n',
            BlackBishop => 'b',
            BlackRook => 'r',
            BlackQueen => 'q',
            BlackKing => 'k',
        };
        write!(f, "{piece}")
    }
}
