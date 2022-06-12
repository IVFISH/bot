extern crate core;

mod board;
mod placement;
mod errors;
mod game;
mod queue;
mod players;

use game::Game;
use board::Board;
use crate::placement::{MoveVector, Placement, Point};

fn main() {
    let mut game = Game::new(None);

    for _ in 0..5 {
        println!("{}", game);
        game.piece_hard_drop();
    }
}
