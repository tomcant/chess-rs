bitflags::bitflags! {
    pub struct CastlingAbility: u8 {
        const NONE = 0;
        const WHITE_KING = 1;
        const WHITE_QUEEN = 2;
        const BLACK_KING = 4;
        const BLACK_QUEEN = 8;
        const ALL = 15;
    }
}
