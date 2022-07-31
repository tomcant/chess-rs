use super::Colour;
use std::slice::Iter;

pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Piece {
    WhitePawn,
    WhiteKnight,
    WhiteBishop,
    WhiteRook,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackKnight,
    BlackBishop,
    BlackRook,
    BlackQueen,
    BlackKing,
}

impl Piece {
    pub fn make(pt: PieceType, colour: Colour) -> Self {
        let is_white = colour == Colour::White;

        #[rustfmt::skip]
        return match pt {
            PieceType::Pawn => if is_white { Self::WhitePawn } else { Self::BlackPawn },
            PieceType::Knight => if is_white { Self::WhiteKnight } else { Self::BlackKnight },
            PieceType::Bishop => if is_white { Self::WhiteBishop } else { Self::BlackBishop },
            PieceType::Rook => if is_white { Self::WhiteRook } else { Self::BlackRook },
            PieceType::Queen => if is_white { Self::WhiteQueen } else { Self::BlackQueen },
            PieceType::King => if is_white { Self::WhiteKing } else { Self::BlackKing },
        };
    }

    pub fn iter() -> Iter<'static, Self> {
        [
            Self::WhitePawn,
            Self::WhiteKnight,
            Self::WhiteBishop,
            Self::WhiteRook,
            Self::WhiteQueen,
            Self::WhiteKing,
            Self::BlackPawn,
            Self::BlackKnight,
            Self::BlackBishop,
            Self::BlackRook,
            Self::BlackQueen,
            Self::BlackKing,
        ]
        .iter()
    }
}
