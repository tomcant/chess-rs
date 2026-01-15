use crate::piece::Piece;
use crate::square::Square;

pub const HISTORY_SCORE_MAX: i32 = 16384;

pub struct HistoryTable {
    table: [[i32; 64]; 12],
}

impl HistoryTable {
    pub fn new() -> Self {
        Self { table: [[0; 64]; 12] }
    }

    pub fn probe(&self, piece: Piece, to: Square) -> i32 {
        self.table[piece][to]
    }

    pub fn store(&mut self, bonus: i32, piece: Piece, to: Square) {
        let entry = &mut self.table[piece][to];
        *entry += bonus - *entry * bonus.abs() / HISTORY_SCORE_MAX;
    }
}
