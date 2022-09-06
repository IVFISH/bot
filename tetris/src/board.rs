#![allow(dead_code)]

use crate::constants::board_constants::*;
use crate::piece::Piece;
use crate::point_vector::{Point, PointVector};
use std::cmp::max;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
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

impl Default for Board {
    fn default() -> Self {
        Self {
            arr: [[false; BOARD_WIDTH]; BOARD_HEIGHT],
            column_heights: [0; BOARD_WIDTH],
        }
    }
}

impl Board {
    pub fn new() -> Self {
        Default::default()
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

    pub fn add_list(&mut self, locations: Vec<Point>, update_heights: bool) {
        for Point(row, col) in locations {
            self.add(row as usize, col as usize, update_heights)
        }
    }

    pub fn remove(&mut self, row: usize, col: usize, update_heights: bool) {
        if update_heights && self.arr[row][col] {
            self.arr[row][col] = false;
            self.update_height_remove(col)
        }
    }

    pub fn remove_list(&mut self, locations: Vec<Point>, update_heights: bool) {
        for Point(row, col) in locations {
            self.remove(row as usize, col as usize, update_heights)
        }
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
    pub fn set_piece(&mut self, piece: &Piece, update_heights: bool) -> bool {
        let mut out = false;
        if let Some(locations) = piece.abs_locations() {
            out = !self.piece_collision(piece);
            self.add_list(locations, update_heights);
        }
        out
    }

    pub fn remove_piece(&mut self, piece: &Piece, update_heights: bool) {
        if let Some(locations) = piece.abs_locations() {
            self.remove_list(locations, update_heights);
        }
    }

    pub fn piece_in_bounds(&self, piece: &Piece) -> bool {
        piece.abs_locations().is_some()
    }

    pub fn piece_collision(&self, piece: &Piece) -> bool {
        let locations = piece.abs_locations();

        locations.is_some()
            && locations
                .unwrap()
                .iter()
                .map(|&Point(row, col)| self.get(row as usize, col as usize))
                .any(|x| x)
    }

    pub fn piece_grounded(&self, piece: &Piece) -> bool {
        if let Some(down) = piece.ret_moved(PointVector(-1, 0)) {
            return self.piece_collision(&down);
        };
        true
    }

    pub fn piece_valid_location(&self, piece: &Piece) -> bool {
        self.piece_in_bounds(piece) && !self.piece_collision(piece)
    }

    pub fn piece_valid_placement(&self, piece: &Piece) -> bool {
        self.piece_valid_location(piece) && self.piece_grounded(piece)
    }

    // update heights
    fn update_height_add(&mut self, col: usize, row: usize) {
        self.column_heights[col] = max(self.column_heights[col], row + 1)
    }

    fn update_height_remove(&mut self, col: usize) {
        if let Some(height) = self.get_col(col).iter().rposition(|&x| x) {
            self.column_heights[col] = height + 1;
            return;
        }
        self.column_heights[col] = 0;
    }

    fn update_all_heights(&mut self) {
        for col in 0..BOARD_WIDTH {
            self.update_height_remove(col);
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
    pub fn top_out(&mut self, piece: &Piece, next: &Piece) -> bool {
        self.set_piece(piece, false);
        if self.piece_collision(next) {
            return true;
        }
        self.remove_piece(piece, false);
        !piece
            .abs_locations()
            .unwrap()
            .iter()
            .any(|&x| x.0 < MAX_PLACE_HEIGHT as i8)
    }

    pub fn all_clear(&self) -> bool {
        // note: may fail edge cases
        !self.get_row(0).contains(&true)
    }

    pub fn clear_lines(&mut self, update_heights: bool) -> usize {
        let full_rows = self.all_full_rows();
        let highest = self.get_max_height();
        let num_cleared = full_rows.len();

        for &row in &full_rows {
            self.remove_row(row, false);
        }

        for &row in full_rows.iter().rev() {
            for r in row..highest {
                self.set_row(r, self.get_row(r + 1), false);
            }
        }

        if update_heights {
            self.update_all_heights();
        }

        num_cleared
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
        let mut holes_count_total = 0;
        let mut holes_count_weighted = 0;
        let mut cell_covered_count = 0;

        for col in 0..BOARD_WIDTH {
            // only counting when you find a filled block
            let mut counting = true;
            let mut covered_counter = 0;

            for row in (0..self.column_heights[col]).rev() {
                // start at top

                let spot = self.get(row, col);
                // hole
                if !spot {
                    cell_covered_count += covered_counter;

                    holes_count_total += 1;

                    if counting {
                        holes_count_weighted += 1;
                        counting = false;
                    }
                } else {
                    covered_counter += 1;
                    counting = true;
                }
            }
        }

        (holes_count_total, holes_count_weighted, cell_covered_count)
    }

    pub fn t_slot(&self) -> usize {
        unimplemented!()
    }

    pub fn get_max_height_difference(&self) -> usize {
        self.get_max_height() - self.get_min_height()
    }

    pub fn get_adjacent_height_differences(&self) -> Vec<usize> {
        self.column_heights
            .windows(2)
            .map(|w| w[0].abs_diff(w[1]))
            .collect()
    }

    // other
    pub fn display_with_active(&self, active_piece: &Piece) -> String {
        let mut out = String::new();
        let locations = active_piece.abs_locations().unwrap();
        for row in (0..VISIBLE_BOARD_HEIGHT).rev() {
            for col in 0..BOARD_WIDTH {
                if self.get(row, col) {
                    out.push_str("■ ");
                } else if locations.contains(&Point(row as i8, col as i8)) {
                    out.push_str("⬚ ");
                } else {
                    out.push_str("□ ");
                }
            }
            out.push_str("\n");
        }

        out
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

        board.add_list(vec![Point(5, 2), Point(3, 2), Point(5, 3)], true);
        assert_eq!(board.get_heights(), [0, 0, 6, 6, 0, 0, 0, 0, 0, 0]);
        board.remove_list(vec![Point(5, 2), Point(5, 3)], true);
        assert_eq!(board.get_heights(), [0, 0, 4, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_heights_2() {
        let mut board = Board::new();

        board.set_row(8, [true; BOARD_WIDTH], true);
        board.add_list(vec![Point(5, 2), Point(3, 2), Point(5, 3)], true);
        assert_eq!(board.get_heights(), [9; BOARD_WIDTH]);
        board.remove_row(8, true);
        assert_eq!(board.get_heights(), [0, 0, 6, 6, 0, 0, 0, 0, 0, 0]);

        board.set_row(8, [true; BOARD_WIDTH], true);
        board.add(9, 3, true);
        board.clear_lines(true);
        assert_eq!(board.get_heights(), [0, 0, 6, 9, 0, 0, 0, 0, 0, 0]);

        board.set_row(6, [true; BOARD_WIDTH], true);
        board.set_row(7, [true; BOARD_WIDTH], true);
        assert_eq!(board.get_heights(), [8, 8, 8, 9, 8, 8, 8, 8, 8, 8]);
        board.clear_lines(true);
        assert_eq!(board.get_heights(), [0, 0, 6, 7, 0, 0, 0, 0, 0, 0]);
    }
}
