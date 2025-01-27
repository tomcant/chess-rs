use super::*;
use std::time::{Duration, Instant};

pub struct Report {
    pub depth: u8,
    pub ply: u8,
    pub nodes: u128,
    pub pv: Option<(MoveList, i32)>,
    started_at: Instant,
}

impl Report {
    const EVAL_CHECKMATE_THRESHOLD: i32 = EVAL_CHECKMATE - MAX_DEPTH as i32;

    pub fn new() -> Self {
        Self {
            depth: 0,
            ply: 0,
            nodes: 0,
            pv: None,
            started_at: Instant::now(),
        }
    }

    pub fn elapsed(&self) -> Duration {
        self.started_at.elapsed()
    }

    pub fn moves_until_mate(&self) -> Option<u8> {
        let Some((_, eval)) = self.pv else {
            return None;
        };

        if eval.abs() < Self::EVAL_CHECKMATE_THRESHOLD || eval.abs() > EVAL_CHECKMATE {
            return None;
        }

        Some((EVAL_CHECKMATE - eval.abs()) as u8)
    }
}

pub trait Reporter {
    fn send(&self, report: &Report);
}
