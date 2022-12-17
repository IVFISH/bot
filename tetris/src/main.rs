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
use crate::board::Board;
use crate::constants::versus_constants::AttackType::T;
use crate::game::Game;
use crate::piece::Piece;
use crate::weight::Weights;
use crate::point_vector::Point;

fn main() {
    // bot_play();
    tetrio_play()
}

fn bot_play() {
    let mut bot = Bot::default();
    println!("{}", bot.get_game().board.get_arr().len());

    // let mut time = 0;
    // println!("{}", bot.get_game());
    // while !bot.get_game().get_game_over() && bot.get_game().game_data.pieces_placed < 10000 {
    //     println!("{}", bot.get_game());
    //     println!("tslot: {}", bot.get_game().board.t_slot());
    //     println!("sent: {}", bot.get_game().game_data.lines_sent);
    //
    //     let now = time::Instant::now();
    //     bot.make_move();
    //     time += now.elapsed().as_micros();
    //
    //     thread::sleep(time::Duration::from_millis(0));
    //     // println!("{}", bot.get_game());
    // }
    // println!(
    //     "Making {} moves took {} microseconds on average",
    //     bot.get_game().game_data.pieces_placed,
    //     time / (bot.get_game().game_data.pieces_placed as u128)
    // );
    // println!("{}", bot.get_game());
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
