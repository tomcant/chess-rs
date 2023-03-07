use crate::r#move::Move;
use std::time::{Duration, Instant};

pub struct Report {
    pub depth: u8,
    pub nodes: u128,
    pub pv: Option<(Vec<Move>, i32)>,
    started_at: Instant,
}

impl Report {
    pub fn new() -> Self {
        Self {
            depth: 0,
            nodes: 0,
            pv: None,
            started_at: Instant::now(),
        }
    }

    pub fn elapsed(&self) -> Duration {
        self.started_at.elapsed()
    }
}

pub trait Reporter {
    fn send(&self, report: &Report);
}
