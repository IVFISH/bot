#![allow(dead_code)]

use crate::board::Board;
use crate::piece::Piece;

#[derive(Default, Copy, Clone, Debug)]
pub struct Game {
    pub board: Board,
    pub active: Piece,
    pub hold: Option<u8>,
    // game stats/game data
}

impl Game {
    // constructors -----------------------------
    /// returns a game with an empty board and random queue
    pub fn new() -> Self {
        unimplemented!()
    }

    /// updates the current game, checking for line clears and changing stats accordingly
    pub fn update(&mut self) {
        let lines_cleared = self.board.clear_lines();

        if lines_cleared == 0 {
            return;
        }
        // update stats
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_api::functions::*;

    // test update
}
