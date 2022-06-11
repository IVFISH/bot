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

    let mut game = Game::default();

    println!("{}", game);

    for _ in 0..15 {
        game.piece_down();
    }
    game.set_piece();

    println!("{}", game);

    for _ in 0..3 {
        game.piece_right();
    }
    game.piece_rotate_ccw();

    game.set_piece();
    println!("{}", game);

}
