extern crate core;

mod board;
mod placement;
mod errors;
mod game;
mod queue;
mod players;
mod bot;

use game::Game;
use board::Board;
use crate::bot::Bot;
use crate::placement::{MoveVector, Placement, Point};
use crate::players::Player;

fn main() {

    let mut bot = Bot::default();

    println!("{}", bot.game);
    for _ in 0..15 {
        bot.make_move();
        println!("{}", bot.game);
    }
    println!("{}", bot.game.active_piece);

}
