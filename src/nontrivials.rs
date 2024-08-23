use crate::bitmatrix::*;
use crate::constants::*;
use crate::game::*;

use std::cmp::min;

/// generate collision map and the trivials
pub fn collision(board: Bitmatrix, piece: usize) -> [Bitmatrix; 4] {
    if piece == PIECE_I - 1 {
        return i_collision(board);
    }

    let mut cmaps = [Bitmatrix::new(); 4];
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

        cmaps[rot] = Bitmatrix { data: p };
    }

    cmaps
}

pub fn i_collision(board: Bitmatrix) -> [Bitmatrix; 4] {
    let nboard = !board;

    let vert = !(board | board.dshift(1) | board.dshift(2) | board.dshift(3));
    let hor = nboard.rshift(2) & nboard.rshift(1) & nboard & nboard.lshift(1);

    [hor, vert, hor, vert.rshift(1)]
}

pub fn trivial(cmap: Bitmatrix) -> Bitmatrix {
    let data = cmap.data.map(|c| {
        if c == 0 {
            0
        } else {
            1 << (c.trailing_ones() - 1)
        }
    });

    Bitmatrix { data }
}

pub fn reachable(collisions: [Bitmatrix; 4], piece: usize) -> [Bitmatrix; 4] {
    let trivials = collisions.map(|c| trivial(c));
    let grounded = collisions.map(|c| c.grounded());

    // NONTRIVIALS:
    // from the trivials, iterate each action
    // and-ing with possibles every iter
    // until nothing new is found
    const N: usize = 20;
    let mut reachables = trivials;

    for _ in 0..N {
        if reachables == grounded {
            return reachables;
        }

        let mut changed = false;
        for rot in 0..4 {
            let r = reachables[rot];
            let p = collisions[rot];

            // get the reachable without rotation
            // next iteration of r
            let mut q = r | (p & (r.lshift(1) | r.rshift(1) | r.softdrop(p)));

            // get the reachable with rotation
            // new_rot is (rot - dir) % 4
            // addition by (4 - dir) is done instead
            let new_rot = (rot + 3) % 4;
            q |= rotate_kick(reachables[new_rot], p, KICKS[piece][new_rot][0]);
            let new_rot = (rot + 2) % 4;
            q |= rotate_kick(reachables[new_rot], p, KICKS[piece][new_rot][1]);
            let new_rot = (rot + 1) % 4;
            q |= rotate_kick(reachables[new_rot], p, KICKS[piece][new_rot][2]);

            changed |= r != q;
            reachables[rot] = q;
        }

        if !changed {
            break;
        }
    }

    reachables.map(|r| r.grounded())
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
    let p = game.next() - 1;

    let is_i_piece = p == PIECE_I - 1;

    for rot in 0..4 {
        let piece = PIECES[p][rot];
        let piece_height = H
            - (piece
                .iter()
                .map(|p| p.leading_zeros() + p.trailing_zeros())
                .min()
                .unwrap() as usize);

        for i in reachable[rot].iter_ones() {
            let (r, c) = (i % H, i / H);

            if r < piece_height - 1 {
                continue;
            }

            let mut cpy = game.clone();

            let mut len = 3; // # cols of piece
            let mut c_idx = 1; // col of the piece's canonical center
            if is_i_piece {
                len = 4;
                c_idx = 2;
            }

            let d = len - 1; // row of the piece's canonical center from the TOP

            // apply in-bounds cols of the piece to the board
            //for i in c_idx.saturating_sub(c)..min(len, W + c_idx - c){
            //    cpy.board[c + i - c_idx] |= piece[i] << (r - d);
            //}

            // apply nonzero cols of the piece to the board
            for (i, x) in piece.iter().enumerate().filter(|(i, &x)| x != 0) {
                cpy.board[c + i - c_idx] |= x << (r - d);
            }

            ret.push(cpy);
        }
    }

    ret
}

pub fn movegen(game: Game) -> Vec<Game> {
    let piece = game.peek() - 1;
    let c = collision(game.board, piece);
    to_game_vec(game, reachable(c, piece))
}

#[cfg(test)]
pub mod tests {
    use crate::nontrivials::movegen;
    use crate::test_api::test_api::*;

    #[test]
    fn i_spin_1() {
        let game = i_spin_game_1();
        #[rustfmt::skip]
        let sol_str = [
            "........o.",
            "..........",
            "oooooo.oo.",
            "oooxxxxooo",
        ];

        // for g in movegen(game) {
        //     println!("{}", g);
        // }

        let gen = movegen(game);
        assert_contains(&gen, game_from_string(&sol_str, 0));
    }

    #[test]
    fn i_spin_2() {
        let game = i_spin_game_2();
        #[rustfmt::skip]
        let sol_str = [
            ".o........",
            "..........",
            ".oo.ooooo.",
            "oooxxxxooo",
        ];

        // for g in movegen(game) {
        //     println!("{}", g);
        // }

        let gen = movegen(game);
        assert_contains(&gen, game_from_string(&sol_str, 0));
    }

    #[test]
    fn i_spin_3() {
        let game = i_spin_game_3();
        #[rustfmt::skip]
        let sol_str = [
            "oooo.ooooo",
            "oooxxxxooo",
        ];

        for g in movegen(game) {
            println!("{}", g);
        }

        let gen = movegen(game);
        assert_not_contains(&gen, game_from_string(&sol_str, 0));
    }

    #[test]
    fn i_spin_4() {
        let game = i_spin_game_4();
        #[rustfmt::skip]
        let sol_str = [
            "oooooo.oo.",
            "oooxxxxooo",
        ];

        // for g in movegen(game) {
        //     println!("{}", g);
        // }

        let gen = movegen(game);
        assert_not_contains(&gen, game_from_string(&sol_str, 0));
    }

    #[test]
    fn l_spin_1() {
        let game = l_spin_game_1();
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
        let game = l_spin_game_2();
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
        let game = l_spin_game_3();
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
        let game = l_spin_game_4();
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
