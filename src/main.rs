pub mod bitmatrix;
pub mod game;
pub mod nontrivials;
mod test_api;

use game::*;
use nontrivials::*;
use std::time;

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
    // sd_test();
    // let mut game = Game::new(0x1);

    // // bench(game);

    // game.board[0] = 0xaaaa;
    // game.board[1] = 0x0000;
    // game.board[2] = 0x2222;

    // println!("{}", game);

    // bench(game);
}

fn sd_test() {
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

    let rea = 0b01001001;
    println!("reachable = 0b01001001");
    display(rea);

    let diffs = col ^ (col.wrapping_add(rea));
    println!("diffs = col ^ (col + reachable)");
    display(diffs);

    let (c, r) = (col >> 2, rea >> 2);
    let sd = (c ^ (c.wrapping_add(r))).wrapping_add(r);
    println!("sd: c = col >> 2; r = reachable >> 2");
    println!("[ (c ^ (c + r)) + r ] >> 2");
    display(sd);

    let (c, r) = (col >> 1, rea >> 1);
    let sd = c ^ ((c + r) | c);
    println!("sd: [ c ^ ((c + r) | c) ] >> 1");
    display(sd);
}

#[cfg(test)]
pub mod test {
    use super::bench;
    use crate::test_api::test_api::*;

    #[test]
    fn bench_test() {
        bench(versus_board_medium());
    }
}
