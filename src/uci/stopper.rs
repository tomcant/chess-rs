use crate::search::{Report, Stopper};
use std::sync::mpsc::Receiver;
use std::time::Duration;

const STOPPER_NODES_MASK: u128 = 255;

pub struct UciStopper<'a> {
    depth: Option<u8>,
    elapsed: Option<Duration>,
    nodes: Option<u128>,
    stop_signal_recv: &'a Receiver<bool>,
    has_stop_signal: bool,
}

impl<'a> UciStopper<'a> {
    pub fn new(stop_signal_recv: &'a Receiver<bool>) -> Self {
        Self {
            depth: None,
            elapsed: None,
            nodes: None,
            stop_signal_recv,
            has_stop_signal: false,
        }
    }

    pub fn at_depth(&self, depth: Option<u8>) -> Self {
        Self { depth, ..*self }
    }

    pub fn at_elapsed(&self, elapsed: Option<Duration>) -> Self {
        Self { elapsed, ..*self }
    }

    pub fn at_nodes(&self, nodes: Option<u128>) -> Self {
        Self { nodes, ..*self }
    }

    pub fn clear_stop_signal(&self) {
        while self.stop_signal_recv.try_recv().is_ok() {}
    }
}

impl<'a> Stopper for UciStopper<'a> {
    fn should_stop(&mut self, report: &Report) -> bool {
        if self.has_stop_signal {
            return true;
        }

        if report.nodes & STOPPER_NODES_MASK != 0 {
            return false;
        }

        if self.stop_signal_recv.try_recv().is_ok() {
            self.has_stop_signal = true;
            return true;
        }

        match (self.elapsed, self.nodes) {
            (Some(elapsed), _) if report.elapsed() > elapsed => true,
            (_, Some(nodes)) if report.nodes > nodes => true,
            _ => false,
        }
    }

    fn max_depth(&self) -> Option<u8> {
        self.depth
    }
}
