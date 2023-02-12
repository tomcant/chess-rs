use crate::attacks::is_in_check;
use crate::eval::{Evaluator, EVAL_CHECKMATE, EVAL_MAX, EVAL_MIN, EVAL_STALEMATE};
use crate::movegen::MoveGenerator;
use crate::position::Position;
use crate::r#move::Move;
use std::time::{Duration, Instant};

pub struct Report {
    pub depth: u8,
    pub nodes: u128,
    pub pv: Option<(Vec<Move>, i32)>,
    started_at: Instant,
}

impl Report {
    fn new() -> Self {
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
    fn send(&mut self, report: &Report);
}

pub struct Stopper {
    depth: Option<u8>,
    elapsed: Option<Duration>,
    nodes: Option<u128>,
    stopped: bool,
}

impl Stopper {
    pub fn new() -> Self {
        Self {
            depth: None,
            elapsed: None,
            nodes: None,
            stopped: false,
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

    pub fn should_stop(&mut self, report: &Report) -> bool {
        self.stopped = (self.depth.is_some() && report.depth > self.depth.unwrap())
            || (self.elapsed.is_some() && report.elapsed() > self.elapsed.unwrap())
            || (self.nodes.is_some() && report.nodes > self.nodes.unwrap());

        self.stopped
    }
}

pub fn search(pos: &mut Position, reporter: &mut dyn Reporter, stopper: &mut Stopper) {
    let mut pv = vec![];
    let mut report = Report::new();

    for depth in 1.. {
        report.depth = depth;

        let eval = alpha_beta(pos, depth, EVAL_MIN, EVAL_MAX, &mut pv, &mut report, stopper);

        if stopper.stopped {
            break;
        }

        report.pv = Some((pv.clone(), eval));
        reporter.send(&report);
    }
}

fn alpha_beta(
    pos: &mut Position,
    depth: u8,
    mut alpha: i32,
    beta: i32,
    pv: &mut Vec<Move>,
    report: &mut Report,
    stopper: &mut Stopper,
) -> i32 {
    if stopper.should_stop(report) {
        return 0;
    }

    if depth == 0 {
        return quiescence(pos, alpha, beta, &mut vec![], report);
    }

    report.nodes += 1;

    let (pv_move, mut next_ply_pv) = split_pv(pv);
    let colour_to_move = pos.colour_to_move;
    let mut has_legal_move = false;

    for mv in order_moves(&pos.generate_all_moves(), pv_move) {
        pos.do_move(&mv);

        if is_in_check(colour_to_move, &pos.board) {
            pos.undo_move(&mv);
            continue;
        }

        has_legal_move = true;

        let eval = -alpha_beta(pos, depth - 1, -beta, -alpha, &mut next_ply_pv, report, stopper);

        if eval >= beta {
            pos.undo_move(&mv);
            return beta;
        }

        if eval > alpha {
            alpha = eval;

            pv.clear();
            pv.push(*mv);
            pv.append(&mut next_ply_pv);
        }

        pos.undo_move(&mv);
    }

    if !has_legal_move {
        return if is_in_check(colour_to_move, &pos.board) {
            EVAL_CHECKMATE
        } else {
            EVAL_STALEMATE
        };
    }

    alpha
}

fn quiescence(pos: &mut Position, mut alpha: i32, beta: i32, pv: &mut Vec<Move>, report: &mut Report) -> i32 {
    report.nodes += 1;

    let eval = pos.evaluate();

    if eval >= beta {
        return beta;
    }

    if eval > alpha {
        alpha = eval;
    }

    let (pv_move, mut next_ply_pv) = split_pv(pv);
    let colour_to_move = pos.colour_to_move;

    for mv in order_moves(&pos.generate_capture_moves(), pv_move) {
        pos.do_move(&mv);

        if is_in_check(colour_to_move, &pos.board) {
            pos.undo_move(&mv);
            continue;
        }

        let eval = -quiescence(pos, -beta, -alpha, &mut next_ply_pv, report);

        if eval >= beta {
            pos.undo_move(&mv);
            return beta;
        }

        if eval > alpha {
            alpha = eval;

            pv.clear();
            pv.push(*mv);
            pv.append(&mut next_ply_pv);
        }

        pos.undo_move(&mv);
    }

    alpha
}

fn split_pv(pv: &mut [Move]) -> (Option<Move>, Vec<Move>) {
    if let Some((head, tail)) = pv.split_first() {
        (Some(*head), tail.to_vec())
    } else {
        (None, vec![])
    }
}

struct OrderedMove {
    mv: Move,
    order: u8,
}

impl std::ops::Deref for OrderedMove {
    type Target = Move;

    fn deref(&self) -> &Self::Target {
        &self.mv
    }
}

const ORDER_PV_MOVE: u8 = 0;
const ORDER_NON_PV_MOVE: u8 = 1;

fn order_moves(moves: &[Move], pv_move: Option<Move>) -> Vec<OrderedMove> {
    let has_pv_move = pv_move.is_some();

    let mut moves = moves
        .iter()
        .map(|mv| OrderedMove {
            mv: *mv,
            order: if has_pv_move && *mv == pv_move.unwrap() {
                ORDER_PV_MOVE
            } else {
                ORDER_NON_PV_MOVE
            },
        })
        .collect::<Vec<OrderedMove>>();

    if has_pv_move {
        moves.sort_unstable_by_key(|mv| mv.order);
    }

    moves
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::castling::CastlingRights;
    use crate::square::Square;
    use doubles::ReporterSpy;

    #[test]
    fn report_the_depth() {
        let mut pos = Position::startpos();
        let mut reporter = ReporterSpy::new();

        search(&mut pos, &mut reporter, &mut Stopper::new().at_depth(Some(3)));

        assert_eq!(vec![1, 2, 3], reporter.depths);
    }

    #[test]
    fn report_the_principal_variation() {
        let mut pos = Position::startpos();
        let mut reporter = ReporterSpy::new();

        search(&mut pos, &mut reporter, &mut Stopper::new().at_depth(Some(1)));

        assert!(!reporter.last_pv_moves.is_empty());
    }

    #[test]
    fn report_the_node_count() {
        let mut pos = Position::startpos();
        let mut reporter = ReporterSpy::new();

        search(&mut pos, &mut reporter, &mut Stopper::new().at_depth(Some(1)));

        assert_eq!(21, reporter.last_nodes);
    }

    #[test]
    fn order_pv_move_to_front() {
        fn make_move(from: Square, to: Square) -> Move {
            Move {
                from,
                to,
                captured_piece: None,
                promotion_piece: None,
                castling_rights: CastlingRights::none(),
                is_en_passant: false,
            }
        }

        let pv_move = make_move(Square::from_index(0), Square::from_index(1));

        let moves = [
            make_move(Square::from_index(2), Square::from_index(3)),
            make_move(Square::from_index(4), Square::from_index(5)),
            pv_move,
        ];

        let ordered_moves = order_moves(&moves, Some(pv_move));

        assert_eq!(ordered_moves[0].mv, pv_move);
    }

    mod doubles {
        use super::*;

        pub struct ReporterSpy {
            pub depths: Vec<u8>,
            pub last_pv_moves: Vec<Move>,
            pub last_nodes: u128,
        }

        impl ReporterSpy {
            pub fn new() -> Self {
                Self {
                    depths: vec![],
                    last_pv_moves: vec![],
                    last_nodes: 0,
                }
            }
        }

        impl Reporter for ReporterSpy {
            fn send(&mut self, report: &Report) {
                self.depths.push(report.depth);
                self.last_nodes = report.nodes;

                if let Some((moves, _)) = &report.pv {
                    self.last_pv_moves = moves.clone();
                }
            }
        }
    }
}
