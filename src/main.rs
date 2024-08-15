mod board;
mod bot;
mod command;
mod constants;
mod controller;
mod evaluator;
mod game;
mod piece;
mod piece_queue;
mod placement;
mod placement_list;
mod pruner;
mod server;
mod suggestion;
mod test_api;

use evaluator::*;

use crate::bot::*;
use crate::pruner::*;
use crate::test_api::functions::*;
use std::time::Instant;

#[allow(unused)]
fn play() {
    let mut bot = Bot::<SimplePruner, SimpleEvaluator>::new();
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
    let bot = Bot::<NoPruner, NoEvaluator>::new();
    let n = 10;

    let now = Instant::now();
    for _ in 0..n {
        bot.move_gen(4);
    }
    println!("Averaged {} ms", now.elapsed().as_millis() / n);
}

#[allow(unused)]
fn test() {
    let bot = Bot::<NoPruner, NoEvaluator>::with_seed(4);
    let d = 4;
    let queue: Vec<u8> = (0..d + 1).map(|x| bot.game.queue.peek_ahead(x)).collect();
    println!("{}, {:?}", bot.game.active.r#type, queue);
    let now = Instant::now();
    let movegen = bot.move_gen(d.into()).placements;
    let mut placements = movegen.iter();
    let t = now.elapsed().as_millis();
    println!("time elapsed: {} ms", t);
    println!("placements generated: {}", placements.clone().count());
    println!(
        "placements per ms: {}",
        placements.clone().count() as u128 / (t + 1)
    );
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
