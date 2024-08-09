use crate::game::*;
use crate::scalar::process_scalar;

use std::collections::*;
use std::ops::*;
use std::simd::prelude::*;

pub fn search_simd_med(x: &[u16], y: &[u16], z: &[u16]) -> (Vec<u16>, Vec<u16>, Vec<u16>) {
    let p_bitmask = PIECES[0][0];

    let sixteen = u16x8::splat(16);
    let ii = i16x8::splat(p_bitmask[0].trailing_zeros() as i16);
    let jj = i16x8::splat(p_bitmask[1].trailing_zeros() as i16);
    let kk = i16x8::splat(p_bitmask[2].trailing_zeros() as i16);

    // chunking x -> Simd types
    let x = x.array_chunks::<8>().map(|&x| u16x8::from_array(x));
    let y = y.array_chunks::<8>().map(|&y| u16x8::from_array(y));
    let z = z.array_chunks::<8>().map(|&z| u16x8::from_array(z));

    // process game (on single column)
    let p0 = u16x8::splat(p_bitmask[0]);
    let p1 = u16x8::splat(p_bitmask[1]);
    let p2 = u16x8::splat(p_bitmask[2]);

    let g = x.zip(y).zip(z).map(|((x, y), z)| {
        let nx = ((sixteen - x.leading_zeros()).cast::<i16>() - ii).cast::<u16>();
        let ny = ((sixteen - y.leading_zeros()).cast::<i16>() - jj).cast::<u16>();
        let nz = ((sixteen - z.leading_zeros()).cast::<i16>() - kk).cast::<u16>();
        let n = nx.simd_max(ny).simd_max(nz);

        (x.bitor(p0 << n).to_array(), (y.bitor(p1 << n).to_array(), z.bitor(p2 << n).to_array()))
    });

    let (x, (y, z)): (Vec<[u16; 8]>, (Vec<[u16; 8]>, Vec<[u16; 8]>)) = g.unzip();
    (x.into_flattened(), y.into_flattened(), z.into_flattened())
}

pub fn search_simd_smol(x: &[u16]) -> Vec<u16> {
    let p_bitmask = PIECES[0][0];

    let sixteen = u16x8::splat(16);
    let ii = i16x8::splat(p_bitmask[0].trailing_zeros() as i16);
    let p0 = u16x8::splat(p_bitmask[0]);

    // chunking x -> Simd types
    let x = x.array_chunks::<8>().map(|&x| u16x8::from_array(x));

    // process game (on single column)
    let i = x.map(|x| {
        let n = ((sixteen - x.leading_zeros()).cast::<i16>() - ii).cast::<u16>();
        x | (p0 << n)
    });

    return i.flat_map(|i| i.to_array()).collect();
}

/// Does a BFS to find all future game states from a given Game.
/// Searches until the queue is empty.
pub fn search_simd(game: Game) -> Vec<Game> {
    let mut res = Vec::new();
    let mut bfs = VecDeque::new();

    let (initial_p, initial_new_queue) = Game::next(game.queue);
    let initial = process_scalar(&[game], initial_p, initial_new_queue);

    bfs.push_front(initial.clone());
    // res.extend(initial);

    while !bfs.is_empty() {
        // take all from que with the same Game.queue
        // iterate the placements with SIMD
        let me = bfs.pop_front().unwrap();
        let (piece, new_queue) = Game::next(me[0].queue);

        // we can actually break here i think
        if piece == 0 {
            continue;
        };

        let mut next = Vec::new();

        for rot in 0..4 {
            // generate the bitmask for piece and rot
            let p_bitmask = PIECES[piece - 1][rot];

            let ii = i16x8::splat(p_bitmask[0].trailing_zeros() as i16);
            let jj = i16x8::splat(p_bitmask[1].trailing_zeros() as i16);
            let kk = i16x8::splat(p_bitmask[2].trailing_zeros() as i16);

            // TODO: include col 0, 9 :)
            for col in 1..9 {
                let x = me
                    .iter()
                    .map(|g| g.board[col - 1])
                    .array_chunks::<8>()
                    .map(|x| u16x8::from_array(x));
                let y = me
                    .iter()
                    .map(|g| g.board[col + 0])
                    .array_chunks::<8>()
                    .map(|y| u16x8::from_array(y));
                let z = me
                    .iter()
                    .map(|g| g.board[col + 1])
                    .array_chunks::<8>()
                    .map(|z| u16x8::from_array(z));

                let sixteen = u16x8::splat(16);

                let i = x.clone().map(|x| sixteen - x.leading_zeros());
                let j = y.clone().map(|y| sixteen - y.leading_zeros());
                let k = z.clone().map(|z| sixteen - z.leading_zeros());

                let n = i.zip(j).zip(k).map(|((i, j), k)| {
                    (i.cast::<i16>() - ii)
                        .simd_max(j.cast::<i16>() - jj)
                        .simd_max(k.cast::<i16>() - kk)
                        .cast::<u16>()
                });

                let p0 = u16x8::splat(p_bitmask[0]);
                let p1 = u16x8::splat(p_bitmask[1]);
                let p2 = u16x8::splat(p_bitmask[2]);

                let g = x.zip(y).zip(z).zip(n).map(|(((x, y), z), n)| {
                    (
                        p0.shl(n).bitor(x).to_array(),
                        p1.shl(n).bitor(y).to_array(),
                        p2.shl(n).bitor(z).to_array(),
                    )
                });

                let gg = me.array_chunks::<8>().zip(g).flat_map(|(m, (p0, p1, p2))| {
                    m.iter()
                        .zip(p0)
                        .zip(p1)
                        .zip(p2)
                        .map(|(((&(mut m), p0), p1), p2)| {
                            m.board[col - 1] |= p0;
                            m.board[col - 0] |= p1;
                            m.board[col + 1] |= p2;
                            Game {
                                board: m.board,
                                queue: new_queue,
                            }
                        })
                });

                let gg: Vec<Game> = gg.collect();
                next.extend(gg);
            }
        }
        // res.extend(next.clone());
        bfs.push_back(next);
    }

    res
}
