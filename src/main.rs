mod board;
mod bot;
mod command;
mod constants;
mod controller;
mod piece;
mod game;
mod placement;
mod test_api;
mod lookahead;

use crate::bot::*;
use crate::game::Game;
use crate::board::Board;
use crate::constants::piece_constants::*;
use crate::piece::*;
use crate::lookahead::many_lookahead;
use std::time::{Instant, SystemTime};

fn bench() {
    let bot = Bot::new();
    let n = 500_000;

    let now = Instant::now();
    for _ in 0..n {
        bot.move_gen();
    }
    println!("Averaged {} microseconds", now.elapsed().as_micros() / n);
}

fn main() {
    let mut bot = Bot::new();
    bot.game.active = Piece::new(PIECE_T);
    println!("{:?}", bot);
    let now = Instant::now();
    let final_games = many_lookahead(bot.game, 2);
    for game in &final_games {
        println!("{}", game.board);
    }
    println!("{}", now.elapsed().as_secs());
    println!("{}", final_games.len());
}
