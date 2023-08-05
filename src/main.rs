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
    let now = Instant::now();
    let bot = Bot::with_seed(3);
    let placements = bot.move_gen(3);
    println!("Took {} seconds", now.elapsed().as_secs());

    // placements.write_fumens("fumens.txt");

    println!("{}", placements.placements.len());
    let placement = &placements.placements[200];
    println!("{}", placement.game_after);
    let placement = &placements.placements[1000];
    println!("{}", placement.game_after);
    // println!("{:?}", placement.pieces);
    // println!("{:?}", placement.piece);
    // println!("{:?}", placement.get_command_sequence());
    // let placement = &placements.placements[100];
    // println!("{:?}", placement.pieces);
    // println!("{:?}", placement.piece);
    // println!("{:?}", placement.get_command_sequence());
}
