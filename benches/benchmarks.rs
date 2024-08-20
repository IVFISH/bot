use criterion::{black_box, criterion_group, criterion_main, Criterion};

use bot::bitmatrix::Bitmatrix;
use bot::game::*;
use bot::nontrivials::*;
use bot::test_api::test_api::*;

pub fn movegen_1d(c: &mut Criterion) {
    let game = Game::new(1);
    let piece = Game::next(game.queue).0 - 1;
    let (cmap, trivials) = collision(game.board, piece);

    c.bench_function("collision map gen: T, blank board", |b| {
        b.iter(|| collision(black_box(game.board), black_box(piece)))
    });

    c.bench_function("reachable gen: T, blank board", |b| {
        b.iter(|| reachable(black_box(cmap), black_box(trivials), black_box(piece)))
    });

    c.bench_function("full movegen: T, blank board", |b| {
        b.iter(|| movegen(black_box(game)))
    });

    let game = l_spin_board_2();
    let piece = Game::next(game.queue).0 - 1;
    let (cmap, trivials) = collision(game.board, piece);

    c.bench_function("collision map gen: L, L spin board 2", |b| {
        b.iter(|| collision(black_box(game.board), black_box(piece)))
    });

    c.bench_function("reachable gen: L, L spin board 2", |b| {
        b.iter(|| reachable(black_box(cmap), black_box(trivials), black_box(piece)))
    });

    c.bench_function("full movegen: L, L spin board 2", |b| {
        b.iter(|| movegen(black_box(game)))
    });
}

criterion_group!(benches, movegen_1d);
criterion_main!(benches);
