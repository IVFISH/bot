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
use std::{thread, time};
use crate::players::Player;

fn main() {
    bot_play();
}

fn bot_play() {
    let mut bot = Bot::default();

    while !bot.get_game().get_game_over() {
        println!("{}", bot.get_game());
        bot.make_move();
        thread::sleep(time::Duration::from_millis(500));
    }

    println!("{}", bot.get_game());
}

