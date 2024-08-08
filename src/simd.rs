use crate::game::*;
use crate::scalar::process_scalar;

use std::collections::*;
use std::ops::*;
use std::simd::prelude::*;

/// Does a BFS to find all future game states from a given Game.
/// Searches until the queue is empty.
pub fn search_simd(game: Game) -> Vec<Game> {
    let mut res = Vec::new();
    let mut bfs = VecDeque::new();
    bfs.push_front(vec![game]);

    while !bfs.is_empty() {
        // take all from que with the same Game.queue
        // iterate the placements with SIMD
        let me = bfs.pop_front().unwrap();
        let (piece, new_queue) = Game::next(me[0].queue);

        // we can actually break here i think
        if piece == 0 {
            continue;
        };

        let len64 = (me.len() / 64) * 64;
        let rem = &me[len64..];
        let mut next = process_scalar(rem, piece, new_queue);

        for rot in 0..4 {
            // generate the bitmask for piece and rot
            let p_bitmask = PIECES[piece - 1][rot];

            let ii = i16x64::splat(p_bitmask[0].trailing_zeros() as i16);
            let jj = i16x64::splat(p_bitmask[1].trailing_zeros() as i16);
            let kk = i16x64::splat(p_bitmask[2].trailing_zeros() as i16);

            // TODO: include col 0, 9 :)
            for col in 1..9 {
                let x = me
                    .iter()
                    .map(|g| g.board[col - 1])
                    .array_chunks::<64>()
                    .map(|x| u16x64::from_array(x));
                let y = me
                    .iter()
                    .map(|g| g.board[col + 0])
                    .array_chunks::<64>()
                    .map(|y| u16x64::from_array(y));
                let z = me
                    .iter()
                    .map(|g| g.board[col + 1])
                    .array_chunks::<64>()
                    .map(|z| u16x64::from_array(z));

                let sixteen = u16x64::splat(16);

                let i = x.clone().map(|x| sixteen - x.leading_zeros());
                let j = y.clone().map(|y| sixteen - y.leading_zeros());
                let k = z.clone().map(|z| sixteen - z.leading_zeros());

                let n = i.zip(j).zip(k).map(|((i, j), k)| {
                    (i.cast::<i16>() - ii)
                        .simd_max(j.cast::<i16>() - jj)
                        .simd_max(k.cast::<i16>() - kk)
                });

                let p0 = i16x64::splat(p_bitmask[0] as i16);
                let p1 = i16x64::splat(p_bitmask[1] as i16);
                let p2 = i16x64::splat(p_bitmask[2] as i16);

                let g = x.zip(y).zip(z).zip(n).map(|(((x, y), z), n)| {
                    (
                        p0.shl(n).cast::<u16>().bitor(x).to_array(),
                        p1.shl(n).cast::<u16>().bitor(y).to_array(),
                        p2.shl(n).cast::<u16>().bitor(z).to_array(),
                    )
                });

                let gg = me
                    .array_chunks::<64>()
                    .zip(g)
                    .flat_map(|(m, (p0, p1, p2))| {
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
        bfs.push_back(next.clone());
        res.extend(next);
    }

    res
}
