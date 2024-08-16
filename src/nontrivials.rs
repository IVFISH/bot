use crate::bitmatrix::*;
use crate::game::*;

/// generate collision map and the trivials
pub fn collision(board: Bitmatrix, piece: usize) -> (Vec<Bitmatrix>, Vec<Bitmatrix>) {
    let (mut c, mut t) = (Vec::new(), Vec::new());
    for rot in 0..4 {
        let m = MASKS[piece][rot];
        let mut p = [0u16; W];

        for col in 1..(W - 1) {
            let p0 = (board[col - 1] & m[0])
                | (board[col - 0] & m[1])
                | (board[col + 1] & m[2]);
            let p1 = (board[col - 1] & (m[0] << 1))
                | (board[col - 0] & (m[1] << 1))
                | (board[col + 1] & (m[2] << 1));
            let p2 = (board[col - 1] & (m[0] << 2))
                | (board[col - 0] & (m[1] << 2))
                | (board[col + 1] & (m[2] << 2));

            let np = ((p0 | (p0 >> 1) | (p0 >> 2)) & 0x1249)
                | ((p1 | (p1 >> 1) | (p1 >> 2)) & 0x2492)
                | ((p2 | (p2 >> 1) | (p2 >> 2)) & 0x4924);
            p[col] = !np;
        }

        let collisions = Bitmatrix {
            data: p.into_iter().collect(),
        };

        for col in 0..W {
            if p[col] == 0 { continue; }
            let l = p[col].leading_ones();
            p[col] &= u16::MAX >> (l - 1);
        }

        let trivials = Bitmatrix {
            data: p.into_iter().collect(),
        };

        c.push(collisions);
        t.push(trivials);
    }

    (c, t)
}

pub fn reachable(collision: Vec<Bitmatrix>, trivials: Vec<Bitmatrix>, piece: usize) -> Vec<Bitmatrix> {
    assert_eq!(collision.len(), 4);
    assert_eq!(trivials.len(), 4);

    // find the initial possible matrix
    // then iterate the actions
    const N: usize = 20;
    let mut reachable = trivials;

    for _ in 0..N {
        let mut changed = false;
        for rot in 0..4 {
            let r = reachable[rot];
            let p = collision[rot];

            // get the reachable without rotation
            let mut q = r | (p & (r.lshift(1) | r.rshift(1) | r.dshift(1)));

            // get the reachable with rotation
            // CW, CCW
            for dir in [1, 3] {
                let r = &reachable[(rot + dir) % 4];

                // use the offset table here --> assuming kicks in all 4 directions for now
                let offsets = KICKS[piece][rot][(dir == 3) as usize];

                // to make sure kicks don't apply multiple times
                // keep track of all the differences (xor)
                // let mut d = Bitmatrix::new();

                // translate positive dR into ushift
                // translate positive dC into rshift
                let mut o = *r;
                for [dr, dc] in offsets {
                    // let s = o.shift(dr, dc);
                    // d |= d ^ s;
                    // o |= s & !d;
                    o |= o.shift(dr, dc);
                }

                q |= p & o;
            }

            changed |= r != q;
            reachable[rot] = q;
        }

        if !changed {
            break;
        }
    }

    reachable
        .into_iter()
        .map(|r| r & !r.ushift(1))
        .collect()
}

pub fn to_game_vec(game: Game, reachable: Vec<Bitmatrix>) -> Vec<Game> {
    assert_eq!(reachable.len(), 4);

    let mut ret = Vec::with_capacity(reachable.iter().map(|d| d.count_ones()).sum());
    let (p, q) = Game::next(game.queue);

    for rot in 0..4 {
        let piece = PIECES[p - 1][rot];

        let copy = Game {
            board: game.board,
            queue: q,
        };

        for i in reachable[rot].iter_ones() {
            let (r, c) = (i % H, i / H);

            let mut cpy = copy;
            cpy.board[c - 1] |= piece[0] << r;
            cpy.board[c - 0] |= piece[1] << r;
            cpy.board[c + 1] |= piece[2] << r;
            ret.push(cpy);
        }
    }

    ret
}

pub fn movegen(game: Game) -> Vec<Game> {

    let piece = Game::next(game.queue).0 - 1;
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

        // for g in movegen(game) {
        //     println!("{}", g);
        // }

        let gen = movegen(game);
        assert_eq!(gen.len(), 34);
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

        // for g in movegen(game) {
        //     println!("{}", g);
        // }

        // NOTE: on 18+ high board, this should generate 50 pieces (there is trivial on the left)
        let gen = movegen(game);
        assert_eq!(gen.len(), 49);
        assert_contains(&gen, game_from_string(&sol_str, 0));
    }

    #[test]
    fn l_spin_3() {
        let game = l_spin_board_3();
        #[rustfmt::skip]
        let sol_str = [
            "oooo.ooooo",
            "oooxxxoooo",
            "oox.oooooo",
        ];

        // for g in movegen(game) {
        //     println!("{}", g);
        // }

        let gen = movegen(game);
        assert_not_contains(&gen, game_from_string(&sol_str, 0));
    }
}
