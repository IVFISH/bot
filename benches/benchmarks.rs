use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tetris::board::*;
use tetris::bot::*;
use tetris::constants::board_constants::*;

pub fn movegen_benchmark(c: &mut Criterion) {
    let bot = Bot::new();
    c.bench_function("movegen empty board", |b| {
        b.iter(|| black_box(bot.move_gen()))
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

// criterion_group!(benches, movegen_benchmark);
criterion_group!(benches, clearlines_benchmark);
criterion_main!(benches);
