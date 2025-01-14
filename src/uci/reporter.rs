use super::r#move::UciMove;
use crate::search::report::{Report, Reporter};
use std::cell::Cell;

const NANOS_PER_SEC: u128 = 1_000_000_000;

pub struct UciReporter {
    best_move: Cell<Option<UciMove>>,
}

impl UciReporter {
    pub fn new() -> Self {
        Self {
            best_move: Cell::new(None),
        }
    }

    pub fn best_move(&self) -> Option<UciMove> {
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

        if let Some((moves, eval)) = &report.pv {
            if let Some(plies) = report.moves_until_mate() {
                let m = if plies % 2 == 0 { plies / 2 } else { (plies + 1) / 2 };
                info.push(format!("score mate {}", m as i32 * eval.signum()));
            } else {
                info.push(format!("score cp {}", eval * 100));
            }

            info.push(format!(
                "pv {}",
                moves
                    .iter()
                    .map(|mv| format!("{}", UciMove::from(*mv)))
                    .collect::<Vec<String>>()
                    .join(" ")
            ));

            self.best_move.set(Some(moves[0].into()));
        }

        println!("info {}", info.join(" "));
    }
}
