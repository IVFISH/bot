mod board;
mod bot;
mod command;
mod constants;
mod controller;
mod game;
mod piece;
mod placement;
mod test_api;

use crate::constants::piece_constants::*;
use crate::board::*;
use crate::bot::*;
use crate::piece::*;

pub fn add_list(board: &mut Board, list: Vec<[usize; 2]>) {
    for [r, c] in list.into_iter() {
        board.set(r, c, 1);
    }
}

fn main() {
    let mut bot = Bot::new();
    let b = &mut bot.game.board;
    add_list(b, vec![[2, 7], [2, 8], [2, 9], [2, 0], [2, 1], [2, 2]]);
    bot.game.active = Piece::new(PIECE_O);
    bot.move_gen();
}
