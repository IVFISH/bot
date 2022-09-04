use crate::constants::board_constants::*;
use crate::piece::Piece;
use std::cmp::max;

#[derive(Debug)]
pub struct Board {
    arr: [[bool; BOARD_WIDTH]; BOARD_HEIGHT],
    column_heights: [usize; BOARD_WIDTH],
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
        unimplemented!()
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
            // TODO: update heights
        }

        self.arr[row][col] = true;
    }

    pub fn add_list(&mut self, locations: Vec<(usize, usize)>, update_heights: bool) {
        let _ = locations.iter().map(|&(row, col)| self.add(row, col, update_heights));
    }

    pub fn remove(&mut self, row: usize, col: usize, update_heights: bool) {
        if update_heights && self.arr[row][col] {
            // TODO: update heights
        }

        self.arr[row][col] = false;
    }

    pub fn remove_list(&mut self, locations: Vec<(usize, usize)>, update_heights: bool) {
        let _ = locations.iter().map(|&(row, col)| self.remove(row, col, update_heights));
    }

    pub fn set_row(&mut self, row: usize, new_row: [bool; BOARD_WIDTH], update_heights: bool) {
        self.arr[row] = new_row;

        if update_heights {
            let _ = (0..BOARD_WIDTH).into_iter().map(|col| self.update_height_add(col, row));
        }
    }

    pub fn remove_row(&mut self, row: usize, update_heights: bool) {
        self.arr[row] = [false; BOARD_WIDTH];

        if update_heights {
            let _ = (0..BOARD_WIDTH).into_iter().map(|col| self.update_height_remove(col));
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
        if let Some(height) = self.arr[col].iter().rposition(|&x| x) {
            self.column_heights[col] = height + 1;
            return;
        }
        self.column_heights[col] = 0;
    }

    fn sub_to_heights(&mut self, amt: usize) {
        let _ = (0..BOARD_WIDTH).into_iter().map(
            |col| self.column_heights[col] -= amt
        );
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

        let _ = full_rows.iter().map(|&row| self.remove_row(row, false));

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
    
}
