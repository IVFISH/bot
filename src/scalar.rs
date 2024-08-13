use crate::game::*;
use std::cmp::*;

/// Does a BFS to find all future game states from a given Game.
/// Searches until the queue is empty.
pub fn search_scalar(game: Game) -> Vec<Game> {
    let mut bfs = vec![game];

    while !bfs.is_empty() {
        // take all from que with the same Game.queue
        // iterate the placements with SIMD
        let (piece, new_queue) = Game::next(bfs[0].queue);

        if piece == 0 {
            break;
        };

        bfs = process_scalar(&bfs, piece, new_queue);
    }

    bfs
}

pub fn process_scalar(work: &[Game], piece: usize, new_queue: usize) -> Vec<Game> {
    let mut next = Vec::new();
    for rot in 0..4 {
        // generate the bitmask for piece and rot
        let p_bitmask = PIECES[piece - 1][rot];

        // TODO: include col 0, 9 :)
        for col in 1..9 {
            let pz0 = p_bitmask[0].trailing_zeros() as i8;
            let pz1 = p_bitmask[1].trailing_zeros() as i8;
            let pz2 = p_bitmask[2].trailing_zeros() as i8;
            // mask the piece onto the game's board
            for &g in work {
                let g = process_game(g, p_bitmask, pz0, pz1, pz2, col, new_queue);
                next.push(g);
            }
        }
    }
    next
}

#[inline(always)]
fn process_game(
    mut g: Game,
    p_bitmask: [u16; 3],
    pz0: i8,
    pz1: i8,
    pz2: i8,
    col: usize,
    new_queue: usize,
) -> Game {
    let x = g.board[col - 1];
    let y = g.board[col + 0];
    let z = g.board[col + 1];

    let i = (16 - x.leading_zeros() as i8) - pz0;
    let j = (16 - y.leading_zeros() as i8) - pz1;
    let k = (16 - z.leading_zeros() as i8) - pz2;

    let n = max(max(i, j), k);

    g.board[col - 1] |= p_bitmask[0] << n;
    g.board[col + 0] |= p_bitmask[1] << n;
    g.board[col + 1] |= p_bitmask[2] << n;

    Game {
        board: g.board,
        queue: new_queue,
    }
}
