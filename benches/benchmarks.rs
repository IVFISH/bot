use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::time::Duration;
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

pub fn eval_benchmark(c: &mut Criterion) {
    use tetris::board::Board;
    bench_versus(c, "heights", |b| {
        black_box(b.get_heights());
    });

    bench_versus(c, "tslot", |b| {
        black_box(Board::t_slot(&b.arr));
    });
}

fn bench_versus<F>(c: &mut Criterion, name: &str, eval: F)
where
    F: Fn(Board),
{
    let board_short = versus_board_short();
    let board_med = versus_board_medium();
    let board_tall = versus_board_tall();
    let mut group = c.benchmark_group(name);
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(3));

    group.bench_function("short", |b| b.iter(|| black_box(eval(board_short))));
    group.bench_function("med", |b| b.iter(|| black_box(eval(board_med))));
    group.bench_function("tall", |b| b.iter(|| black_box(eval(board_tall))));

    group.finish();
}

criterion_group!(benches, movegen_benchmark_no_pruning);
// criterion_group!(benches, movegen_benchmark_pc_pruning);
// criterion_group!(benches, clearlines_benchmark);
// criterion_group!(benches, eval_benchmark);
criterion_main!(benches);
