use crate::r#move::Move;
use crate::search::{Report, Reporter};
use std::cell::Cell;

const NANOS_PER_SEC: u128 = 1_000_000_000;

pub struct UciReporter {
    best_move: Cell<Option<Move>>,
}

impl UciReporter {
    pub fn new() -> Self {
        Self {
            best_move: Cell::new(None),
        }
    }

    pub fn best_move(&self) -> Option<Move> {
        self.best_move.get()
    }
}

impl Reporter for UciReporter {
    fn send(&self, report: &Report) {
        let mut info = vec![
            format!("depth {}", report.depth),
            format!("nodes {}", report.nodes),
            format!("nps {}", report.nodes * NANOS_PER_SEC / report.elapsed().as_nanos()),
            format!("time {}", report.elapsed().as_millis()),
        ];

        if let Some((moves, score)) = &report.pv {
            info.push(format!(
                "score cp {} pv {}",
                score * 100,
                moves
                    .iter()
                    .map(|mv| format!("{mv}"))
                    .collect::<Vec<String>>()
                    .join(" ")
            ));

            self.best_move.set(Some(moves[0]));
        }

        println!("info {}", info.join(" "));
    }
}
