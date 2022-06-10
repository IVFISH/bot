use std::fmt::{Formatter, Result, Display};

use itertools::Itertools;

const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 20;

pub struct Board {
        width: usize,
        height: usize,

        arr: [[u8; BOARD_WIDTH]; BOARD_HEIGHT]
    }

impl Board {

    pub fn get(&self, row: usize, col: usize) -> u8 {
        self.arr[row][col]
    }

    pub fn set(&mut self, row: usize, col: usize, new: u8) {
        self.arr[row][col] = new;
    }

    pub fn col_in_bounds(&self, col: usize) -> bool {
        col < self.width
    }

    pub fn row_in_bounds(&self, row: usize) -> bool {
        row < self.height
    }

    pub fn in_bounds(&self, row: usize, col: usize) -> bool {
        self.col_in_bounds(col) && self.row_in_bounds(row)
    }

    pub fn new() -> Self {
        Default::default()
    }
}
impl Default for Board {

    fn default() -> Self {
        Self {
            width: BOARD_WIDTH,
            height: BOARD_HEIGHT,
            arr: [[0; BOARD_WIDTH]; BOARD_HEIGHT]
        }
    }
}

impl Display for Board {

    fn fmt(&self, f: &mut Formatter) -> Result {
        for row in (0..self.height).rev() {
            for col in 0..self.width {
                match self.get(row, col) {
                    0 => write!(f, "□ ")?,
                    1 => write!(f, "■ ")?,
                    _ => unreachable!("There are only two states")
                }
            }
            write!(f, "\n")?
        }

        Ok(())
    }
}

#[cfg(test)]
mod board_tests {
    use super::*;

    #[test]
    fn in_bounds() {
        let board = Board::new();
        assert!(!board.row_in_bounds(BOARD_HEIGHT));
        assert!(board.row_in_bounds(5));

        assert!(!board.col_in_bounds(BOARD_WIDTH));
        assert!(board.col_in_bounds(5));

        assert!(board.in_bounds(1, 1));

    }

}
