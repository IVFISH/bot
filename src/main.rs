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
    let n = 10;

    let now = Instant::now();
    for _ in 0..n {
        bot.move_gen(4);
    }
    println!("Averaged {} ms", now.elapsed().as_millis() / n);
}

#[allow(unused)]
fn test() {
    let bot = Bot::<NoPruner>::with_seed(4);
    let d = 2;
    let queue: Vec<u8> = (0..d + 1).map(|x| bot.game.queue.peek_ahead(x)).collect();
    println!("{:?}", queue);
    let now = Instant::now();
    let movegen = bot.move_gen(d.into()).placements;
    let mut placements = movegen.iter();
    let t = now.elapsed().as_millis();
    println!("time elapsed: {} ms", t);
    println!("placements generated: {}", placements.clone().count());
    println!(
        "placements per ms: {}",
        placements.clone().count() / t as usize
    );
    //println!("{}", placements.next().unwrap().game.board);
}

fn main() {
    //bench();
    test();
    //server::init();
}
