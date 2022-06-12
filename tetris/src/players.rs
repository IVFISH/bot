use std::io;

use crate::game::*;
use crate::board::*;
use crate::placement::*;

pub trait Player {

    fn make_move(&mut self);
    fn get_next_move(&self);
    fn score_board(&self) -> i32;
    fn score(&self) -> i32;

}