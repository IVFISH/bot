use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tetris::board::*;
use tetris::bot::*;
use tetris::constants::board_constants::*;
use tetris::pruner::*;
use tetris::test_api::functions::*;

pub fn movegen_benchmark_no_pruning(c: &mut Criterion) {
    let mut bot = Bot::<NoPruner>::with_seed(3);

    let mut group1 = c.benchmark_group("depth 1");
    group1.bench_function("movegen empty board depth=1", |b| {
        b.iter(|| black_box(bot.move_gen(1)))
    });

    bot.game.board = l_spin_board_5();
    group1.bench_function("movegen l-spin-fuckery board depth=1", |b| {
        b.iter(|| black_box(bot.move_gen(1)))
    });
    group1.finish();

    let mut group2 = c.benchmark_group("depth 3");
    group2.sample_size(60);
    bot.game.board = Board::new();
    group2.bench_function("movegen empty board depth=3", |b| {
        b.iter(|| black_box(bot.move_gen(3)))
    });

    bot.game.board = l_spin_board_5();
    group2.bench_function("movegen l-spin-fuckery board depth=3", |b| {
        b.iter(|| black_box(bot.move_gen(3)))
    });
    group2.finish();
}

pub fn movegen_benchmark_pc_pruning(c: &mut Criterion) {
    let mut bot = Bot::<AllClearPruner>::with_seed(4);
    bot.game.board = pco_board();
    c.bench_function("pco start find-pcs depth=3", |b| {
        b.iter(|| black_box(bot.move_gen(3)))
    });

    bot.game.board = pco_board_1();
    c.bench_function("pco start find-pcs depth=5", |b| {
        b.iter(|| black_box(bot.move_gen(5)))
    });

    bot.game.board = pco_board_2();
    let mut group = c.benchmark_group("pco");
    group.sample_size(10);
    group.bench_function("pco start find-pcs depth=7", |b| {
        b.iter(|| black_box(bot.move_gen(7)))
    });
    group.finish();
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

// criterion_group!(benches, movegen_benchmark_no_pruning);
criterion_group!(benches, movegen_benchmark_pc_pruning);
// criterion_group!(benches, clearlines_benchmark);
criterion_main!(benches);
