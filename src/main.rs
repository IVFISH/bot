mod board;
mod bot;
mod command;
mod constants;
mod controller;
mod game;
mod piece;
mod piece_queue;
mod placement;
mod test_api;

use crate::bot::*;
use std::time::Instant;

#[allow(unused)]
fn bench() {
    let bot = Bot::new();
    let n = 500_000;

    let now = Instant::now();
    for _ in 0..n {
        bot.move_gen(1);
    }
    println!("Averaged {} microseconds", now.elapsed().as_micros() / n);
}

fn main() {
    let bot = Bot::new();
    let placements = bot.move_gen(2);

    placements.write_fumens("fumens.txt");

    println!("{}", placements.placements.len());
    let placement = &placements.placements[1];
    println!("{:?}", placement.pieces);
    println!("{:?}", placement.piece);
    println!("{:?}", placement.get_command_sequence());
    let placement = &placements.placements[100];
    println!("{:?}", placement.pieces);
    println!("{:?}", placement.piece);
    println!("{:?}", placement.get_command_sequence());
}
