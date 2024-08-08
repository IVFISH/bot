#![feature(portable_simd)]
#![feature(array_repeat)]
#![feature(slice_as_chunks)]

mod game;

use game::*;
use std::cmp::*;
use std::collections::*;
use std::time;

use std::simd::prelude::*;

const PIECES: [[[usize; 3]; 4]; 3] = [
    // T
    [
        // N
        [0b010, 0b011, 0b010],
        // E
        [0b000, 0b111, 0b010],
        // S
        [0b001, 0b011, 0b001],
        // W
        [0b010, 0b111, 0b000],
    ],
    // S
    [
        // N
        [0b010, 0b011, 0b001],
        // E
        [0b000, 0b011, 0b110],
        // S
        [0b010, 0b011, 0b001],
        // W
        [0b011, 0b110, 0b000],
    ],
    // L
    [
        // N
        [0b010, 0b010, 0b011],
        // E
        [0b000, 0b111, 0b100],
        // S
        [0b011, 0b001, 0b001],
        // W
        [0b001, 0b111, 0b000],
    ],
];

/// Does a BFS to find all future game states from a given Game.
/// Searches until the queue is empty.
fn search_scalar(game: Game) -> Vec<Game> {
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

        let mut next = Vec::new();

        for rot in 0..4 {
            // generate the bitmask for piece and rot
            let p_bitmask = PIECES[piece - 1][rot];

            // columns to check are {col - 1, col, col + 1}
            let skip0 = (p_bitmask[0] == 0) as i8;
            let skip2 = (p_bitmask[2] == 0) as i8;

            // TODO: include col 0, 9 :)
            for col in 1..9 {
                for &(mut g) in &me {
                    // mask the piece onto the game's board

                    let x = g.board[col - 1];
                    let y = g.board[col];
                    let z = g.board[col + 1];

                    let i = (16 - x.leading_zeros() as i8) - (p_bitmask[0].trailing_zeros() as i8);
                    let j = (16 - y.leading_zeros() as i8) - (p_bitmask[1].trailing_zeros() as i8);
                    let k = (16 - z.leading_zeros() as i8) - (p_bitmask[2].trailing_zeros() as i8);

                    let n = max(max(i * skip0, j), k * skip2);

                    g.board[col - 1] |= (p_bitmask[0] << n) as u16;
                    g.board[col] |= (p_bitmask[1] << n) as u16;
                    g.board[col + 1] |= (p_bitmask[2] << n) as u16;

                    res.push(Game {
                        board: g.board,
                        queue: new_queue,
                    });

                    next.push(Game {
                        board: g.board,
                        queue: new_queue,
                    });
                }
            }
        }
        bfs.push_back(next);
    }

    res
}

fn main() {
    // note: the queue is backwards (also 1-indexed)
    let game = Game::new(0x21_123);

    let now = time::Instant::now();
    let res = search_scalar(game);
    println!(
        "found {} boards in {} microseconds",
        res.len(),
        now.elapsed().as_micros()
    );

    println!("{:?}", res.last().unwrap().board);
}
