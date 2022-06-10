use std::fmt::{Formatter, Result, Display};

use std::cmp::max;

use crate::placement::Placement;



const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 20;

pub struct Board {
        width: usize,
        height: usize,

        arr: [[bool; BOARD_WIDTH]; BOARD_HEIGHT],
        heights_for_each_column: [usize; BOARD_WIDTH],
    }

impl Board {

    pub fn add(&mut self, row: usize, col: usize, update_heights: bool) {
        self.arr[row][col] = true;

        if update_heights {
            self.update_height_add(col, row);
        }
    }

    pub fn remove(&mut self, row: usize, col: usize, update_heights: bool) {
        self.arr[row][col] = false;

        if update_heights {
            self.update_height_remove(col)
        }
    }

    pub fn get(&self, row: usize, col: usize) -> bool {
        self.arr[row][col]
    }

    pub fn get_row(&self, row: usize) -> [bool; BOARD_WIDTH] {
        self.arr[row]
    }

    pub fn set_row(&mut self, row: usize, new_row: [bool; BOARD_WIDTH], update_heights: bool) {
        self.arr[row] = new_row;

        if update_heights {
            for column in 0..self.width {
                self.update_height_add(column, row);
            }
        }
    }

    pub fn remove_row(&mut self, row: usize, update_heights: bool) {
        self.arr[row] = [false; BOARD_WIDTH];

        if update_heights {
            for column in 0..self.width {
                self.update_height_remove(column);
            }
        }
    }

    pub fn set_piece(&mut self, piece: Placement, update_heights: bool) {
        for location in piece.to_list() {
            self.add(location.row, location.col, update_heights);
        }
    }

    pub fn remove_piece(&mut self, piece: Placement, update_heights: bool) {
        for location in piece.to_list() {
            self.remove(location.row, location.col, update_heights);
        }
    }

    pub fn max_filled_height(&self) -> usize {
        *self.heights_for_each_column.iter().max().unwrap()
    }

    pub fn min_filled_height(&self) -> usize {
        println!("{:?}", self.heights_for_each_column);
        *self.heights_for_each_column.iter().min().unwrap()
    }

    pub fn col_in_bounds(col: usize) -> bool {
        col < BOARD_WIDTH
    }

    pub fn row_in_bounds(row: usize) -> bool {
        row < BOARD_HEIGHT
    }

    pub fn in_bounds(row: usize, col: usize) -> bool {
        Board::col_in_bounds(col) && Board::row_in_bounds(row)
    }

    pub fn new() -> Self {
        Default::default()
    }

    pub fn check_collision(&self, piece: Placement) -> bool {
        for location in piece.to_list() {
            if self.get(location.row, location.col) {
                return true;
            }
        }
        false
    }

    pub fn check_grounded(&self, piece: Placement) -> bool {
        let locations = piece.to_list();
        unimplemented!()
    }

    pub fn all_clear(&self) -> bool {
        !self.get_row(0).contains(&true)
    }

    pub fn clear(&mut self) {
        self.heights_for_each_column = [0; BOARD_WIDTH];
        self.arr = [[false; BOARD_WIDTH]; BOARD_HEIGHT];

    }

    pub fn add_garbage(&self) {

    }

    pub fn clear_lines(&self) -> u8 {
        unimplemented!()
    }

    pub fn top_out(&self) -> bool {
        unimplemented!()
    }

    fn full_row(&self, row: usize) -> bool {
        unimplemented!()
    }

    fn update_height_add(&mut self, col: usize, new: usize) {
        let height = self.heights_for_each_column[col];
        self.heights_for_each_column[col] = max(height, new + 1);
    }

    fn update_height_remove(&mut self, col: usize) {
        for row in (0..self.heights_for_each_column[col]).rev() {
            if self.get(row, col) {
                self.heights_for_each_column[col] = row + 1;
                break;
            }
        }

        self.heights_for_each_column[col] = 0;
    }
}

impl Default for Board {

    fn default() -> Self {
        Self {
            width: BOARD_WIDTH,
            height: BOARD_HEIGHT,
            arr: [[false; BOARD_WIDTH]; BOARD_HEIGHT],
            heights_for_each_column: [0; BOARD_WIDTH],
        }
    }
}

impl Display for Board {

    fn fmt(&self, f: &mut Formatter) -> Result {
        for row in (0..self.height as usize).rev() {
            for col in 0..self.width as usize {
                if self.get(row, col) {
                    write!(f, "■ ")?
                } 
                else { write!(f, "□ ")? }
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

    #[test]
    fn get_and_set() {
        let board = create_preset_board();

        assert!(board.get(1, 1));
        assert!(board.get(5, 1));
        assert!(!board.get(3, 4));
        assert!(!board.get(12, 5));

    }

    #[test]
    fn remove() {
        let mut board = create_preset_board();

        assert!(board.get(1, 1));
        board.remove(1, 1, false);
        assert!(!board.get(1, 1));
    }

    #[test]
    fn max_height() {
        let mut board = create_preset_board();
        assert_eq!(board.max_filled_height(), 6);

        board.remove(5, 1, true);
        assert_eq!(board.max_filled_height(), 4);
    }

    #[test]
    fn min_height() {
        let mut board = create_preset_board();
        assert_eq!(board.min_filled_height(), 0);

        board.set_row(0, [true; BOARD_WIDTH], true);
        assert_eq!(board.min_filled_height(), 1);

        board.remove_row(0, true);
        assert_eq!(board.min_filled_height(), 0);
    }

    fn create_preset_board() -> Board {
        let mut board = Board::new();

        board.add(1, 1, true);
        board.add(1, 2, true);
        board.add(5, 1, true);
        board.add(3, 7, true);

        board

    }

}
