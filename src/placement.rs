#![allow(dead_code)]

use core::f32;

use crate::board::Board;
use crate::game::Game;
use crate::piece::Piece;

#[derive(Clone, Debug)]
pub struct Placement {
    pub game: Game, // game after the piece has been placed
    pub base_piece: Piece,
    pub eval: f32,
}

impl Ord for Placement {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.eval.total_cmp(&other.eval)
    }
}

impl Eq for Placement {}

impl PartialOrd for Placement {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Placement {
    fn eq(&self, other: &Self) -> bool {
        self.eval.eq(&other.eval)
    }
}

impl Placement {
    pub fn new(game: Game) -> Self {
        let mut out = Self {
            game,
            base_piece: Piece::default(),
            eval: f32::MAX,
        };
        out.eval();
        out
    }

    pub fn new_base(game: Game, base_piece: Piece) -> Self {
        let mut out = Self {
            game,
            base_piece,
            eval: f32::MAX,
        };
        out.eval();
        out
    }

    // Eval functions ---------------
    // TODO: Change this so its not lazily evaluated. Currently only called when sorting, resulting in single-threaded eval
    pub fn eval(&mut self) {
        let board = self.game.board.arr;

        let max = Board::get_max_height(&board);
        let min = Board::get_min_height(&board);
        let holes = Board::get_total_holes(&board);

        let messiness = Board::get_messiness(&board);
        let coveredness = Board::get_cell_coveredness(&board);

        let adj_diff = Board::get_adjacent_height_differences(&board);
        let stack_diff = Board::get_stack_height_differences(&board);

        // TODO: figure out why tslot isn't working
        let tslot = Board::t_slot(&board);
        // let tslot = 0;

        // (10 * max as u32 + holes) as f32

        self.eval = (max + min + messiness + adj_diff + stack_diff - 5 * tslot + (holes + coveredness) as usize)
            as f32
    }
}
