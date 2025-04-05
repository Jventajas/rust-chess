use crate::board::Board;
use crate::move_::Move;
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
}

impl Game {

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
        Vec::new()
    }

    fn apply_move(&self, move_: &Move) {

    }
}





