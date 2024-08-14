use bitvec::view::BitViewSized;

use crate::bitmatrix::*;
use crate::game::*;

/// generate collision map and the trivials
/// TODO: implement other rotations
pub fn collision(game: Game) -> (Vec<Bitmatrix>, Vec<Bitmatrix>) {
    let (mut c, mut t) = (Vec::new(), Vec::new());
    for rot in 0..4 {
        let m = MASKS[Game::next(game.queue).0 - 1][rot];
        let mut p = [0u16; W];

        for col in 1..(W - 1) {
            let p0 = (game.board[col - 1] & m[0])
                | (game.board[col - 0] & m[1])
                | (game.board[col + 1] & m[2]);
            let p1 = (game.board[col - 1] & (m[0] << 1))
                | (game.board[col - 0] & (m[1] << 1))
                | (game.board[col + 1] & (m[2] << 1));
            let p2 = (game.board[col - 1] & (m[0] << 2))
                | (game.board[col - 0] & (m[1] << 2))
                | (game.board[col + 1] & (m[2] << 2));

            let np = ((p0 | (p0 >> 1) | (p0 >> 2)) & 0x1249)
                | ((p1 | (p1 >> 1) | (p1 >> 2)) & 0x2492)
                | ((p2 | (p2 >> 1) | (p2 >> 2)) & 0x4924);
            p[col] = !np;
        }

        let collisions = Bitmatrix {
            data: p.into_bitarray().to_bitvec(),
        };

        for col in 0..W {
            let q = p[col];

            let l = q.leading_zeros();
            p[col] = q & (u16::MAX.rotate_left(l));
        }

        let trivials = Bitmatrix {
            data: p.into_bitarray().to_bitvec(),
        };

        c.push(collisions);
        t.push(trivials);
    }

    (c, t)
}

pub fn reachable(collision: Vec<Bitmatrix>, trivials: Vec<Bitmatrix>) -> Vec<Bitmatrix> {
    assert_eq!(collision.len(), 4);
    assert_eq!(trivials.len(), 4);

    // find the initial possible matrix
    // then iterate the actions
    const N: usize = 20;
    let mut reachable = trivials;

    for _ in 0..N {
        let mut changed = false;
        for rot in 0..4 {
            let r = &reachable[rot];
            let p = &collision[rot];

            // get the reachable without rotation
            let mut q = r.or(p.and((r.lshift()).or(r.rshift()).or(r.ushift()).or(r.dshift())));

            // get the reachable with rotation
            for other in 0..4 {
                if other == rot {
                    continue;
                }

                let r = &reachable[other];

                // use the offset table here --> assuming kicks in all 4 directions for now
                q = q.or(p.and((r.lshift()).or(r.rshift()).or(r.ushift()).or(r.dshift())));
            }

            changed |= *r != q;
            reachable[rot] = q;
        }

        if !changed {
            break;
        }
    }

    reachable
        .into_iter()
        .map(|r| r.and(r.ushift().not()))
        .collect()
}

pub fn to_game_vec(game: Game, reachable: Vec<Bitmatrix>) -> Vec<Game> {
    assert_eq!(reachable.len(), 4);

    let mut ret = Vec::with_capacity(reachable.iter().map(|d| d.data.count_ones()).sum());
    let (p, q) = Game::next(game.queue);

    for rot in 0..4 {
        let piece = PIECES[p - 1][rot];

        let copy = Game {
            board: game.board,
            queue: q,
        };

        for i in reachable[rot].data.iter_ones() {
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
    let (c, t) = collision(game);
    to_game_vec(game, reachable(c, t))
}
