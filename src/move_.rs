use crate::types::PieceType;

pub(crate) struct Move {
    from: u8,
    to: u8,
    promotion: Option<PieceType>,
    is_castling: bool,
}