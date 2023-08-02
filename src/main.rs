mod board;
mod bot;
mod command;
mod constants;
mod controller;
mod game;
mod piece;
mod placement;
mod test_api;

use crate::bot::*;
use crate::constants::piece_constants::*;
use crate::piece::*;
use std::time::Instant;

fn bench() {
    let bot = Bot::new();
    let n = 500_000;

    let now = Instant::now();
    for _ in 0..n {
        bot.move_gen();
    }
    println!("Averaged {} microseconds", now.elapsed().as_micros() / n);
}

fn main() {
    let mut bot = Bot::new();
    bot.game.active = Piece::new(PIECE_T);
    let placements = bot.move_gen();

    let placement = &placements.placements[15];
    println!("{:?}", placement.piece);
    println!("{:?}", placement.get_command_sequence());
}
