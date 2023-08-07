mod board;
mod bot;
mod command;
mod constants;
mod controller;
mod game;
mod piece;
mod piece_queue;
mod placement;
mod placement_list;
mod pruner;
mod test_api;

use crate::bot::*;
use crate::board::*;
use crate::piece::*;
use crate::game::*;
use crate::pruner::*;
use crate::test_api::functions::*;
use std::time::Instant;

#[allow(unused)]
fn bench() {
    let bot = Bot::<NoPruner>::new();
    let n = 500_000;

    let now = Instant::now();
    for _ in 0..n {
        bot.move_gen(1);
    }
    println!("Averaged {} microseconds", now.elapsed().as_micros() / n);
}

fn main() {
    println!("{:?}", std::env::current_exe());
    std::env::set_var("RUST_BACKTRACE", "1");
    let now = Instant::now();
    // let mut bot = Bot::<NoPruner>::with_seed(3);
    // bot.game.board = l_spin_board_5();
    let mut bot = Bot::<AllClearPruner>::with_seed(4);
    bot.game.board = pco_board();
    let placements = bot.move_gen(7);
    println!("Took {} seconds", now.elapsed().as_secs());
    println!("{}", placements.placements.len());

    // filter the placements
    let pc_placements = placements.placements.clone().into_iter()
        .filter(|p| Board::get_max_height(&p.game_after.board.arr) == 0)
        .collect::<Vec<_>>();

    // placements.write_fumens("fumens.txt");

    // println!("{:?}", placement.pieces);
    // println!("{:?}", placement.piece);
    // println!("{:?}", placement.get_command_sequence());
    // let placement = &placements.placements[100];
    // println!("{:?}", placement.pieces);
    // println!("{:?}", placement.piece);
    // println!("{:?}", placement.get_command_sequence());
}
