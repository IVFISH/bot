pub mod bitmatrix;
pub mod game;
pub mod nontrivials;
mod test_api;

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
    let game = l_spin_board_4();
    #[rustfmt::skip]
    let sol_str = [
        "ooooo.x.oo",
        "ooooxxxooo",
    ];

    // for g in movegen(game) {
    //     println!("{}", g);
    // }

    let gen = movegen(game);
    assert_not_contains(&gen, game_from_string(&sol_str, 0));
    for g in gen {
        println!("{}", g);
    }

    bench(game);
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
        // bench(versus_board_medium());
    }
}
