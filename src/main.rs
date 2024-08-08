#![feature(portable_simd)]
#![feature(array_repeat)]
#![feature(slice_as_chunks)]

mod game;
mod scalar;
mod simd;

use game::*;
use std::time;
use simd::*;
use scalar::*;

fn main() {
    // note: the queue is backwards (also 1-indexed)
    let game = Game::new(0x21_123);

    let now = time::Instant::now();
    let res1 = search_scalar(game);
    println!(
        "found {} boards in {} microseconds",
        res1.len(),
        now.elapsed().as_micros()
    );

    // let now2 = time::Instant::now();
    // let res2 = search_simd(game);
    // println!(
    //     "found {} boards in {} microseconds",
    //     res2.len(),
    //     now2.elapsed().as_micros()
    // );

    println!("{:?}", res1.last().unwrap().board);
}
