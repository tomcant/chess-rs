use crate::colour::Colour;
use crate::piece::{Piece, PieceType};
use crate::square::Square;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UciMove {
    pub from: Square,
    pub to: Square,
    pub promotion_piece: Option<Piece>,
}

impl FromStr for UciMove {
    type Err = String;

    fn from_str(mv: &str) -> Result<Self, Self::Err> {
        if mv.len() != 4 && mv.len() != 5 {
            return Err("invalid UCI move".to_string());
        }

        let from = mv[0..2].parse::<Square>()?;
        let to = mv[2..4].parse::<Square>()?;

        let promotion_piece = if mv.len() > 4 {
            Some(Piece::from(
                match mv.chars().nth(4).unwrap() {
                    'n' => PieceType::Knight,
                    'b' => PieceType::Bishop,
                    'r' => PieceType::Rook,
                    'q' => PieceType::Queen,
                    _ => return Err("invalid promotion piece in UCI move".to_string()),
                },
                match to.rank() {
                    0 => Colour::Black,
                    _ => Colour::White,
                },
            ))
        } else {
            None
        };

        Ok(UciMove {
            from,
            to,
            promotion_piece,
        })
    }
}
