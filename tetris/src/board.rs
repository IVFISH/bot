#![allow(dead_code)]

use std::fmt::{Display, Formatter};
use crate::piece::Piece;

const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 40;
const VISIBLE_BOARD_HEIGHT: usize = 23;

#[derive(Debug, Clone, Default)]
pub struct Board {
    pub arr: [u64; BOARD_WIDTH],
}


impl Display for Board {
    /// returns a string representation of the board
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in (0..VISIBLE_BOARD_HEIGHT).rev() {
            for col in 0..BOARD_WIDTH {
                if self.get(row, col) {
                    write!(f, "■ ")?
                } else {
                    write!(f, "□ ")?
                }
            }
            write!(f, "\n")?
        }
        Ok(())
    }
}


impl Board {
    // constructors -----------------------------
    /// returns an empty board with all empty cells
    pub fn new() -> Self {
        Default::default()
    }

    // getters ----------------------------------
    /// returns the indices of the first empty row in each column
    pub fn get_heights(&self) -> [usize; BOARD_WIDTH] {
        let mut out = [0; BOARD_WIDTH];
        for col in 0..BOARD_WIDTH {
            out[col] = self.get_height(col);  
        }
        out
    }

    /// returns the index of the first empty row in column col
    pub fn get_height(&self, col: usize) -> usize {
        (usize::BITS - self.arr[col].leading_zeros()) as usize
    }
    
    /// returns the state (0 or 1) at the grid's row and col
    pub fn get(&self, row: usize, col: usize) -> bool {
        (self.arr[col] >> row & 1) > 0
    }

    // setters ----------------------------------
    /// sets the cell at the row and col to the specified state
    pub fn set(&mut self, row: usize, col: usize, state: usize) {
        if state == 0 {
            self.remove(row, col)
        } else {
            self.add(row, col)
        }
    }
    
    // piece API --------------------------------
    /// returns whether a piece can be set
    /// does not change the board in the case of not set
    /// sets the four minos of the piece if setable
    pub fn set_piece(&mut self, piece: &Piece) -> bool {
        unimplemented!()
    }
    
    /// removes the minos that occupy the piece's location
    pub fn remove_piece(&mut self, piece: &Piece) -> bool {
        unimplemented!()
    }
    
    /// returns whether the piece minos are within the grid
    pub fn piece_in_bounds(&self, piece: &Piece) -> bool {
        unimplemented!()
    }

    /// returns whether the piece minos can be shifted downwards
    pub fn piece_grounded(&self, piece: &Piece) -> bool {
        unimplemented!()
    }
    
    /// returns whether the piece has a collision inside the grid
    pub fn piece_collision(&self, piece: &Piece) -> bool {
        unimplemented!()
    }

    /// returns whether the piece has no collision and is grounded
    pub fn piece_can_set(&self, piece: &Piece) -> bool {
        unimplemented!()
    }
    
    // aux methods ------------------------------
    /// clears all lines on the board
    pub fn clear_lines(&mut self) {
        unimplemented!()
    }

    // private methods --------------------------
    /// sets the state at the row and col to 0
    fn remove(&mut self, row: usize, col: usize) {
        self.arr[col] &= !(1 << row);
    }
    
    /// sets the state at the row and col to 1
    fn add(&mut self, row: usize, col: usize) {
        self.arr[col] |= !(1 << row);
    }

}
