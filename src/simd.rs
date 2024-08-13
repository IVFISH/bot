use crate::game::*;
use crate::scalar::*;

use std::ops::*;
use std::simd::prelude::*;

pub fn search_simd(game: Game) -> [Vec<u16>; 10] {
    let (initial_p, mut new_queue) = Game::next(game.queue);
    let initial = process_scalar(&[game], initial_p, new_queue);

    // transpose initial
    let mut res = [
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
    ];
    for i in 0..10 {
        res[i] = initial.iter().map(|x| x.board[i]).collect();
    }

    while new_queue != 0 {
        let (p, q) = Game::next(new_queue);
        new_queue = q;
        res = search_simd_step(res, p - 1);
    }

    res
}

// processes transposed games :)
pub fn search_simd_step(boards: [Vec<u16>; 10], piece: usize) -> [Vec<u16>; 10] {
    let size = boards.len() * 32;
    let mut res = [
        Vec::with_capacity(size),
        Vec::with_capacity(size),
        Vec::with_capacity(size),
        Vec::with_capacity(size),
        Vec::with_capacity(size),
        Vec::with_capacity(size),
        Vec::with_capacity(size),
        Vec::with_capacity(size),
        Vec::with_capacity(size),
        Vec::with_capacity(size),
    ];

    for rot in 0..4 {
        let p_bitmask = PIECES[piece][rot];

        for col in 1..9 {
            let x = &boards[col - 1];
            let y = &boards[col - 0];
            let z = &boards[col + 1];

            // inline to make it reference instead of owned
            let sixteen = u16x32::splat(16);
            let ii = i16x32::splat(p_bitmask[0].trailing_zeros() as i16);
            let jj = i16x32::splat(p_bitmask[1].trailing_zeros() as i16);
            let kk = i16x32::splat(p_bitmask[2].trailing_zeros() as i16);

            // chunking x -> Simd types
            let x = x.array_chunks::<32>().map(|&x| u16x32::from_array(x));
            let y = y.array_chunks::<32>().map(|&y| u16x32::from_array(y));
            let z = z.array_chunks::<32>().map(|&z| u16x32::from_array(z));

            // process game (on single column)
            let p0 = u16x32::splat(p_bitmask[0]);
            let p1 = u16x32::splat(p_bitmask[1]);
            let p2 = u16x32::splat(p_bitmask[2]);

            let g = x.zip(y).zip(z).map(|((x, y), z)| {
                let nx = ((sixteen - x.leading_zeros()).cast::<i16>() - ii).cast::<u16>();
                let ny = ((sixteen - y.leading_zeros()).cast::<i16>() - jj).cast::<u16>();
                let nz = ((sixteen - z.leading_zeros()).cast::<i16>() - kk).cast::<u16>();
                let n = nx.simd_max(ny).simd_max(nz);

                (
                    x.bitor(p0 << n).to_array(),
                    (y.bitor(p1 << n).to_array(), z.bitor(p2 << n).to_array()),
                )
            });

            let (x, (y, z)): (Vec<[u16; 32]>, (Vec<[u16; 32]>, Vec<[u16; 32]>)) = g.unzip();

            for i in 0..(col - 1) {
                res[i].extend(&boards[i]);
            }

            res[col - 1].extend(x.as_flattened());
            res[col - 0].extend(y.as_flattened());
            res[col + 1].extend(z.as_flattened());
            for i in (col + 1)..10 {
                res[i].extend(&boards[i]);
            }
        }
    }

    res
}
