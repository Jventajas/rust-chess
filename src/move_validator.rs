use crate::board::Board;
use crate::move_::Move;
use crate::types::Color;

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
        Vec::new()
    }





}