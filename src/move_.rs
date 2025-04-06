use crate::types::PieceType;

pub(crate) struct Move {
    from: u8,
    to: u8,
    promotion: Option<PieceType>,
    is_castling: bool,
}

impl Move {
    pub(crate) fn new(from: u8, to: u8, promotion: Option<PieceType>, is_castling: bool) -> Self {
        Self { from, to, promotion, is_castling }
    }
}