use self::{
    report::{Report, Reporter},
    stopper::Stopper,
};
use crate::eval::*;
use crate::movegen::{Move, MoveList};
use crate::position::Position;

pub mod report;
pub mod stopper;
pub mod tt;

mod alphabeta;
mod quiescence;

const MAX_DEPTH: u8 = u8::MAX;

pub fn search(pos: &mut Position, reporter: &impl Reporter, stopper: &impl Stopper) {
    let mut pv = MoveList::new();
    let mut tt = tt::Table::with_mb(tt::size_mb());
    let mut report = Report::new();

    let max_depth = match stopper.max_depth() {
        Some(depth) => depth,
        None => MAX_DEPTH,
    };

    for depth in 1..=max_depth {
        report.depth = depth;

        let eval = alphabeta::search(pos, depth, EVAL_MIN, EVAL_MAX, &mut pv, &mut tt, &mut report, stopper);

        if stopper.should_stop(&report) {
            break;
        }

        report.pv = Some((pv.clone(), eval));
        report.tt_stats = (tt.usage, tt.capacity);
        reporter.send(&report);
    }
}

fn order_moves(moves: &mut [Move]) {
    moves.sort_unstable_by(|a, b| {
        // 1) Captures before quiets
        let a_is_capture = a.captured_piece.is_some();
        let b_is_capture = b.captured_piece.is_some();

        if a_is_capture != b_is_capture {
            return b_is_capture.cmp(&a_is_capture);
        }

        if a_is_capture {
            // 2) Both are captures, higher weighted victim first (MVV)
            let a_victim = material::PIECE_WEIGHTS[a.captured_piece.unwrap()];
            let b_victim = material::PIECE_WEIGHTS[b.captured_piece.unwrap()];

            if a_victim != b_victim {
                return b_victim.cmp(&a_victim);
            }

            // 3) Same victim, lower weighted attacker first (LVA)
            let a_attacker = material::PIECE_WEIGHTS[a.piece];
            let b_attacker = material::PIECE_WEIGHTS[b.piece];

            return a_attacker.cmp(&b_attacker);
        }

        std::cmp::Ordering::Equal
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::piece::Piece;
    use crate::position::CastlingRights;
    use crate::square::Square;
    use doubles::*;

    #[test]
    fn report_the_depth() {
        let reporter = ReporterSpy::new();

        search(&mut Position::startpos(), &reporter, &StopperStub(3));

        assert_eq!(vec![1, 2, 3], reporter.depths());
    }

    #[test]
    fn report_the_principal_variation() {
        let reporter = ReporterSpy::new();

        search(&mut Position::startpos(), &reporter, &StopperStub(1));

        assert!(!reporter.last_pv_moves().is_empty());
    }

    #[test]
    fn report_the_node_count() {
        let reporter = ReporterSpy::new();

        search(&mut Position::startpos(), &reporter, &StopperStub(1));

        assert_eq!(21, reporter.last_nodes());
    }

    #[test]
    fn report_the_number_of_moves_until_mate() {
        let mut pos = parse_fen("8/8/4R3/8/k1K5/8/8/8 w - - 0 1");
        let reporter = ReporterSpy::new();

        search(&mut pos, &reporter, &StopperStub(1));

        assert_eq!(reporter.last_moves_until_mate(), Some(1));
    }

    #[test]
    fn order_captures_by_mvv_lva_and_before_quiets() {
        let quiet_move = make_move(Piece::WP, parse_square("c4"), parse_square("c5"), None);
        let pawn_captures_pawn = make_move(Piece::WP, parse_square("c4"), parse_square("b5"), Some(Piece::BP));
        let pawn_captures_queen = make_move(Piece::WP, parse_square("c4"), parse_square("d5"), Some(Piece::BQ));
        let knight_captures_bishop = make_move(Piece::WN, parse_square("f4"), parse_square("d3"), Some(Piece::BB));
        let knight_captures_queen = make_move(Piece::WN, parse_square("f4"), parse_square("d5"), Some(Piece::BQ));
        let knight_captures_rook = make_move(Piece::WN, parse_square("f4"), parse_square("g6"), Some(Piece::BR));
        let knight_captures_knight = make_move(Piece::WN, parse_square("f4"), parse_square("h3"), Some(Piece::BN));

        let mut moves = [
            quiet_move,
            pawn_captures_pawn,
            pawn_captures_queen,
            knight_captures_bishop,
            knight_captures_queen,
            knight_captures_rook,
            knight_captures_knight,
        ];

        order_moves(&mut moves);

        assert_eq!(
            moves,
            [
                pawn_captures_queen,
                knight_captures_queen,
                knight_captures_rook,
                knight_captures_bishop,
                knight_captures_knight,
                pawn_captures_pawn,
                quiet_move,
            ],
        );
    }

    fn make_move(piece: Piece, from: Square, to: Square, captured_piece: Option<Piece>) -> Move {
        Move {
            piece,
            from,
            to,
            captured_piece,
            promotion_piece: None,
            castling_rights: CastlingRights::none(),
            half_move_clock: 0,
            is_en_passant: false,
        }
    }

    fn parse_fen(str: &str) -> Position {
        let pos = str.parse();
        assert!(pos.is_ok());

        pos.unwrap()
    }

    fn parse_square(str: &str) -> Square {
        let square = str.parse();
        assert!(square.is_ok());

        square.unwrap()
    }

    mod doubles {
        use super::*;
        use std::cell::{Cell, RefCell};

        pub struct ReporterSpy {
            depths: RefCell<Vec<u8>>,
            last_pv_moves: RefCell<MoveList>,
            last_nodes: Cell<u128>,
            last_moves_until_mate: Cell<Option<u8>>,
        }

        impl ReporterSpy {
            pub fn new() -> Self {
                Self {
                    depths: RefCell::new(vec![]),
                    last_pv_moves: RefCell::new(MoveList::new()),
                    last_nodes: Cell::new(0),
                    last_moves_until_mate: Cell::new(None),
                }
            }

            pub fn depths(&self) -> Vec<u8> {
                self.depths.borrow().clone()
            }

            pub fn last_pv_moves(&self) -> MoveList {
                self.last_pv_moves.borrow().clone()
            }

            pub fn last_nodes(&self) -> u128 {
                self.last_nodes.get()
            }

            pub fn last_moves_until_mate(&self) -> Option<u8> {
                self.last_moves_until_mate.get()
            }
        }

        impl Reporter for ReporterSpy {
            fn send(&self, report: &Report) {
                self.depths.borrow_mut().push(report.depth);
                self.last_nodes.set(report.nodes);
                self.last_moves_until_mate.set(report.moves_until_mate());

                if let Some((moves, _)) = &report.pv {
                    *self.last_pv_moves.borrow_mut() = moves.clone();
                }
            }
        }

        pub struct StopperStub(pub u8);

        impl Stopper for StopperStub {
            fn should_stop(&self, report: &Report) -> bool {
                report.depth > self.0
            }

            fn max_depth(&self) -> Option<u8> {
                Some(self.0)
            }
        }
    }
}
