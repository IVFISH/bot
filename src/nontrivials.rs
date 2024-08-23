use crate::bitmatrix::*;
use crate::game::*;

/// generate collision map and the trivials
pub fn collision(board: Bitmatrix, piece: usize) -> ([Bitmatrix; 4], [Bitmatrix; 4]) {
    let (mut c, mut t) = ([Bitmatrix::new(); 4], [Bitmatrix::new(); 4]);
    for rot in 0..4 {
        let m = MASKS[piece][rot];
        let mut p = [0u16; W];

        for col in 1..(W - 1) {
            let p0 = (board[col - 1] & m[0]) | (board[col - 0] & m[1]) | (board[col + 1] & m[2]);
            let p1 = (board[col - 1] & (m[0] >> 1))
                | (board[col - 0] & (m[1] >> 1))
                | (board[col + 1] & (m[2] >> 1));
            let p2 = (board[col - 1] & (m[0] >> 2))
                | (board[col - 0] & (m[1] >> 2))
                | (board[col + 1] & (m[2] >> 2));

            let np = ((p0 | (p0 << 1) | (p0 << 2)) & 0o111110)  // 1_001_001..000
                | ((p1 | (p1 << 1) | (p1 << 2)) & 0o44444)      // 0_100_100..
                | ((p2 | (p2 << 1) | (p2 << 2)) & 0o22222); // 0_010_010..
            p[col] = !np;
        }

        // try col = 0
        if m[0] == 0 {
            let col = 0;
            let p0 = (board[col - 0] & m[1]) | (board[col + 1] & m[2]);
            let p1 = (board[col - 0] & (m[1] >> 1)) | (board[col + 1] & (m[2] >> 1));
            let p2 = (board[col - 0] & (m[1] >> 2)) | (board[col + 1] & (m[2] >> 2));

            let np = ((p0 | (p0 << 1) | (p0 << 2)) & 0o111110)  // 1_001_001..000
                | ((p1 | (p1 << 1) | (p1 << 2)) & 0o44444)      // 0_100_100..
                | ((p2 | (p2 << 1) | (p2 << 2)) & 0o22222); // 0_010_010..
            p[col] = !np;
        }

        // try col = 9
        if m[2] == 0 {
            let col = 9;
            let p0 = (board[col - 1] & m[0]) | (board[col - 0] & m[1]);
            let p1 = (board[col - 1] & (m[0] >> 1)) | (board[col - 0] & (m[1] >> 1));
            let p2 = (board[col - 1] & (m[0] >> 2)) | (board[col - 0] & (m[1] >> 2));

            let np = ((p0 | (p0 << 1) | (p0 << 2)) & 0o111110)  // 1_001_001..000
                | ((p1 | (p1 << 1) | (p1 << 2)) & 0o44444)      // 0_100_100..
                | ((p2 | (p2 << 1) | (p2 << 2)) & 0o22222); // 0_010_010..
            p[col] = !np;
        }

        let collisions = Bitmatrix { data: p };

        for col in 0..W {
            if p[col] == 0 {
                continue;
            }

            let l = p[col].trailing_ones();
            p[col] = 1 << (l - 1);
        }

        let trivials = Bitmatrix { data: p };

        c[rot] = collisions;
        t[rot] = trivials;
    }

    (c, t)
}

pub fn reachable(
    collision: [Bitmatrix; 4],
    trivials: [Bitmatrix; 4],
    piece: usize,
) -> [Bitmatrix; 4] {
    if collision.map(|cmap| cmap.grounded()) == trivials {
        // no grounded nontrivials
        return trivials;
    }

    // NONTRIVIALS:
    // from the trivials, iterate each action
    // and-ing with possibles every iter
    // until nothing new is found
    const N: usize = 20;
    let mut reachable = trivials;

    for _ in 0..N {
        let mut changed = false;
        for rot in 0..4 {
            let r = reachable[rot];
            let p = collision[rot];

            // get the reachable without rotation
            // next iteration of r
            let mut q = r | (p & (r.lshift(1) | r.rshift(1) | r.softdrop(p)));

            // get the reachable with rotation
            // new_rot is (rot - dir) % 4
            // addition by (4 - dir) is done instead
            let new_rot = (rot + 3) % 4;
            q |= rotate_kick(reachable[new_rot], p, &KICKS[piece][new_rot][0]);
            let new_rot = (rot + 2) % 4;
            q |= rotate_kick(reachable[new_rot], p, &KICKS_180[piece][new_rot]);
            let new_rot = (rot + 1) % 4;
            q |= rotate_kick(reachable[new_rot], p, &KICKS[piece][new_rot][1]);

            changed |= r != q;
            reachable[rot] = q;
        }

        if !changed {
            break;
        }
    }

    reachable.map(|r| r.grounded())
}

#[inline(always)]
pub fn rotate_kick(t: Bitmatrix, p: Bitmatrix, offsets: &[[i32; 2]]) -> Bitmatrix {
    // t: reachables in (rot - dir) to try kicking from, updated each iter
    // p: possibles in (rot)

    offsets
        .iter()
        .fold((Bitmatrix::new(), t), |(o, t), &[dc, dr]| {
            let o1 = t.shift(dr, dc) & p; // new places found this iteration
            (o | o1, t ^ o1.shift(-dr, -dc)) // remove places that successfully kicked
        })
        .0
}

pub fn to_game_vec(mut game: Game, reachable: [Bitmatrix; 4]) -> Vec<Game> {
    let mut ret = Vec::with_capacity(reachable.iter().map(|d| d.count_ones()).sum());
    let p = game.next();

    for rot in 0..4 {
        let piece = PIECES[p - 1][rot];

        let copy = game.clone();

        for i in reachable[rot].iter_ones() {
            let (r, c) = (i % H, i / H);

            if r < 2 {
                continue;
            }

            let mut cpy = copy;
            if piece[0] != 0 {
                cpy.board[c - 1] |= piece[0] << r - 2;
            }
            cpy.board[c - 0] |= piece[1] << r - 2;
            if piece[2] != 0 {
                cpy.board[c + 1] |= piece[2] << r - 2;
            }
            ret.push(cpy);
        }
    }

    ret
}

pub fn movegen(game: Game) -> Vec<Game> {
    let piece = game.peek() - 1;
    let (c, t) = collision(game.board, piece);
    to_game_vec(game, reachable(c, t, piece))
}

#[cfg(test)]
pub mod tests {
    use crate::nontrivials::movegen;
    use crate::test_api::test_api::*;

    #[test]
    fn l_spin_1() {
        let game = l_spin_board_1();
        #[rustfmt::skip]
        let sol_str_1 = [
            "oooo..oooo",
            "oooxxxoooo",
            "oooxoooooo",
        ];
        #[rustfmt::skip]
        let sol_str_2 = [
            "oooo.xoooo",
            "oooxxxoooo",
            "ooo.oooooo",
        ];

        for g in movegen(game) {
            println!("{}", g);
        }

        let gen = movegen(game);
        assert_eq!(gen.len(), 36);
        assert_contains(&gen, game_from_string(&sol_str_1, 0));
        assert_contains(&gen, game_from_string(&sol_str_2, 0));
    }

    #[test]
    fn l_spin_2() {
        let game = l_spin_board_2();
        #[rustfmt::skip]
        let sol_str = [
            "oo........",
            "o.........",
            "oo.ooooooo",
            "oo.o..oooo",
            "oo....oooo",
            "ooooo.oooo",
            "oooo....oo",
            "oooo..oooo",
            "ooooo.oooo",
            "ooo...oooo",
            "oo..oooooo",
            "o...oooooo",
            "oxo..ooooo",
            "oxoooooooo",
            "oxxooooooo",
        ];

        for g in movegen(game) {
            println!("{}", g);
        }

        let gen = movegen(game);
        assert_eq!(gen.len(), 43);
        // NOTE: there are 7 pieces on the top left that are cut-off by the u16 limit
        // should this limit be increased -- increase assert to 50
        assert_contains(&gen, game_from_string(&sol_str, 0));
    }

    #[test]
    fn l_spin_3() {
        let game = l_spin_board_3();
        #[rustfmt::skip]
        let sol_str = [
            "oooo.ooooo",
            "oooxxxoooo",
            "oooxoooooo",
        ];

        // for g in movegen(game) {
        //     println!("{}", g);
        // }

        let gen = movegen(game);
        assert_not_contains(&gen, game_from_string(&sol_str, 0));
    }

    #[test]
    fn l_spin_4() {
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
    }

    #[test]
    fn sd() {
        let board_str = [
            "ooo..........",
            "o............",
            "o............",
            "ooo..........",
        ];

        let sol_str = [
            "ooo..........",
            "oxx..........",
            "oxx..........",
            "ooo..........",
        ];

        let game = game_from_string(&board_str, 0o3);
        let sol = game_from_string(&sol_str, 0o3);

        for g in movegen(game) {
            println!("{}", g);
        }

        let gen = movegen(game);
        assert_not_contains(&gen, sol);
    }
}
