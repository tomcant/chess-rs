use super::Colour;
use std::slice::Iter;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl PieceType {
    pub fn iter() -> Iter<'static, Self> {
        [
            Self::Pawn,
            Self::Knight,
            Self::Bishop,
            Self::Rook,
            Self::Queen,
            Self::King,
        ]
        .iter()
    }
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

const PIECE_TYPE_TO_COLOUR_MAP: [[Piece; 6]; 2] = [
    [
        Piece::WhitePawn,
        Piece::WhiteKnight,
        Piece::WhiteBishop,
        Piece::WhiteRook,
        Piece::WhiteQueen,
        Piece::WhiteKing,
    ],
    [
        Piece::BlackPawn,
        Piece::BlackKnight,
        Piece::BlackBishop,
        Piece::BlackRook,
        Piece::BlackQueen,
        Piece::BlackKing,
    ],
];

impl Piece {
    pub fn make(piece_type: PieceType, colour: Colour) -> Self {
        PIECE_TYPE_TO_COLOUR_MAP[colour as usize][piece_type as usize]
    }

    pub fn index(&self) -> usize {
        *self as usize
    }

    pub fn get_type(&self) -> PieceType {
        match self {
            Self::WhitePawn | Self::BlackPawn => PieceType::Pawn,
            Self::WhiteKnight | Self::BlackKnight => PieceType::Knight,
            Self::WhiteBishop | Self::BlackBishop => PieceType::Bishop,
            Self::WhiteRook | Self::BlackRook => PieceType::Rook,
            Self::WhiteQueen | Self::BlackQueen => PieceType::Queen,
            Self::WhiteKing | Self::BlackKing => PieceType::King,
        }
    }

    pub fn colour(&self) -> Colour {
        match self {
            Self::WhitePawn
            | Self::WhiteKnight
            | Self::WhiteBishop
            | Self::WhiteRook
            | Self::WhiteQueen
            | Self::WhiteKing => Colour::White,
            _ => Colour::Black,
        }
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
