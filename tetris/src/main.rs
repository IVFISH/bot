mod board;
mod bot;
mod communications;
mod errors;
mod game;
mod human;
mod placement;
mod players;
mod population;
mod queue;

use crate::bot::*;
use crate::game::Game;
use crate::human::*;
use crate::placement::piece_data::*;
use crate::players::*;
use crate::population::Population;


fn main() {
    // let mut population = Population::new(100);
    // population.train(1000, 100, 100);
    // communications::init();
    bot_play();
    // bot_debug();
    // println!("process has ended.");
}
