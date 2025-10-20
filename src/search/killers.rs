use super::MAX_DEPTH;
use crate::movegen::Move;

pub struct KillerMoves {
    moves: [[Option<Move>; 2]; MAX_DEPTH as usize],
}

impl KillerMoves {
    pub fn new() -> Self {
        Self {
            moves: [[None; 2]; MAX_DEPTH as usize],
        }
    }

    pub fn probe(&self, ply: u8, index: usize) -> Option<Move> {
        self.moves[ply as usize][index]
    }

    pub fn store(&mut self, ply: u8, mv: Move) {
        let ply = ply as usize;

        if Some(mv) != self.moves[ply][0] {
            self.moves[ply][1] = self.moves[ply][0];
            self.moves[ply][0] = Some(mv);
        }
    }
}
