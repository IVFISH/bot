#![feature(maybe_uninit_uninit_array)]
#![feature(wrapping_next_power_of_two)]

pub mod bitmatrix;
pub mod game;
pub mod nontrivials;
mod test_api;

use bitmatrix::Bitmatrix;
use game::*;
use nontrivials::*;
use std::time;
use test_api::test_api::*;

#[allow(dead_code)]
fn bench(game: Game) {
    // for g in movegen(game) {
    //     println!("{}", g);
    // }

    let rep = 100_000;
    let piece = Game::next(game.queue).0 - 1;
    let (c, t) = collision(game.board, piece);

    let now = time::Instant::now();
    for _ in 0..rep {
        let _ = collision(game.board, piece);
    }
    println!(
        "collision took {} nanoseconds",
        now.elapsed().as_nanos() / rep
    );

    let now = time::Instant::now();
    for _ in 0..rep {
        let _ = rotate_kick(Bitmatrix::new(), Bitmatrix::new(), &KICKS_180[0][0]);
    }
    println!(
        "kick checking took {} nanoseconds",
        now.elapsed().as_nanos() / rep
    );

    let mut elapsed = 0;
    for _ in 0..rep {
        let now = time::Instant::now();
        let _ = reachable(c.clone(), t.clone(), piece);
        elapsed += now.elapsed().as_nanos();
    }
    println!("reachable took {} nanoseconds", elapsed / rep);

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
    bench(Game::new(1));
    bench(l_spin_board_2());
    // for piece in 2..3 {
    //     for rot in 0..4 {
    //         let mut game = Game::new(0);

    //         game.board[0] = PIECES[piece][rot][0];
    //         game.board[1] = PIECES[piece][rot][1];
    //         game.board[2] = PIECES[piece][rot][2];

    //         game.board[7] = MASKS[piece][rot][0];
    //         game.board[8] = MASKS[piece][rot][1];
    //         game.board[9] = MASKS[piece][rot][2];

    //         println!("{}", game);
    //     }
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
