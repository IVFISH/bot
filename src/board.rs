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
        if let Some(locs) = piece.abs_locations() {
            for [row, col] in locs {
                self.add(row, col);
            }
        }
    }

    /// removes the minos that occupy the piece's location
    pub fn remove_piece(&mut self, piece: &Piece) {
        if let Some(locs) = piece.abs_locations() {
            for [row, col] in locs {
                self.remove(row, col);
            }
        }
    }

    /// returns whether the piece minos can be shifted downwards
    pub fn piece_grounded(&self, piece: &Piece) -> bool {
        if let Some(locs) = piece.abs_locations() {
            locs.iter()
                .any(|&[row, col]| row == 0 || self.get(row - 1, col))
        } else {
            false
        }
    }

    /// returns whether the piece has a collision inside the grid
    pub fn piece_collision(&self, piece: &Piece) -> bool {
        if let Some(locs) = piece.abs_locations() {
            locs.iter().any(|&[row, col]| self.get(row, col))
        } else {
            false
        }
    }

    /// returns whether the piece has no collision and is grounded
    pub fn piece_can_set(&self, piece: &Piece) -> bool {
        !self.piece_collision(piece) && self.piece_grounded(piece)
    }

    // aux methods ------------------------------
    /// clears all filled lines on the board and moves down
    /// the blocks above those lines
    pub fn clear_lines(&mut self) {
        let full_rows = self.arr.into_iter().reduce(|x, y| x & y).unwrap();
        for i in 0..BOARD_WIDTH {
            let mut rows = full_rows; // copy
            self.arr[i] &= !rows; // delete the cleared rows
            while rows != 0 {
                let c = self.arr[i];
                let r = rows.trailing_zeros();
                let m = 1 << r;
                let m1 = m - 1 >> 1;
                self.arr[i] = c >> 1 & !m1 | (c & m1);
                rows = (rows ^ m) << 1;
            }
        }
    }

    // private methods --------------------------
    /// sets the state at the row and col to 0
    fn remove(&mut self, row: usize, col: usize) {
        self.arr[col] &= !(1 << row);
    }

    /// sets the state at the row and col to 1
    fn add(&mut self, row: usize, col: usize) {
        self.arr[col] |= 1 << row;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_api::functions::*;

    //#[test]
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

        board.set_row(7, [true; BOARD_WIDTH]);
        println!("{}", board);
        assert_eq!(board.get_heights(), [8, 8, 8, 9, 8, 8, 8, 8, 8, 8]);
        println!("{}", board);
        board.clear_lines();
        println!("{}", board);
        assert_eq!(board.get_heights(), [0, 0, 6, 8, 0, 0, 0, 0, 0, 0]);
    }

    //#[test]
    fn test_clear_multiple_lines() {
        let mut board = Board::new();

        add_list(&mut board, vec![[5, 2], [3, 2], [5, 3], [10, 3]]);
        board.set_row(8, [true; BOARD_WIDTH]);
        board.set_row(7, [true; BOARD_WIDTH]);
        println!("{}", board);
        board.clear_lines();
        println!("{}", board);
        assert_eq!(board.get_heights(), [0, 0, 6, 9, 0, 0, 0, 0, 0, 0]);
    }

    //#[test]
    fn test_piece_can_set() {
        let mut board = Board::new();
        println!("{}", board);

        let mut p = Piece::new(6);
        p.rotate(2);

        assert!(!board.piece_can_set(&p));
        board.set_row(4, [true; BOARD_WIDTH]);
        add_list(&mut board, vec![[5, 3], [5, 5]]);
        p.set_row(6);
        assert!(board.piece_can_set(&p));
        p.set_row(5);
        assert!(!board.piece_can_set(&p));
        p.set_row(4);
        assert!(!board.piece_can_set(&p));
        p.set_row(3);
        assert!(!board.piece_can_set(&p));
        p.set_row(1);
        assert!(board.piece_can_set(&p));
        p.set_row(0);
        assert!(!board.piece_can_set(&p));
    }

    //#[test]
    fn test_heights() {
        let mut board = Board::new();

        add_list(&mut board, vec![[3, 2], [5, 2], [5, 3]]);
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
}
