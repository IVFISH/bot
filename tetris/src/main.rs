#![allow(dead_code)]

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
    // tetrio_play()
}

fn bot_play() {
    let mut bot = Bot::default();
    let mut time = 0;
    println!("{}", bot.get_game());
    while !bot.get_game().get_game_over() && bot.get_game().game_data.pieces_placed < 10000 {
        println!("{}", bot.get_game());
        println!("{}", bot.get_game().board.t_slot());

        let now = time::Instant::now();
        bot.make_move();
        time += now.elapsed().as_micros();

        thread::sleep(time::Duration::from_millis(0));
        // println!("{}", bot.get_game());
    }
    println!(
        "Making {} moves took {} microseconds on average",
        bot.get_game().game_data.pieces_placed,
        time / (bot.get_game().game_data.pieces_placed as u128)
    );
    println!("{}", bot.get_game());
}

fn tetrio_play() {
    communications::init()
}
