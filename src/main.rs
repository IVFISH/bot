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

use crate::bot::*;
use crate::placement_list::*;
use crate::pruner::*;
use crate::test_api::functions::*;
use rayon::prelude::ParallelIterator;
use std::time::Instant;

#[allow(unused)]
fn play() {
    let mut bot = Bot::<SimplePruner>::new();
    bot.game.board = versus_board_tall();
    let n = 25;

    let mut t = 0;

    for _ in 0..n {
        println!("{}", bot.game);

        let now = Instant::now();
        bot.r#do(8);
        t += now.elapsed().as_millis();
        println!(
            "Spent {} ms on movegen and execution",
            now.elapsed().as_millis(),
        );
    }
    println!("------------------------\n{}", bot.game);
    println!("Averaged {} ms", t / n);
}

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
    let d = 4;
    let queue: Vec<u8> = (0..d + 1).map(|x| bot.game.queue.peek_ahead(x)).collect();
    println!("{}, {:?}", bot.game.active.r#type, queue);
    let now = Instant::now();
    let mut placements = bot.move_gen(d.into()).into_placements().collect::<Vec<_>>();

    let t = now.elapsed().as_millis();
    println!("time elapsed: {} ms", t);
    println!("placements generated: {}", placements.len());
    println!("placements per ms: {}", placements.len() as u128 / (t + 1));

    for placement in placements {
        //println!("{}", placement.game.board);
    }
    //println!("{}", placements.next().unwrap().game.board);
}

fn main() {
    // bench();
    // test();
    play();
    //server::init();
}
