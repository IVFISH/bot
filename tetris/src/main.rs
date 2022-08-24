mod board;
mod bot;
mod communications;
mod errors;
mod game;
mod human;
mod placement;
mod players;
mod queue;
mod population;

use crate::bot::*;
use crate::game::Game;
use crate::human::*;
use crate::placement::piece_data::*;
use crate::players::*;

fn main() {
    // communications::init();
    bot_play();
    // bot_debug();
    println!("process has ended.");
}
