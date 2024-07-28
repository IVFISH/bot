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

use argmin::core::{CostFunction, Error, Executor};
use argmin::solver::particleswarm::ParticleSwarm;
use polynomial::Polynomial;

struct Trainer {}

impl CostFunction for Trainer {
    type Param = Vec<f32>;
    type Output = f32;

    fn cost(&self, param: &Self::Param) -> Result<Self::Output, Error> {
        let mut bot = Bot::with_weights(
            Weights {
                height_weight: Polynomial::new(vec![0.0, param[0]]),
                adjacent_height_differences_weight: Polynomial::new(vec![0.0, param[1]]),
                total_height_difference_weight: Polynomial::new(vec![0.0, param[2]]),
                num_hole_total_weight: Polynomial::new(vec![0.0, param[3]]),
                num_hole_weighted_weight: Polynomial::new(vec![0.0, param[4]]),
                cell_covered_weight: Polynomial::new(vec![0.0, param[5]]),
    
                t_slot_weight: Polynomial::new(vec![0.0, param[6]]),
                b2b_weight: Polynomial::new(vec![0.0, param[7]]),
                combo_weight: Polynomial::new(vec![0.0, param[8]]),
                damage_weight: Polynomial::new(vec![0.0, param[9]]),
                clear_weight: Polynomial::new(vec![0.0, param[10]]),
                
                perfect_clear_weight: param[11],
                tspin_weight: param[12],
            }
        );

        while !bot.get_game().get_game_over() && bot.get_game().game_data.pieces_placed < 1000 {
            bot.make_move();
        }

        let mut out = bot.get_game().game_data.lines_sent as f32 + 0.01 * bot.get_game().game_data.pieces_placed as f32;
        out *= -1.0;
        if bot.get_game().get_game_over() { out += 100.0 }
        
        Ok(out)
    }
}


fn main() {
    if let Err(ref e) = bot_train() {
        println!("{e}");
    }
    // bot_play();
    // tetrio_play();

}

fn bot_train() -> Result<(), Error> {
    let cost_function = Trainer {};

    let solver = ParticleSwarm::new((vec![-4.0, -4.0], vec![4.0, 4.0]), 40);

    let res = Executor::new(cost_function, solver)
        .configure(|state| state.max_iters(100))
        .run()?;

    // Print Result
    println!("{res}");

    Ok(())
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
