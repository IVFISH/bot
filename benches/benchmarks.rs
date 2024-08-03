use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tetris::board::*;
use tetris::bot::*;
use tetris::constants::board_constants::*;
use tetris::piece;
use tetris::piece_queue::PieceQueue;
use tetris::test_api::functions::l_spin_board_5;
use tetris::lookahead::many_lookahead;

pub fn movegen_benchmark(c: &mut Criterion) {
    let mut bot = Bot::new();
    c.bench_function("movegen empty board d=1", |b| {
        b.iter(|| black_box(many_lookahead(bot.game, 1)))
    });

    let mut bot = Bot::new();
    c.bench_function("movegen empty board d=3", |b| {
        b.iter(|| black_box(many_lookahead(bot.game, 3)))
    });

    let mut bot = Bot::new();
    bot.game.board = l_spin_board_5();
    c.bench_function("movegen l spin fuckery board d=1", |b| {
        b.iter(|| black_box(many_lookahead(bot.game, 1)))
    });

    let mut bot = Bot::new();
    bot.game.board = l_spin_board_5();
    c.bench_function("movegen l spin fuckery board d=3", |b| {
        b.iter(|| black_box(many_lookahead(bot.game, 3)))
    });
}

pub fn clearlines_benchmark(c: &mut Criterion) {
    c.bench_function("clearing_lines", |b| {
        b.iter(|| {
            let mut board = Board::new();

            add_list(&mut board, vec![[5, 2], [3, 2], [5, 3], [10, 3]]);
            board.set_row(8, [true; BOARD_WIDTH]);
            board.set_row(7, [true; BOARD_WIDTH]);
            board.clear_lines();
        })
    });
}

fn add_list(board: &mut Board, list: Vec<[usize; 2]>) {
    for [r, c] in list.into_iter() {
        board.set(r, c, 1);
    }
}

pub fn piece_queue_benchmark(c: &mut Criterion) {
    let mut q = PieceQueue::new(3);
    c.bench_function("piece queue generation", |b| {
        b.iter(|| black_box(q.next()))
    });

    c.bench_function("piece queue copy", |b| {
        b.iter(|| black_box(q.clone()))
    });
}
criterion_group!(benches, movegen_benchmark);
// criterion_group!(benches, clearlines_benchmark);
// criterion_group!(benches, piece_queue_benchmark);
criterion_main!(benches);
