use crate::game::*;
use std::cmp::*;
use std::collections::*;

/// Does a BFS to find all future game states from a given Game.
/// Searches until the queue is empty.
pub fn search_scalar(game: Game) -> Vec<Game> {
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

                    let n = max(max(i, j), k);

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
