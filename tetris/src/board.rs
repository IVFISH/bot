use crate::constants::board_constants::*;
use crate::piece::Piece;
use std::cmp::max;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Board {
    arr: [[bool; BOARD_WIDTH]; BOARD_HEIGHT],
    column_heights: [usize; BOARD_WIDTH],
}

impl Display for Board {
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
    pub fn new() -> Self {
        Self {
            arr: [[false; BOARD_WIDTH]; BOARD_HEIGHT],
            column_heights: [0; BOARD_WIDTH],
        }
    }

    // getters
    pub fn get_arr(&self) -> [[bool; BOARD_WIDTH]; BOARD_HEIGHT] {
        self.arr
    }

    pub fn get_heights(&self) -> [usize; BOARD_WIDTH] {
        self.column_heights
    }

    pub fn get(&self, row: usize, col: usize) -> bool {
        self.arr[row][col]
    }

    pub fn get_row(&self, row: usize) -> [bool; BOARD_WIDTH] {
        self.arr[row]
    }

    pub fn get_col(&self, col: usize) -> [bool; BOARD_HEIGHT] {
        let mut out = [false; BOARD_HEIGHT];


        for row in 0..BOARD_HEIGHT {
            out[row] = self.get(row, col);
        }

        out
    }

    pub fn get_max_height(&self) -> usize {
        *self.column_heights.iter().max().unwrap()
    }

    pub fn get_min_height(&self) -> usize {
        *self.column_heights.iter().min().unwrap()
    }

    // setters
    pub fn add(&mut self, row: usize, col: usize, update_heights: bool) {
        if update_heights && !self.arr[row][col] {
            self.update_height_add(col, row);
        }

        self.arr[row][col] = true;
    }

    pub fn add_list(&mut self, locations: Vec<(usize, usize)>, update_heights: bool) {
        for (row, col) in locations{
            self.add(row, col, update_heights)
        };
    }

    pub fn remove(&mut self, row: usize, col: usize, update_heights: bool) {
        if update_heights && self.arr[row][col] {
            self.arr[row][col] = false;
            self.update_height_remove(col)
        }
    }

    pub fn remove_list(&mut self, locations: Vec<(usize, usize)>, update_heights: bool) {
        for (row, col) in locations{
            self.remove(row, col, update_heights)
        };
    }

    pub fn set_row(&mut self, row: usize, new_row: [bool; BOARD_WIDTH], update_heights: bool) {
        self.arr[row] = new_row;

        if update_heights {
            for col in 0..BOARD_WIDTH {
                self.update_height_add(col, row);
            }
        }
    }

    pub fn remove_row(&mut self, row: usize, update_heights: bool) {
        self.arr[row] = [false; BOARD_WIDTH];

        if update_heights {
            for col in 0..BOARD_WIDTH {
                self.update_height_remove(col);
            }
        }
    }

    // piece interactions
    pub fn set_piece(&mut self, piece: &Piece, update_heights: bool) {
        unimplemented!()
    }

    pub fn remove_piece(&mut self, piece: &Piece, update_heights: bool) {
        unimplemented!()
    }

    pub fn piece_in_bounds(&self, piece: &Piece) -> bool {
        unimplemented!()
    }

    pub fn piece_collision(&self, piece: &Piece) -> bool {
        unimplemented!()
    }

    pub fn piece_grounded(&self, piece: &Piece) -> bool {
        unimplemented!()
    }

    pub fn piece_valid_location(&self, piece: &Piece) -> bool {
        unimplemented!()
    }

    pub fn piece_valid_placement(&self, piece: &Piece) -> bool {
        unimplemented!()
    }

    // update heights
    fn update_height_add(&mut self, col: usize, row: usize) {
        self.column_heights[col] = max(self.column_heights[col], row + 1)
    }

    fn update_height_remove(&mut self, col: usize) {
        println!("{:?}", self.get_col(col));
        if let Some(height) = self.get_col(col).iter().rposition(|&x| x) {
            self.column_heights[col] = height + 1;
            return;
        }
        self.column_heights[col] = 0;
    }

    fn sub_to_heights(&mut self, amt: usize) {
        for col in 0..BOARD_WIDTH {
            self.column_heights[col] -= amt
        }
    }

    // bounds checking
    fn col_in_bounds(col: usize) -> bool {
        col < BOARD_WIDTH
    }

    fn row_in_bounds(row: usize) -> bool {
        row < BOARD_HEIGHT
    }

    pub fn in_bounds(row: usize, col: usize) -> bool {
        Board::col_in_bounds(col) && Board::row_in_bounds(row)
    }

    // versus
    pub fn all_clear(&self) -> bool {
        // note: may fail edge cases
        !self.get_row(0).contains(&true)
    }

    pub fn clear_lines(&mut self, update_heights: bool) -> usize {
        let full_rows = self.all_full_rows();
        let highest = self.get_max_height();

        for &row in &full_rows {
            self.remove_row(row, false);
        }

        // iterate top down
        let mut index = 0;
        for row in 0..highest {
            if row == full_rows[index] {
                index += 1;
            }
            self.set_row(row, self.get_row(row + index), false);
        }

        assert_eq!(index, full_rows.len());

        if update_heights {
            self.sub_to_heights(index);
        }

        index
    }

    fn full_row(&self, row: usize) -> bool {
        self.arr[row].iter().all(|x| *x)
    }

    fn all_full_rows(&self) -> Vec<usize> {
        (0..self.get_max_height())
            .into_iter()
            .filter(|x| self.full_row(*x))
            .collect()
    }

    // stats
    pub fn holes_cell_covered(&self) -> (usize, usize, usize) {
        unimplemented!()
    }

    pub fn t_slot(&self) -> usize {
        unimplemented!()
    }

    pub fn max_height_difference(&self) -> usize {
        self.get_max_height() - self.get_min_height()
    }

    pub fn get_adjacent_height_differences(&self) -> Vec<usize> {
        // maybe make output an array

        self.column_heights
            .windows(2)
            .map(|w| w[0].abs_diff(w[1]))
            .collect()
    }
}

#[cfg(test)]
mod board_tests {
    use super::*;

    #[test]
    fn test_heights() {
        let mut board = Board::new();

        board.add(5, 2, true);
        board.add(3, 2, true);
        board.add(5, 3, true);

        assert_eq!(board.get_heights(), [0, 0, 6, 6, 0, 0, 0, 0, 0, 0]);

        board.remove(5, 2, true);
        assert_eq!(board.get_heights()[2], 4);
        board.remove(5, 3, true);
        assert_eq!(board.get_heights()[3], 0);
        assert_eq!(board.get_heights(), [0, 0, 4, 0, 0, 0, 0, 0, 0, 0]);

        let mut board = Board::new();

        board.add_list(vec![(5, 2), (3, 2), (5, 3)], true);
        assert_eq!(board.get_heights(), [0, 0, 6, 6, 0, 0, 0, 0, 0, 0]);
        board.remove_list(vec![(5, 2), (5, 3)], true);
        assert_eq!(board.get_heights(), [0, 0, 4, 0, 0, 0, 0, 0, 0, 0]);

    }

    #[test]
    fn test_heights_2() {
        // set row, remove row
        // clear lines
        // piece stuffs
    }
}
