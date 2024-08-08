mod game;

use drain_while::*;
use game::*;
use std::time;

/// Does a BFS to find all future game states from a given Game.
/// Searches until the queue is empty.
fn search(game: Game) -> Vec<Game> {
    let mut res = Vec::new();
    let mut bfs = vec![game];

    while !bfs.is_empty()
    {
        // take all from que with the same Game.queue
        // iterate the placements with SIMD
        let queue = bfs[0].queue;
        let (piece, new_queue) = Game::next(queue);
        let me: Vec<_> = bfs.drain_while(|x| x.queue == queue).collect();

        for rot in 0..4 {
            for col in 1..9 {
                for g in &me {
                    res.push(g.place(piece, rot, col, new_queue));
                }
            }
        }
    }

    res
}

fn main() {
    let game = Game::new(0x12345_54321);

    let now = time::Instant::now();
    let res = search(game);
    println!("found {} boards in {}", res.len(), now.elapsed().as_micros());
    // println!("Hello, world!");
}
