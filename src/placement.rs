#![allow(dead_code)]

use core::f32;

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
        Self {
            game,
            base_piece: Piece::default(),
            eval: f32::MAX
        }
    }

    pub fn new_base(game: Game, base_piece: Piece) -> Self {
        Self {
            game,
            base_piece,
            eval: f32::MAX
        }
    }
}
