#![allow(dead_code)]
use crate::game::Game;
use crate::piece::Piece;

#[derive(Clone, Debug)]
pub struct Placement {
    pub game: Game, // game after the piece has been placed
    pub base_piece: Piece,
    pub pos_eval: i16,
    pub versus_eval: i16,
}

impl Ord for Placement {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.pos_eval + self.versus_eval).cmp(&(other.pos_eval + other.versus_eval))
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
        (self.pos_eval + self.versus_eval).eq(&(other.pos_eval + other.versus_eval))
    }
}

impl Placement {
    pub fn new(game: Game) -> Self {
        Self {
            game,
            base_piece: Piece::default(),
            pos_eval: 0,
            versus_eval: 0,
        }
    }

    pub const fn new_base(game: Game, base_piece: Piece) -> Self {
        Self {
            game,
            base_piece,
            pos_eval: 0,
            versus_eval: 0,
        }
    }
}
