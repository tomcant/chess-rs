use crate::attacks::is_in_check;
use crate::eval::{Evaluator, EVAL_CHECKMATE, EVAL_MAX, EVAL_MIN, EVAL_STALEMATE};
use crate::movegen::MoveGenerator;
use crate::position::Position;
use crate::r#move::Move;
use std::time::{Duration, Instant};

pub trait Report {
    fn principal_variation(&mut self, moves: Vec<Move>, eval: i32);
    fn elapsed_time(&mut self, time: Duration);
}

pub fn search(pos: &mut Position, max_depth: u8, report: &mut dyn Report) {
    let start = Instant::now();
    let mut pv = vec![];

    for depth in 1..=max_depth {
        let eval = alpha_beta(pos, depth, EVAL_MIN, EVAL_MAX, &mut pv);

        report.elapsed_time(start.elapsed());
        report.principal_variation(pv.clone(), eval);
    }
}

fn alpha_beta(pos: &mut Position, depth: u8, mut alpha: i32, beta: i32, pv: &mut Vec<Move>) -> i32 {
    if depth == 0 {
        return pos.evaluate();
    }

    let colour_to_move = pos.colour_to_move;
    let mut has_legal_move = false;

    // todo: order moves by PV move and MVV/LVA

    for mv in pos.generate_moves() {
        pos.do_move(&mv);

        if is_in_check(colour_to_move, &pos.board) {
            pos.undo_move(&mv);
            continue;
        }

        has_legal_move = true;

        let mut this_pv = vec![];
        let eval = -alpha_beta(pos, depth - 1, -beta, -alpha, &mut this_pv);

        if eval >= beta {
            pos.undo_move(&mv);
            return beta;
        }

        if eval > alpha {
            alpha = eval;

            pv.clear();
            pv.push(mv);
            pv.append(&mut this_pv);
        }

        pos.undo_move(&mv);
    }

    if has_legal_move {
        return alpha;
    }

    if is_in_check(pos.colour_to_move, &pos.board) {
        return EVAL_CHECKMATE + depth as i32;
    }

    EVAL_STALEMATE
}

#[cfg(test)]
mod tests {
    use super::*;
    use doubles::ReportSpy;

    #[test]
    fn report_a_principal_variation() {
        let mut pos = Position::startpos();
        let mut report = ReportSpy::new();

        search(&mut pos, 1, &mut report);

        assert!(!report.last_pv_moves.is_empty());
    }

    #[test]
    fn report_an_elapsed_time_greater_than_zero() {
        let mut pos = Position::startpos();
        let mut report = ReportSpy::new();

        search(&mut pos, 1, &mut report);

        assert!(report.last_elapsed_time.gt(&Duration::ZERO));
    }

    mod doubles {
        use super::*;

        pub struct ReportSpy {
            pub last_pv_moves: Vec<Move>,
            pub last_elapsed_time: Duration,
        }

        impl ReportSpy {
            pub fn new() -> Self {
                Self {
                    last_pv_moves: vec![],
                    last_elapsed_time: Duration::ZERO,
                }
            }
        }

        impl Report for ReportSpy {
            fn principal_variation(&mut self, moves: Vec<Move>, _eval: i32) {
                self.last_pv_moves = moves;
            }

            fn elapsed_time(&mut self, time: Duration) {
                self.last_elapsed_time = time;
            }
        }
    }
}
