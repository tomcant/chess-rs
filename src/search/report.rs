use super::*;
use std::time::{Duration, Instant};

pub struct Report {
    pub depth: u8,
    pub ply: u8,
    pub nodes: u128,
    pub pv: Option<(MoveList, i32)>,
    pub tt_usage: usize,
    started_at: Instant,
}

#[allow(clippy::new_without_default)]
impl Report {
    pub fn new() -> Self {
        Self {
            depth: 0,
            ply: 0,
            nodes: 0,
            pv: None,
            tt_usage: 0,
            started_at: Instant::now(),
        }
    }

    pub fn elapsed(&self) -> Duration {
        self.started_at.elapsed()
    }

    pub fn moves_until_mate(&self) -> Option<u8> {
        let (_, eval) = self.pv.clone()?;

        if eval.abs() < EVAL_MATE_THRESHOLD || eval.abs() > EVAL_MATE {
            return None;
        }

        Some((EVAL_MATE - eval.abs()) as u8)
    }
}

pub trait Reporter {
    fn send(&self, report: &Report);
}
