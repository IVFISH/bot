#![feature(maybe_uninit_uninit_array)]
#![feature(wrapping_next_power_of_two)]

pub mod bitmatrix;
pub mod constants;
pub mod game;
pub mod nontrivials;
pub mod suggestion;
pub mod test_api;

use bitmatrix::Bitmatrix;
use constants::*;
use game::*;
use nontrivials::*;
use std::time;
use suggestion::*;
use test_api::test_api::*;

fn main() {
    // let mut game = Game::new(1);
    // game.queue = PIECE_I as u64;

    // let collisions = i_collision(game.board);
    // let trivials = collisions.map(|cmap| trivial(cmap));
    // let reachable = reachable(collisions, PIECE_I - 1);

    // println!("{}", collisions[0]);
    // println!("{}", trivials[0]);
    // println!("{}", reachable[0]);

    // for g in movegen(game) {
    //     println!("{}", g);
    // }

    let before = l_spin_game_1();
    let gen = movegen(before);

    println!("{}", before);
    // println!("{}", after);

    // let piece = before.peek() - 1;
    // let placed = after.board ^ before.board;
    // for (r, c, rot) in get_location(placed, piece) {
    //     println!("{} {} {}", r, c, rot);
    // }

    for g in gen {
        println!("{:?}", to_command_list(before, g, false));
        println!("{}", g);
    }

    // let reps = 5;
    // for _ in 0..reps {
    //     println!("{}", game.peek());
    //     game = movegen(game)[11];
    //     println!("{}", game);
    // }
}

fn main1() {
    fn display(x: u8) {
        for i in 0..8 {
            if (x & 1 << i) != 0 {
                println!("■ ");
            } else {
                println!("□ ");
            }
        }
    }

    let col = 0b11001101;
    println!("col = 0b11001101");
    display(col);

    let rea = 0b01001101;
    println!("reachable = 0b01001101");
    display(rea);

    let (c, r) = (col >> 2, rea >> 2);
    let sd = (c ^ (c + r)) + r;
    println!("sd: c = col >> 2; r = reachable >> 2");
    println!("[ (c ^ (c + r)) + r ] >> 2");
    display(sd);

    let (c, r) = (col >> 1, rea >> 1);
    let sd = c ^ ((c + r) | c);
    println!("sd: [ c ^ ((c + r) | c) ] >> 1");
    display(sd);

    let (c, r) = (col >> 1, rea >> 1);
    let sd = (c + r) & !c;
    println!("sd: [ c ^ ((c + r) | c) ] >> 1");
    display(sd);
}
