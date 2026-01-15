use crate::piece::Piece;
use crate::position::Board;

pub const MAX_PHASE: i32 = 24;

pub fn phase(board: &Board) -> i32 {
    let knights = board.count_pieces(Piece::WN) + board.count_pieces(Piece::BN);
    let bishops = board.count_pieces(Piece::WB) + board.count_pieces(Piece::BB);
    let rooks = board.count_pieces(Piece::WR) + board.count_pieces(Piece::BR);
    let queens = board.count_pieces(Piece::WQ) + board.count_pieces(Piece::BQ);

    MAX_PHASE.min((knights + bishops + 2 * rooks + 4 * queens) as i32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::position::Position;
    use crate::testing::*;

    #[test]
    fn startpos_has_max_phase() {
        let pos = Position::startpos();
        assert_eq!(phase(&pos.board), MAX_PHASE);
    }

    #[test]
    fn kings_only_has_zero_phase() {
        let pos = parse_fen("4k3/8/8/8/8/8/8/4K3 w - - 0 1");
        assert_eq!(phase(&pos.board), 0);
    }
}
