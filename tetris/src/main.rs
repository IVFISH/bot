mod board;
mod placement;
mod errors;
mod game;
mod queue;
mod players;
mod bot;
mod human;
mod communications;

use crate::bot::*;
use crate::human::*;
use crate::players::*;



fn main() {
    // human_play();

    // bot_play();
    bot_debug();

}
