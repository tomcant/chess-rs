use crate::colour::Colour;
use crate::square::Square;

const HISTORY_SCORE_MAX: i32 = 16384;

pub struct HistoryTable {
    table: [[[i32; 64]; 64]; 2],
}

impl HistoryTable {
    pub fn new() -> Self {
        Self {
            table: [[[0; 64]; 64]; 2],
        }
    }

    pub fn probe(&self, colour: Colour, from: Square, to: Square) -> i32 {
        self.table[colour][from][to]
    }

    pub fn store(&mut self, colour: Colour, from: Square, to: Square, depth: u8) {
        let bonus = (depth * depth) as i32;
        let entry = &mut self.table[colour][from][to];
        *entry += bonus - (*entry * bonus / HISTORY_SCORE_MAX);
    }
}
