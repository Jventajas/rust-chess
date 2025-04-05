use crate::board::Board;
use crate::move_::Move;
use crate::move_validator::MoveValidator;
use crate::player::Player;
use crate::types::Color;
use crate::types::GameResult;

struct Game {
    board: Board,
    turn: Color,
    move_history: Vec<Move>,
    result: Option<GameResult>,
    white_player: Box<dyn Player>,
    black_player: Box<dyn Player>,
    move_validator: MoveValidator,
}

impl Game {

    fn new(white_player: Box<dyn Player>, black_player: Box<dyn Player>) -> Self {
        Self {
            board: Board::new(),
            turn: Color::White,
            move_history: Vec::new(),
            result: None,
            white_player,
            black_player,
            move_validator: MoveValidator::new()
        }
    }

    fn get_game_result(&self) -> &Option<GameResult> {
        &self.result
    }

    fn is_over(&self) -> bool {
        self.result.is_some()
    }

    fn get_current_player(&self) -> &Box<dyn Player> {
        match self.turn {
            Color::White => &self.white_player,
            Color::Black => &self.black_player,
        }
    }

    fn get_legal_moves(&self) -> Vec<Move> {
        self.move_validator.get_legal_moves(&self.board, self.turn)
    }

    fn apply_move(&self, move_: &Move) -> Result<(), String> {
        if !self.move_validator.is_move_legal(&self.board, move_) {
            return Err(format!("Illegal move selected: {:?}", move_));
        }

        // todo: implement validation.

        Ok(())
    }
}





