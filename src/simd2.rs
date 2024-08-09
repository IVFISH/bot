use crate::game::*;
use crate::scalar::*;

use std::ops::*;
use std::simd::prelude::*;

pub fn search_simd2(game: Game) -> [Vec<u16>; 10] {
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

    for rot in 0..4 {
        let p_bitmask = PIECES[piece][rot];

        for col in 1..9 {
            let x = &boards[col - 1];
            let y = &boards[col - 0];
            let z = &boards[col + 1];

            // inline to make it reference instead of owned
            let (xx, yy, zz) = search_simd_helper(x, y, z, p_bitmask);

            for i in 0..(col - 1) {
                res[i].extend(&boards[i]);
            }
            res[col - 1].extend(xx);
            res[col - 0].extend(yy);
            res[col + 1].extend(zz);
            for i in (col + 1)..10 {
                res[i].extend(&boards[i]);
            }
        }
    }

    res
}

fn search_simd_helper(
    x: &[u16],
    y: &[u16],
    z: &[u16],
    p_bitmask: [u16; 3],
) -> (Vec<u16>, Vec<u16>, Vec<u16>) {
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

        (
            x.bitor(p0 << n).to_array(),
            (y.bitor(p1 << n).to_array(), z.bitor(p2 << n).to_array()),
        )
    });

    let (x, (y, z)): (Vec<[u16; 8]>, (Vec<[u16; 8]>, Vec<[u16; 8]>)) = g.unzip();
    (x.into_flattened(), y.into_flattened(), z.into_flattened())
}
