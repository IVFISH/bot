#![feature(maybe_uninit_uninit_array)]
#![feature(wrapping_next_power_of_two)]

pub mod bitmatrix;
pub mod game;
pub mod nontrivials;
pub mod test_api;

use bitmatrix::Bitmatrix;
use game::*;
use nontrivials::*;
use std::time;
use test_api::test_api::*;

fn main() {
   let mut game = Game::new(1);
   let reps = 5;
   for _ in 0..reps {
       println!("{}", game.peek());
       game = movegen(game)[11];
       println!("{}", game);
   }
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
