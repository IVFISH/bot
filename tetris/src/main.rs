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
use crate::human::*;
use crate::players::*;


fn main() {

    // human_play();

    bot_play();
    // bot_debug();
}
