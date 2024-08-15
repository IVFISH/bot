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

fn bench(game: Game) {
    // for g in movegen(game) {
    //     println!(
    //         "{}",
    //         Bitmatrix {
    //             data: g.board.into_iter().collect()
    //         }
    //     );
    // }

    let rep = 100_000;
    let now = time::Instant::now();
    for _ in 0..rep {
        let _ = movegen(game);
    }
    println!(
        "movegen took {} nanoseconds",
        now.elapsed().as_nanos() / rep
    );
}

fn main() {
    let mut game = Game::new(0x1);

    bench(game);

    game.board[0] = 0xaaaa;
    game.board[1] = 0x0000;
    game.board[2] = 0x2222;

    bench(game);
}
