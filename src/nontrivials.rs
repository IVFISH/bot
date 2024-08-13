use bitvec::view::BitViewSized;

use crate::bitmatrix::*;
use crate::game::*;

/// generate collision map
/// TODO: implement other rotations
pub fn collision(game: Game) -> Bitmatrix {
    let m = MASKS[Game::next(game.queue).0 - 1][0];
    let mut p = [0u16; W];

    for col in 1..(W-1) {
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

    Bitmatrix {
        data: p.into_bitarray().to_bitvec(),
    }
}

pub fn reachable(collision: Bitmatrix) -> Bitmatrix {
    // find the initial possible matrix
    // then iterate the actions
    const N: usize = 5;
    let mut r = Bitmatrix::new();
    r.data.set(79, true); // starting
    for _ in 0..N {
        // todo: break early if doesn't change
        r = r.or(collision.and((r.lshift()).or(r.rshift()).or(r.ushift()).or(r.dshift())));
    }
    r
}

pub fn to_game_vec(game: Game, reachable: Bitmatrix) -> Vec<Game> {
    let mut ret = Vec::with_capacity(reachable.data.count_ones());
    let (p, q) = Game::next(game.queue);
    let piece = PIECES[p][0];

    let copy = Game { board: game.board, queue: q };

    for i in reachable.data.iter_ones() {
        let (r, c) = (i % H, i / H);

        let mut cpy = copy;
        cpy.board[c - 1] |= piece[0] << r;
        cpy.board[c - 0] |= piece[1] << r;
        cpy.board[c + 1] |= piece[2] << r;
        ret.push(cpy);
    }

    ret
}

pub fn movegen(game: Game) -> Vec<Game> {
    to_game_vec(game, reachable(collision(game)))
}
