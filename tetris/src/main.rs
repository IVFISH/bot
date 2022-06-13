mod board;
mod placement;
mod errors;
mod game;
mod queue;
mod players;
mod bot;

use crate::bot::*;
use crate::players::*;


fn main() {

    let mut bot = Bot::default();

    println!("{}", bot.game);
    for _ in 0..15 {
        bot.make_move();
        println!("{}", bot.game);
    }
    println!("{}", bot.game.active_piece);

}
