use crate::colour::Colour;
use crate::position::Board;

mod material;
mod mobility;
mod pawns;
mod psqt;

pub use material::PIECE_WEIGHTS;

pub const TERMS: [fn(Colour, &Board) -> EvalTerm; 4] = [material::eval, mobility::eval, pawns::eval, psqt::eval];

#[derive(Debug, Clone, Copy)]
pub struct EvalTerm(i32, i32);

impl EvalTerm {
    #[inline(always)]
    pub const fn new(mg: i32, eg: i32) -> Self {
        Self(mg, eg)
    }

    #[inline(always)]
    pub const fn zero() -> Self {
        Self(0, 0)
    }

    #[inline(always)]
    pub const fn unphased(eval: i32) -> Self {
        Self(eval, eval)
    }

    #[inline(always)]
    pub const fn mg(self) -> i32 {
        self.0
    }

    #[inline(always)]
    pub const fn eg(self) -> i32 {
        self.1
    }
}

impl std::ops::Add for EvalTerm {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.mg() + rhs.mg(), self.eg() + rhs.eg())
    }
}

impl std::ops::Sub for EvalTerm {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.mg() - rhs.mg(), self.eg() - rhs.eg())
    }
}
