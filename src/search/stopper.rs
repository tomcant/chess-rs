use super::report::Report;
use std::cell::Cell;
use std::sync::mpsc::Receiver;
use std::time::Duration;

const STOPPER_NODES_MASK: u128 = 255;

pub struct Stopper<'a> {
    pub depth: Option<u8>,
    elapsed: Option<Duration>,
    eval: Option<i32>,
    nodes: Option<u128>,
    signal_recv: &'a Receiver<bool>,
    has_signal: Cell<bool>,
}

impl<'a> Stopper<'a> {
    pub fn new(signal_recv: &'a Receiver<bool>) -> Self {
        Self {
            depth: None,
            elapsed: None,
            eval: None,
            nodes: None,
            signal_recv,
            has_signal: Cell::new(false),
        }
    }

    pub fn at_depth(&mut self, depth: Option<u8>) {
        self.depth = depth;
    }

    pub fn at_elapsed(&mut self, elapsed: Option<Duration>) {
        self.elapsed = elapsed;
    }

    pub fn at_eval(&mut self, eval: Option<i32>) {
        self.eval = eval;
    }

    pub fn at_nodes(&mut self, nodes: Option<u128>) {
        self.nodes = nodes;
    }

    pub fn should_stop(&self, report: &Report) -> bool {
        if self.has_signal.get() {
            return true;
        }

        if report.nodes & STOPPER_NODES_MASK != 0 {
            return false;
        }

        if self.signal_recv.try_recv().is_ok() {
            self.has_signal.set(true);
            return true;
        }

        if let Some(elapsed) = self.elapsed
            && report.elapsed() > elapsed
        {
            return true;
        }

        if let Some(eval) = self.eval
            && report.eval().unwrap_or(0).abs() >= eval
        {
            return true;
        }

        if let Some(nodes) = self.nodes
            && report.nodes > nodes
        {
            return true;
        }

        false
    }
}
