use bitvec::view::BitViewSized;

use crate::bitmatrix::*;
use crate::game::*;

/// generate collision map and the trivials
/// TODO: implement other rotations
pub fn collision(game: Game) -> (Bitmatrix, Bitmatrix) {
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

    let collisions = Bitmatrix {
        data: p.into_bitarray().to_bitvec(),
    };

    for col in 0..W {
        let q = p[col];
        let l = q.leading_zeros();
        p[col] = q & (u16::MAX << l);
    }

    let trivials = Bitmatrix {
        data: p.into_bitarray().to_bitvec(),
    };

    (collisions, trivials)
}

pub fn reachable(collision: Bitmatrix, trivials: Bitmatrix) -> Bitmatrix {
    // find the initial possible matrix
    // then iterate the actions
    const N: usize = 20;
    let mut r = trivials;

    for _ in 0..N {
        let q = r.or(collision.and((r.lshift()).or(r.rshift()).or(r.ushift()).or(r.dshift())));
        if q == r { break;}
        r = q;
    }

    r.and(r.ushift().not())
}

pub fn to_game_vec(game: Game, reachable: Bitmatrix) -> Vec<Game> {
    let mut ret = Vec::with_capacity(reachable.data.count_ones());
    let (p, q) = Game::next(game.queue);
    let piece = PIECES[p - 1][0];

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
    let (c, t) = collision(game);
    to_game_vec(game, reachable(c, t))
}
