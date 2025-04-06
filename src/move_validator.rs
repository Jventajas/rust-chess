use crate::board::Board;
use crate::move_::Move;
use crate::types::Color;

const KING_MOVES: [u64; 64] = {
    let mut moves = [0u64; 64];
    for sq in 0..64 {
        let bit = 1u64 << sq;
        moves[sq] = (bit << 8 & !0xFF) | (bit >> 8 & !0xFF00000000000000) |
            (bit << 1 & !0x0101010101010101) | (bit >> 1 & !0x8080808080808080) |
            (bit << 9 & !0xFF) | (bit >> 9 & !0xFF00000000000000) |
            (bit << 7 & !0xFF) | (bit >> 7 & !0xFF00000000000000);
    }
    moves
};

pub(crate) struct MoveValidator {

}

impl MoveValidator {

    pub(crate) fn new() -> Self {
        Self {

        }
    }

    pub(crate) fn get_legal_moves(&self, board: &Board, color: Color) -> Vec<Move> {
        self.get_pseudo_legal_moves(board, color)
            .iter()
            .filter(|move_| self.is_move_legal(board, move_))
            .collect()
    }

    pub(crate) fn is_move_legal(&self, board: &Board, move_: &Move) -> bool {
        false
    }

    fn get_pseudo_legal_moves(&self, board: &Board, color: Color) -> Vec<Move> {
        Vec::new()
    }


    fn get_pseudo_legal_king_moves(&self, board: &Board, color: Color) -> Vec<Move> {
        let bitboard = match color {
            Color::White => { board.white_king }
            Color::Black => { board.black_king }
        };

        let king_square = bitboard.leading_zeros() as u8;

        let king_moves_bitboard = KING_MOVES[king_square];
        let king_moves_squares = self.bitboard_to_squareset(king_moves_bitboard);

        king_moves_squares
            .iter()
            .map(
                |sq| {
                    Move::new(king_square, *sq, None, false)
                }
            ).collect()
    }

    fn bitboard_to_squareset(&self, bitboard: u64) -> Vec<u8> {
        (0..64).filter(|&i| (bitboard & (1u64 << i)) != 0).collect()
    }





}