mod board;
mod bot;
mod command;
mod constants;
mod controller;
mod game;
mod piece;
mod piece_queue;
mod placement;
mod placement_list;
mod pruner;
mod server;
mod suggestion;
mod test_api;

use crate::board::*;
use crate::bot::*;
use crate::game::*;
use crate::piece::*;
use crate::pruner::*;
use crate::test_api::functions::*;
use std::time::Instant;

#[allow(unused)]
fn bench() {
    let bot = Bot::<NoPruner>::new();
    let n = 500_000;

    let now = Instant::now();
    for _ in 0..n {
        bot.move_gen(1);
    }
    println!("Averaged {} microseconds", now.elapsed().as_micros() / n);
}

#[allow(unused)]
fn test() {
    let bot = Bot::<NoPruner>::with_seed(4);
    let movegen = bot.move_gen(1).placements;
    let mut placements = movegen.iter();
    println!("{}", placements.clone().count());
    println!("{}", placements.next().unwrap().game.board);
    println!("{}", placements.next().unwrap().game.board);
}

fn main() {
    server::init();
}
