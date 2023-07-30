#![allow(dead_code)]

use crate::constants::board_constants::*;
use crate::piece::Piece;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Default, Copy)]
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

    pub fn set_row(&mut self, row: usize, data: [bool; BOARD_WIDTH]) {
        for (col, state) in data.into_iter().enumerate() {
            self.set(row, col, state as usize);
        }
    }

    pub fn remove_row(&mut self, row: usize) {
        self.set_row(row, [false; BOARD_WIDTH]);
    }

    // piece API --------------------------------
    /// sets the four minos of the piece
    pub fn set_piece(&mut self, piece: &Piece) {
        unimplemented!()
    }

    /// removes the minos that occupy the piece's location
    pub fn remove_piece(&mut self, piece: &Piece) {
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

    // static -----------------------------------
    /// returns whether the piece minos are within the grid
    pub fn piece_in_bounds(piece: &Piece) -> bool {
        if let Some(locations) = piece.abs_locations() {
            !locations
                .iter()
                .any(|&[row, col]| Self::in_bounds_row(row as i8) && Self::in_bounds_col(col as i8))
        } else {
            false
        }
    }

    /// bounds checking on the row
    pub fn in_bounds_row(row: i8) -> bool {
        (row >= 0) && (row as usize <= BOARD_HEIGHT)
    }

    /// bounds checking on the col
    pub fn in_bounds_col(col: i8) -> bool {
        (col >= 0) && (col as usize <= BOARD_WIDTH)
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

#[cfg(test)]
mod tests {
    use super::*;

    fn add_list(board: &mut Board, list: Vec<[usize; 2]>) {
        for [r, c] in list.into_iter() {
            board.add(r, c);
        }
    }

    fn remove_list(board: &mut Board, list: Vec<[usize; 2]>) {
        for [r, c] in list.into_iter() {
            board.remove(r, c);
        }
    }

    #[test]
    fn test_heights() {
        let mut board = Board::new();

        board.add(5, 2);
        board.add(3, 2);
        board.add(5, 3);
        println!("{}", board);
        println!("{:?}", board.get_heights());

        assert_eq!(board.get_heights(), [0, 0, 6, 6, 0, 0, 0, 0, 0, 0]);

        board.remove(5, 2);
        println!("{}", board);
        assert_eq!(board.get_heights()[2], 4);
        board.remove(5, 3);
        println!("{}", board);
        assert_eq!(board.get_heights()[3], 0);
        assert_eq!(board.get_heights(), [0, 0, 4, 0, 0, 0, 0, 0, 0, 0]);

        let mut board = Board::new();

        add_list(&mut board, vec![[5, 2], [3, 2], [5, 3]]);
        assert_eq!(board.get_heights(), [0, 0, 6, 6, 0, 0, 0, 0, 0, 0]);
        remove_list(&mut board, vec![[5, 2], [5, 3]]);
        assert_eq!(board.get_heights(), [0, 0, 4, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_clear_lines() {
        let mut board = Board::new();

        board.set_row(8, [true; BOARD_WIDTH]);
        add_list(&mut board, vec![[5, 2], [3, 2], [5, 3]]);
        assert_eq!(board.get_heights(), [9; BOARD_WIDTH]);
        board.remove_row(8);
        assert_eq!(board.get_heights(), [0, 0, 6, 6, 0, 0, 0, 0, 0, 0]);

        board.set_row(8, [true; BOARD_WIDTH]);
        board.add(9, 3);
        println!("{}", board);
        board.clear_lines();
        println!("{}", board);
        assert_eq!(board.get_heights(), [0, 0, 6, 9, 0, 0, 0, 0, 0, 0]);

        board.set_row(6, [true; BOARD_WIDTH]);
        board.set_row(7, [true; BOARD_WIDTH]);
        assert_eq!(board.get_heights(), [8, 8, 8, 9, 8, 8, 8, 8, 8, 8]);
        board.clear_lines();
        assert_eq!(board.get_heights(), [0, 0, 6, 7, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_clear_multiple_lines() {
        let mut board = Board::new();

        add_list(&mut board, vec![[5, 2], [3, 2], [5, 3], [10, 3]]);
        board.set_row(8, [true; BOARD_WIDTH]);
        board.set_row(7, [true; BOARD_WIDTH]);
        println!("{}", board);
        board.clear_lines();
        println!("{}", board);
        assert_eq!(board.get_heights(), [0, 0, 6, 9, 0, 0, 0, 0, 0, 0]);

        board.set_row(6, [true; BOARD_WIDTH]);
        board.set_row(7, [true; BOARD_WIDTH]);
        assert_eq!(board.get_heights(), [8, 8, 8, 9, 8, 8, 8, 8, 8, 8]);
        board.clear_lines();
        assert_eq!(board.get_heights(), [0, 0, 6, 7, 0, 0, 0, 0, 0, 0]);
    }

}
