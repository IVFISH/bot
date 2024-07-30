#![allow(dead_code)]
#![allow(unused_imports)]

mod board;
mod book;
mod bot;
mod communications;
mod constants;
mod game;
mod opener;
mod piece;
mod players;
mod point_vector;
mod population;
mod queue;
mod versus;
mod weight;

use crate::board::Board;
use crate::bot::*;
use crate::constants::types::Dependencies;
use crate::constants::versus_constants::AttackType::T;
use crate::game::Game;
use crate::opener::*;
use crate::piece::Piece;
use crate::players::Player;
use crate::point_vector::Point;
use crate::weight::Weights;
use std::collections::VecDeque;
use std::{thread, time};

use argmin::core::observers::ObserverMode;
use argmin::core::{CostFunction, Error, Executor};
use argmin::solver;
use argmin::solver::particleswarm::ParticleSwarm;
use argmin_checkpointing_file::{CheckpointingFrequency, FileCheckpoint};
use argmin_observer_paramwriter::{ParamWriter, ParamWriterFormat};
use argmin_observer_slog::SlogLogger;
use polynomial::Polynomial;
use rand::SeedableRng;

struct Trainer {}

impl CostFunction for Trainer {
    type Param = Vec<f32>;
    type Output = f32;

    fn cost(&self, param: &Self::Param) -> Result<Self::Output, Error> {
        // Simulate without garbage
        let mut bot = Bot::with_weights(Weights::from_params(param), false);

        while !bot.get_game().get_game_over() && bot.get_game().game_data.pieces_placed < 2000 {
            bot.make_move();
        }

        let mut out1 = bot.get_game().game_data.lines_sent as f32
            + 0.001 * bot.get_game().game_data.pieces_placed as f32;
        out1 *= -1.0;
        if bot.get_game().get_game_over() {
            out1 += 800.0; // We really hate dying
        }

        // Simulate with garbage
        bot = Bot::with_weights(Weights::from_params(param), true);
        while !bot.get_game().get_game_over() && bot.get_game().game_data.pieces_placed < 2000 {
            bot.make_move();
        }

        let mut out2 = bot.get_game().game_data.lines_sent as f32
            + 0.001 * bot.get_game().game_data.pieces_placed as f32;
        out2 *= -1.0;
        if bot.get_game().get_game_over() {
            out2 += 500.0; // We hate dying a little less
        }

        //println!("Bot scored {}, {} solo and {} with garbage", out1+out2, out1, out2);
        Ok(out1 + out2)
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
    let solver = ParticleSwarm::new(
        (
            vec![
                -100.0, -100.0, -100.0, -100.0, -100.0, -100.0, -100.0, -100.0, -100.0, -100.0,
                -100.0, -100.0, -100.0, -100.0, -100.0, -100.0, -100.0, -100.0, -100.0, -100.0,
            ],
            vec![
                100.0, 100.0, 100.0, 100.0, 100.0, 100.0, 100.0, 100.0, 100.0, 100.0, 100.0, 100.0,
                100.0, 0.0, 100.0, 100.0, 100.0, 100.0, 100.0, 100.0,
            ],
        ),
        24,
    )
    .with_rng_generator(rand_xoshiro::Xoroshiro128Plus::seed_from_u64(12213328));

    let checkpoint = FileCheckpoint::new(
        ".checkpoints",
        "test3",
        CheckpointingFrequency::Always,
    );

    let res = Executor::new(cost_function, solver)
        .configure(|state| state.max_iters(200))
        .add_observer(SlogLogger::term_noblock(), ObserverMode::Always)
        .add_observer(
            ParamWriter::new("params", "writer", ParamWriterFormat::JSON),
            ObserverMode::NewBest,
        )
        .checkpointing(checkpoint)
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
