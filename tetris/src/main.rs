mod board;
mod bot;
mod communications;
mod constants;
mod game;
mod human;
mod piece;
mod players;
mod point_vector;
mod population;
mod queue;
mod versus;
mod weight;

use crate::bot::*;
use crate::players::Player;
use std::{thread, time};

fn main() {
    bot_play();
}

fn bot_play() {
    let mut bot = Bot::default();
    let mut pieces_counter = 0;
    let mut time = 0;

    while !bot.get_game().get_game_over() && pieces_counter < 10000 {
        // println!("{}", bot.get_game());

        let now = time::Instant::now();
        bot.make_move();
        time += now.elapsed().as_micros();

        // thread::sleep(time::Duration::from_millis(100));
        pieces_counter += 1;
    }
    println!(
        "Making {} moves took {} microseconds on average",
        pieces_counter,
        time / pieces_counter
    );
    println!("{}", bot.get_game());
}
