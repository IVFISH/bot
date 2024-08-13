#![feature(portable_simd)]
#![feature(iter_array_chunks)]
#![feature(array_chunks)]
#![feature(slice_as_chunks)]

pub mod game;
pub mod scalar;
pub mod simd;

use game::*;
use scalar::*;
use simd::*;
use std::time;

fn main() {
    // note: the queue is backwards (also 1-indexed)
    let game = Game::new(0x1_321);

    let now1 = time::Instant::now();
    let res1 = search_scalar(game);
    println!(
        "scalar found {} boards in {} microseconds",
        res1.len(),
        now1.elapsed().as_micros()
    );

    let now2 = time::Instant::now();
    let res2 = search_simd(game);
    println!(
        "simd found {} boards in {} microseconds",
        res2[0].len(),
        now2.elapsed().as_micros()
    );

}
