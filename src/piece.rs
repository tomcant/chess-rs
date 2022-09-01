use self::{Piece::*, PieceType::*};
use crate::colour::Colour;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

const PIECE_TYPES: [PieceType; 6] = [Pawn, Knight, Bishop, Rook, Queen, King];

impl PieceType {
    pub fn is_pawn(&self) -> bool {
        matches!(self, Pawn)
    }

    pub fn is_king(&self) -> bool {
        matches!(self, King)
    }

    pub fn types() -> &'static [Self] {
        &PIECE_TYPES
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

const PIECES: [Piece; 12] = [
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
];

const PROMOTION_PIECES: [[Piece; 4]; 2] = [
    [WhiteKnight, WhiteBishop, WhiteRook, WhiteQueen],
    [BlackKnight, BlackBishop, BlackRook, BlackQueen],
];

const PIECE_TYPE_TO_COLOUR_MAP: [[Piece; 6]; 2] = [
    [WhitePawn, WhiteKnight, WhiteBishop, WhiteRook, WhiteQueen, WhiteKing],
    [BlackPawn, BlackKnight, BlackBishop, BlackRook, BlackQueen, BlackKing],
];

impl Piece {
    pub fn from(piece_type: PieceType, colour: Colour) -> Self {
        PIECE_TYPE_TO_COLOUR_MAP[colour as usize][piece_type as usize]
    }

    pub fn index(&self) -> usize {
        *self as usize
    }

    pub fn get_type(&self) -> PieceType {
        match self {
            WhitePawn | BlackPawn => Pawn,
            WhiteKnight | BlackKnight => Knight,
            WhiteBishop | BlackBishop => Bishop,
            WhiteRook | BlackRook => Rook,
            WhiteQueen | BlackQueen => Queen,
            WhiteKing | BlackKing => King,
        }
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

    pub fn promotions(colour: Colour) -> &'static [Self] {
        &PROMOTION_PIECES[colour as usize]
    }

    pub fn pieces() -> &'static [Self] {
        &PIECES
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
