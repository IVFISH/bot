use bot::constants::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

use bot::nontrivials::*;
use bot::test_api::test_api::*;

pub fn movegen_components(c: &mut Criterion) {
    let game = game_from_string(&[""], PIECE_T as u64); // T PIECE
    let piece = game.peek() - 1 as usize;
    let cmaps = collision(game.board, piece);
    let trivials = cmaps.map(|x| trivial(x));
    c.bench_function("rotate with kick: 180 T kicks, blank board", |b| {
        b.iter(|| rotate_kick(trivials[0], cmaps[2], &KICKS[0][0][2]))
    });

    let cmap = collision(l_spin_game_2().board, 1)[0];
    c.bench_function("find grounded: l_spin_game_2 cmap(L)", |b| {
        b.iter(|| black_box(cmap.grounded()))
    });

    let to_clear = 0b10110000;
    let board = l_spin_game_2().board;
    c.bench_function("clear lines: l_spin_game_2", |b| {
        b.iter(|| board.clearrows(black_box(to_clear)))
    });
}

pub fn movegen_1d(c: &mut Criterion) {
    let game = game_from_string(&[""], PIECE_T as u64);
    let piece = game.peek() - 1 as usize;
    let cmaps = collision(game.board, piece);

    c.bench_function("collision map gen: T, blank board", |b| {
        b.iter(|| collision(black_box(game.board), black_box(piece)))
    });

    c.bench_function("reachable gen: T, blank board", |b| {
        b.iter(|| reachable(black_box(cmaps), black_box(piece)))
    });

    c.bench_function("full movegen: T, blank board", |b| {
        b.iter(|| movegen(black_box(game)))
    });

    let game = l_spin_game_2();
    let piece = game.peek() - 1 as usize;
    let cmaps = collision(game.board, piece);

    c.bench_function("collision map gen: L, L spin board 2", |b| {
        b.iter(|| collision(black_box(game.board), black_box(piece)))
    });

    c.bench_function("reachable gen: L, L spin board 2", |b| {
        b.iter(|| reachable(black_box(cmaps), black_box(piece)))
    });

    c.bench_function("full movegen: L, L spin board 2", |b| {
        b.iter(|| movegen(black_box(game)))
    });
}

criterion_group!(benches, movegen_components, movegen_1d);
criterion_main!(benches);
