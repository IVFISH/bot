#![allow(dead_code)]
#![allow(unused_imports)]

mod board;
mod book;
mod bot;
mod communications;
mod constants;
mod game;
mod piece;
mod players;
mod point_vector;
mod population;
mod queue;
mod versus;
mod weight;
mod opener;

use crate::bot::*;
use crate::players::Player;
use std::{thread, time};
use std::collections::VecDeque;
use crate::board::Board;
use crate::constants::types::Dependencies;
use crate::constants::versus_constants::AttackType::T;
use crate::game::Game;
use crate::piece::Piece;
use crate::weight::Weights;
use crate::point_vector::Point;
use crate::opener::*;

fn main() {
    // bot_play();
    // tetrio_play();

}

fn bot_train() {

}

fn bot_play() {
    let mut bot = Bot::default();
    println!("{}", bot.get_game().board.get_arr().len());

    let mut time = 0;
    while !bot.get_game().get_game_over() && bot.get_game().game_data.pieces_placed < 10000 {

        let now = time::Instant::now();
        bot.make_move();
        time += now.elapsed().as_micros();

        thread::sleep(time::Duration::from_millis(0));
        // println!("{}", bot.get_game());
        println!("{}", bot.get_game());

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
