#![feature(portable_simd)]
#![feature(iter_array_chunks)]
#![feature(array_chunks)]
#![feature(slice_as_chunks)]

pub mod bitmatrix;
pub mod game;
pub mod nontrivials;
pub mod scalar;

use bitmatrix::*;
use bitvec::prelude::*;
use game::*;
use nontrivials::*;
use scalar::*;
use std::time;

// fn main() {
//     // note: the queue is backwards (also 1-indexed)
//     let game = Game::new(0x1_321);
//
//     let now1 = time::Instant::now();
//     let res1 = search_scalar(game);
//     println!(
//         "scalar found {} boards in {} microseconds",
//         res1.len(),
//         now1.elapsed().as_micros()
//     );
//
//     let now2 = time::Instant::now();
//     let res2 = search_simd(game);
//     println!(
//         "simd found {} boards in {} microseconds",
//         res2[0].len(),
//         now2.elapsed().as_micros()
//     );
// }

fn main() {
    let mut game = Game::new(0x1);
    game.board[0] = 0xaaaa;
    game.board[1] = 0x0000;
    game.board[2] = 0x2222;

    // game.board[7] = 0x8000;
    // game.board[8] = 0x36db << 1;
    // game.board[9] = 0x1249 << 1;

    println!("{}", Bitmatrix { data: game.board.into_bitarray().to_bitvec()});

    // println!("===================");

    let (collisions, trivials) = collision(game);
    // println!("{}", collisions);

    // println!("===================");
    
    let reached = reachable(collisions, trivials);
    println!("{}", reached[0]);

    for g in movegen(game) {
        println!(
            "{}",
            Bitmatrix {
                data: g.board.into_bitarray().to_bitvec()
            }
        );
    }

    let rep = 10_000;
    let now = time::Instant::now();
    for _ in 0..rep {
        let _ = movegen(game);
    }
    println!("movegen took {} microsconds", now.elapsed().as_micros() / rep);

    let now = time::Instant::now();
    for _ in 0..rep {
        let _ = collision(game);
    }
    println!("collision map took {} microsconds", now.elapsed().as_micros() / rep);

}
