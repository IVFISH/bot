#![feature(portable_simd)]
#![feature(iter_array_chunks)]
#![feature(array_chunks)]
#![feature(slice_as_chunks)]

pub mod bitmatrix;
pub mod game;
pub mod nontrivials;
pub mod scalar;
mod test_api;

use game::*;
use nontrivials::*;
use std::time;

#[allow(dead_code)]
fn bench(game: Game) {
    for g in movegen(game) {
        println!("{}", g);
    }

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

    // bench(game);

    game.board[0] = 0xaaaa;
    game.board[1] = 0x0000;
    game.board[2] = 0x2222;

    println!("{}", game);

    // bench(game);
}

#[cfg(test)]
pub mod test {
    use crate::test_api::test_api::l_spin_board_2;
    use super::bench;

    #[test]
    fn bench_l_spin_fuckery() {
        bench(l_spin_board_2());
    }
}
