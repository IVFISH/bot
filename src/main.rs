mod board;
mod bot;
mod command;
mod constants;
mod controller;
mod game;
mod lookahead;
mod piece;
mod placement;
mod test_api;
mod piece_queue;

use crate::bot::*;
use crate::lookahead::many_lookahead;
use std::time::Instant;

#[allow(dead_code)]
fn bench() {
    let bot = Bot::new();
    let n = 500_000;

    let now = Instant::now();
    for _ in 0..n {
        bot.move_gen_1d();
    }
    println!("Averaged {} microseconds", now.elapsed().as_micros() / n);
}

fn main() {
    let bot = Bot::new();
    //bot.game.active = Piece::new(PIECE_T);
    println!("{:?}", bot.game);
    let now = Instant::now();
    let final_games = many_lookahead(bot.game, 4);
    //for game in &final_games {
       // println!("{}", game.board);
    //}
    println!("took {} ms", now.elapsed().as_millis());
    println!("found {} placements", final_games.len());
}
