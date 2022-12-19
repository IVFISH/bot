#![allow(dead_code)]

mod board;
mod bot;
mod communications;
mod constants;
mod game;
mod human;
mod piece;
mod players;
mod point_vector;
mod population;
mod queue;
mod versus;
mod weight;
mod opener;

use crate::bot::*;
use crate::players::Player;
use std::{thread, time};
use std::collections::VecDeque;
use crate::board::Board;
use crate::constants::types::Dependencies;
use crate::constants::versus_constants::AttackType::T;
use crate::game::Game;
use crate::piece::Piece;
use crate::weight::Weights;
use crate::point_vector::Point;
use crate::opener::*;

fn test_opener() {

    let test = VecDeque::from(vec!([1, 2, 3]));

    let opener_sequence = [
        Piece { piece_type: 0, rotation_state: 0, center: Point(1, 4), last_kick: 0 },
        Piece { piece_type: 1, rotation_state: 1, center: Point(1, 0), last_kick: 0 },
        Piece { piece_type: 2, rotation_state: 0, center: Point(0, 8), last_kick: 0 },
        Piece { piece_type: 3, rotation_state: 1, center: Point(1, 6), last_kick: 0 },
        Piece { piece_type: 4, rotation_state: 0, center: Point(0, 4), last_kick: 0 },
        Piece { piece_type: 5, rotation_state: 2, center: Point(3, 4), last_kick: 0 },
        Piece { piece_type: 6, rotation_state: 0, center: Point(0, 4), last_kick: 0 }, // always invalid
    ];

    let dependency = Dependency {dependency: vec![4, 0, 5, 6]};

    let mut opener = Opener::new(vec![opener_sequence], vec!(vec![dependency]));
    println!("{:?}", opener.status);
    opener.init(&vec![2, 0, 3, 4, 5, 6, 1]);
    println!("{:?}", opener.status);
    println!("{}", opener.next_placement(&vec![2, 0, 3, 4, 5, 6, 1]));
    println!("{}", opener.next_placement(&vec![2, 0, 3, 4, 5, 6, 1]));
    println!("{}", opener.next_placement(&vec![2, 0, 3, 4, 5, 6, 1]));
    println!("{}", opener.next_placement(&vec![2, 0, 3, 4, 5, 6, 1]));
    println!("{}", opener.next_placement(&vec![2, 0, 3, 4, 5, 6, 1]));
    println!("{}", opener.next_placement(&vec![2, 0, 3, 4, 5, 6, 1]));
    println!("{}", opener.next_placement(&vec![2, 0, 3, 4, 5, 6, 1]));
    println!("{:?}", opener.status);

    println!("unexpected stuff");
    let mut test = Opener::default();



    // let test = opener.solve_bag(&vec![2, 0, 3, 4, 5, 6, 1]);
    // println!("{}", test);
}

fn main() {
    // test_opener()
    // bot_play();
    tetrio_play()
    // more_test();
    // ndt_test();
}

fn ndt_test() {
    // 0 1 2 3 4 5 6
    // Z L O S I J T
    const NDT1: [Piece; 7] = [
        Piece{piece_type: 0, rotation_state: 0, center: Point(1,8), last_kick: 0},
        Piece{piece_type: 1, rotation_state: 0, center: Point(0,4), last_kick: 0},
        Piece{piece_type: 2, rotation_state: 0, center: Point(0,0), last_kick: 0},
        Piece{piece_type: 3, rotation_state: 0, center: Point(1,4), last_kick: 0},
        Piece{piece_type: 4, rotation_state: 1, center: Point(2,6), last_kick: 0},
        Piece{piece_type: 5, rotation_state: 0, center: Point(0,8), last_kick: 0},
        Piece{piece_type: 6, rotation_state: 0, center: Point(3,4), last_kick: 0},
    ];
    let DEP1: Dependencies = vec![
        Dependency{ dependency: vec![1, 3, 6] }, // L<S<T
        Dependency{ dependency: vec![5, 0] },    // J<Z
    ];
    const NDT2: [Piece; 7] = [
        Piece{piece_type: 0, rotation_state: 1, center: Point(5,6), last_kick: 0},
        Piece{piece_type: 1, rotation_state: 3, center: Point(5,3), last_kick: 0},
        Piece{piece_type: 2, rotation_state: 0, center: Point(3,7), last_kick: 0},
        Piece{piece_type: 3, rotation_state: 1, center: Point(5,4), last_kick: 0},
        Piece{piece_type: 4, rotation_state: 1, center: Point(4,9), last_kick: 0},
        Piece{piece_type: 5, rotation_state: 1, center: Point(0,3), last_kick: 0},
        Piece{piece_type: 6, rotation_state: 2, center: Point(2,2), last_kick: 0},
    ];
    let DEP2: Dependencies = vec![            // can be improved, idc
        Dependency{ dependency: vec![2, 0] },   // O<Z
        Dependency{ dependency: vec![0, 6] },   // T must be last
        Dependency{ dependency: vec![1, 6] },
        Dependency{ dependency: vec![2, 6] },
        Dependency{ dependency: vec![3, 6] },
        Dependency{ dependency: vec![4, 6] },
        Dependency{ dependency: vec![5, 6] },
    ];
    let ndt = Opener::new(vec![NDT1, NDT2], vec![DEP1, DEP2]);
}

fn more_test() {
    let mut bot = Bot::default();
    bot.get_game_mut().active_piece = Piece::new(2);
    bot.get_game_mut().piece_queue.set_queue(VecDeque::from([0, 3, 4, 5, 6, 1]));
    println!("{}", bot);
    bot.make_n_moves(6);
    println!("{}", bot);
    bot.make_move();
    println!("{}", bot);


}
fn bot_play() {
    let mut bot = Bot::default();
    println!("{}", bot.get_game().board.get_arr().len());

    let mut time = 0;
    while !bot.get_game().get_game_over() && bot.get_game().game_data.pieces_placed < 10000 {

        let now = time::Instant::now();
        bot.make_move();
        time += now.elapsed().as_micros();

        thread::sleep(time::Duration::from_millis(0));
        // println!("{}", bot.get_game());
        println!("{}", bot.get_game());

    }
    println!(
        "Making {} moves took {} microseconds on average",
        bot.get_game().game_data.pieces_placed,
        time / (bot.get_game().game_data.pieces_placed as u128)
    );
    println!("{}", bot.get_game());
}

// fn bot_play() {
//     let mut bot = Bot::default();
//     let mut game = bot.get_game_mut();
//     game.set_active_piece(Piece::new(6));
//     println!("{}", game);
//     game.board.add(0,0);
//     game.board.add(1, 0);
//     game.board.add(2, 0);
//     game.board.add(0, 1);
//     game.board.add(0, 3);
//     game.board.add(2, 3);
//     game.board.add(0, 4);
//     game.board.add(1, 4);
//     game.board.add(2, 4);
//     game.board.add(0, 5);
//     game.board.add(1, 5);
//     game.board.add(2, 5);
//     game.board.add(0, 6);
//     game.board.add(1, 6);
//     game.board.add(2, 6);
//     game.board.add(0, 7);
//     game.board.add(1, 7);
//     game.board.add(2, 7);
//     game.board.add(0, 8);
//     game.board.add(1, 8);
//     game.board.add(2, 8);
//     game.board.add(0, 9);
//     game.board.add(1, 9);
//     game.board.add(2, 9);
//     println!("{}", game);
//     let placements = Bot::move_placement_score_1d(game, &Default::default()).1;
//     for placement in placements{
//         game.board.add_list(placement.abs_locations().unwrap());
//         println!("{}", game);
//         game.board.remove_list(placement.abs_locations().unwrap());
//     }
// }

fn tetrio_play() {
    communications::init()
}
