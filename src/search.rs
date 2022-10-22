use crate::attacks::is_in_check;
use crate::eval::{Evaluator, EVAL_MAX, EVAL_MIN, EVAL_STALEMATE};
use crate::movegen::MoveGenerator;
use crate::position::Position;
use crate::r#move::Move;
use std::time::{Duration, Instant};

pub trait Report {
    fn best_move(&mut self, mv: Move, eval: i32, depth: u8);
    fn elapsed_time(&mut self, time: Duration);
}

pub fn search(pos: &mut Position, depth: u8, report: &mut dyn Report) {
    let start = Instant::now();
    let mut best_eval = EVAL_MIN;

    for mv in pos.generate_moves() {
        pos.do_move(&mv);

        if !is_in_check(pos.opponent_colour(), &pos.board) {
            let eval = -alpha_beta(pos, depth - 1, EVAL_MIN, EVAL_MAX);

            if eval > best_eval {
                best_eval = eval;
                report.best_move(mv, eval, depth);
            }
        }

        pos.undo_move(&mv);
        report.elapsed_time(start.elapsed());
    }
}

fn alpha_beta(pos: &mut Position, depth: u8, mut alpha: i32, beta: i32) -> i32 {
    if depth == 0 {
        return pos.evaluate();
    }

    let mut has_legal_move = false;

    // todo: sort moves

    for mv in pos.generate_moves() {
        pos.do_move(&mv);

        if !is_in_check(pos.opponent_colour(), &pos.board) {
            has_legal_move = true;

            let eval = -alpha_beta(pos, depth - 1, -beta, -alpha);

            if eval >= beta {
                alpha = beta;
                pos.undo_move(&mv);
                break;
            }

            if eval > alpha {
                alpha = eval;
            }
        }

        pos.undo_move(&mv);
    }

    if has_legal_move {
        return alpha;
    }

    if is_in_check(pos.colour_to_move, &pos.board) {
        return EVAL_MIN;
    }

    EVAL_STALEMATE
}

#[cfg(test)]
mod tests {
    use super::*;
    use doubles::ReportSpy;

    #[test]
    fn report_a_best_move() {
        let mut pos = Position::startpos();
        let mut report = ReportSpy::new();

        search(&mut pos, 1, &mut report);

        assert!(report.last_best_move.is_some());
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
            pub last_best_move: Option<Move>,
            pub last_elapsed_time: Duration,
        }

        impl ReportSpy {
            pub fn new() -> Self {
                Self {
                    last_best_move: None,
                    last_elapsed_time: Duration::ZERO,
                }
            }
        }

        impl Report for ReportSpy {
            fn best_move(&mut self, mv: Move, _eval: i32, _depth: u8) {
                self.last_best_move = Some(mv);
            }

            fn elapsed_time(&mut self, time: Duration) {
                self.last_elapsed_time = time;
            }
        }
    }
}
