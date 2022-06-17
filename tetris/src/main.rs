mod board;
mod bot;
mod communications;
mod errors;
mod game;
mod human;
mod placement;
mod players;
mod queue;

use crate::bot::*;
use crate::human::*;
use crate::players::*;
use crate::placement::piece_data::*;
use crate::game::Game;

fn main() {
    // communications::init();
    bot_play();
    // bot_debug();
    println!("proccess has ended.");
}