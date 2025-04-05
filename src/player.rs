use rand::{Rng};

use crate::move_::Move;


pub trait Player {
    fn select_move<'a>(&self, moves: &'a[Move]) -> &'a Move;
}

#[derive(Default)]
pub struct RandomPlayer;

impl Player for RandomPlayer {
    fn select_move<'a>(&self, moves: &'a[Move]) -> &'a Move {
        let random_index = rand::rng().random_range(0..moves.len());
        &moves[random_index]
    }
}

// todo: Implement CLI player