#![allow(dead_code)]

mod board;
mod book;
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
    tetrio_play()
    // more_test();
    // ndt_test();
}

fn more_test() {
    let mut bot = Bot::default();
    bot.get_game_mut().active_piece = Piece::new(2);
    bot.get_game_mut().piece_queue.set_queue(VecDeque::from([0, 3, 4, 5, 6, 1]));
    println!("{}", bot);
    bot.make_n_moves(6);
    println!("{}", bot);
    bot.make_move();
    println!("{}", bot);


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

// fn bot_play() {
//     let mut bot = Bot::default();
//     let mut game = bot.get_game_mut();
//     game.set_active_piece(Piece::new(6));
//     println!("{}", game);
//     game.board.add(0,0);
//     game.board.add(1, 0);
//     game.board.add(2, 0);
//     game.board.add(0, 1);
//     game.board.add(0, 3);
//     game.board.add(2, 3);
//     game.board.add(0, 4);
//     game.board.add(1, 4);
//     game.board.add(2, 4);
//     game.board.add(0, 5);
//     game.board.add(1, 5);
//     game.board.add(2, 5);
//     game.board.add(0, 6);
//     game.board.add(1, 6);
//     game.board.add(2, 6);
//     game.board.add(0, 7);
//     game.board.add(1, 7);
//     game.board.add(2, 7);
//     game.board.add(0, 8);
//     game.board.add(1, 8);
//     game.board.add(2, 8);
//     game.board.add(0, 9);
//     game.board.add(1, 9);
//     game.board.add(2, 9);
//     println!("{}", game);
//     let placements = Bot::move_placement_score_1d(game, &Default::default()).1;
//     for placement in placements{
//         game.board.add_list(placement.abs_locations().unwrap());
//         println!("{}", game);
//         game.board.remove_list(placement.abs_locations().unwrap());
//     }
// }

fn tetrio_play() {
    communications::init()
}
