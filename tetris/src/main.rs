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
use crate::constants::localbotgameplay::*;
use crate::constants::board_constants::CONSOLE_DISPLAY_STATS;
use crate::game::Game;
use crate::piece::Piece;
use crate::weight::Weights;
use crate::point_vector::Point;
use crate::opener::*;
use colored::Colorize;
use std::fs;
use std::string::*;

fn main() {
    // bot_play();
    bot_play_local();
    // tetrio_play();

    // test_tspinkicks();
    // test_cheese();
    // more_test();
    // dt_test();
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
        println!("{} milliseconds to move", format!("{}", now.elapsed().as_micros() / 1000).green())
    }
    println!(
        "Making {} moves took {} microseconds on average",
        bot.get_game().game_data.pieces_placed,
        time / (bot.get_game().game_data.pieces_placed as u128)
    );
    println!("{}", bot.get_game());
    thread::sleep(time::Duration::from_millis(10000));
}

fn bot_play_local() {
    let mut bot = Bot::default();
    println!("{}", bot.get_game().board.get_arr().len());

    let mut time = 0;
    while !bot.get_game().get_game_over() && bot.get_game().game_data.pieces_placed < 10000 {
        let now = time::Instant::now();
        bot.get_game_mut().update_garbage_amount();
        bot.make_move();
        time += now.elapsed().as_micros();
        
        if ALLOWLOCALGAMEPLAY && bot.get_game().game_data.combo == 0 {
            let mut commsfile = fs::read_to_string(LOCALGAMEPLAYFILEPATH).expect("e");
            let garbage = commsfile.chars().nth(BOTNUM).expect("e").to_string().parse::<usize>().unwrap();
            bot.addgarbage((time % 10).try_into().unwrap(), garbage);
            commsfile.replace_range(BOTNUM..BOTNUM+1, "0");
            let _ = fs::write(LOCALGAMEPLAYFILEPATH, commsfile);
        }

        thread::sleep(time::Duration::from_millis(0));
        // println!("{}", bot.get_game());
        println!("{}", bot.get_game());
        if CONSOLE_DISPLAY_STATS {
            println!("{} milliseconds to move", format!("{}", now.elapsed().as_micros() / 1000).green());
            println!("{} pps, {} apm", format!("{}", (bot.get_game().game_data.pieces_placed as f64 / (time as f64 / 1000000000.0)).round() / 1000.0).green(), format!("{}", (60.0 * bot.get_game().game_data.lines_sent as f64 / (time as f64 / 1000000000.0)).round() / 1000.0).green());
            println!("{} lines sent / {} pieces placed = {} app", format!("{}", bot.get_game().game_data.lines_sent).green(), format!("{}", bot.get_game().game_data.pieces_placed).green(), format!("{}", (bot.get_game().game_data.lines_sent as f64) / (bot.get_game().game_data.pieces_placed as f64)).green());
            println!("{} b2b, {} combo", format!("{}", bot.get_game().game_data.b2b).green(), format!("{}", bot.get_game().game_data.combo).green());
        }
    }
    println!(
        "Making {} moves took {} microseconds on average",
        bot.get_game().game_data.pieces_placed,
        time / (bot.get_game().game_data.pieces_placed as u128)
    );
    println!("{}", bot.get_game());
    thread::sleep(time::Duration::from_millis(100000));
}

fn test_tspinkicks() {
    let mut bot = Bot::default();
    println!("{}", bot.get_game().board.get_arr().len());

    bot.addgarbage(3,3);
    bot.removefromboard(1,2);
    //bot.removefromboard(0,2);
    bot.addtoboard(3,4);
    bot.addtoboard(4,4);
    bot.addtoboard(4,3);

    println!("{}", bot.get_game());

    let mut time = 0;
    while !bot.get_game().get_game_over() && bot.get_game().game_data.pieces_placed < 10000 {
        let now = time::Instant::now();
        bot.make_move();
        time += now.elapsed().as_micros();
        
        if ALLOWLOCALGAMEPLAY && bot.get_game().game_data.combo == 0 {
            let mut commsfile = fs::read_to_string(LOCALGAMEPLAYFILEPATH).expect("e");
            let garbage = commsfile.chars().nth(BOTNUM).expect("e").to_string().parse::<usize>().unwrap();
            bot.addgarbage((time % 10).try_into().unwrap(), garbage);
            commsfile.replace_range(BOTNUM..BOTNUM+1, "0");
            let _ = fs::write(LOCALGAMEPLAYFILEPATH, commsfile);
        }

        thread::sleep(time::Duration::from_millis(0));
        // println!("{}", bot.get_game());
        println!("{}", bot.get_game());
        println!("{}", bot.get_game().game_data.t_spin_type);
        println!("{} milliseconds to move", format!("{}", now.elapsed().as_micros() / 1000).green());
        println!("{} lines sent / {} pieces placed = {} app", format!("{}", bot.get_game().game_data.lines_sent).green(), format!("{}", bot.get_game().game_data.pieces_placed).green(), format!("{}", (bot.get_game().game_data.lines_sent as f64) / (bot.get_game().game_data.pieces_placed as f64)).green());
        println!("{} b2b, {} combo", format!("{}", bot.get_game().game_data.b2b).green(), format!("{}", bot.get_game().game_data.combo).green())
    }
    println!(
        "Making {} moves took {} microseconds on average",
        bot.get_game().game_data.pieces_placed,
        time / (bot.get_game().game_data.pieces_placed as u128)
    );
    println!("{}", bot.get_game());
    thread::sleep(time::Duration::from_millis(100000));
}

fn test_cheese() {
    let mut bot = Bot::default();
    println!("{}", bot.get_game().board.get_arr().len());

    println!("{}", bot.get_game());

    let mut time = 0;
    while !bot.get_game().get_game_over() && bot.get_game().game_data.pieces_placed < 10000 {

        let now = time::Instant::now();
        bot.make_move();
        time += now.elapsed().as_micros();
        if bot.get_game().game_data.pieces_placed % 3 == 0 { bot.addgarbage((time % 10).try_into().unwrap(), 1); } // cheese timer in zen mode be like

        thread::sleep(time::Duration::from_millis(0));
        // println!("{}", bot.get_game());
        println!("{}", bot.get_game());
        println!("{} milliseconds to move", format!("{}", now.elapsed().as_micros() / 1000).green());
        println!("{} pieces placed", format!("{}", bot.get_game().game_data.pieces_placed).green())
    }
    println!(
        "Making {} moves took {} microseconds on average",
        bot.get_game().game_data.pieces_placed,
        time / (bot.get_game().game_data.pieces_placed as u128)
    );
    println!("{}", bot.get_game());
    thread::sleep(time::Duration::from_millis(10000));
}

fn dt_test() {
    let mut bot = Bot::default();
    let game = bot.get_game_mut();
    let dt = vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (0, 1), (1, 1), (4, 1), (6, 2), (0, 3), (1, 3), (3, 3), (4, 3), (5, 3), (6, 3), (0, 4), (1, 4), (2, 4), (3, 4), (4, 4), (5, 4), (6, 4), (0, 5), (1, 5), (2, 5), (3, 5), (4, 5), (5, 5), (0, 6), (1, 6), (2, 6), (3, 6), (4, 6), (5, 6), (0, 7), (1, 7), (2, 7), (3, 7), (4, 7), (5, 7), (6, 7), (0, 8), (1, 8), (2, 8), (3, 8), (4, 8), (0, 9), (1, 9), (2, 9), (3, 9), (4, 9), (5, 9)];
    game.board.add_list(dt.iter().map(|x| Point(x.0,x.1)).collect());
    println!("{}", game);
    game.set_active_piece(Piece::new(6));
    let placements = Bot::move_placement_score_1d(game, &Default::default()).1;
    for placement in placements{
        if placement.center.0 < 6 {
            game.set_active_piece(placement);
            println!("{}", game);
        }
    }
}

fn tetrio_play() {
    communications::init()
}
