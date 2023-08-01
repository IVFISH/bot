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
use std::time::Instant;

fn main() {
    let bot = Bot::new();
    let n = 500_000;
    
    let now = Instant::now();
    for _ in 0..n {
        bot.move_gen();
    }
    println!("Averaged {} microseconds", now.elapsed().as_micros() / n);
}
