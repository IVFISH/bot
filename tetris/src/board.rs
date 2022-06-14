use std::fmt::{Formatter, Display};

use std::cmp::max;

use crate::errors::GameError;
use crate::placement::{Placement, Point, MoveVector};
use crate::queue::GarbageItem;


pub const BOARD_WIDTH: usize = 10;
pub const BOARD_HEIGHT: usize = 40;

pub struct Board {
    width: usize,
    height: usize,

    arr: [[bool; BOARD_WIDTH]; BOARD_HEIGHT],
    heights_for_each_column: [usize; BOARD_WIDTH],
}

impl Board {
    pub fn get_board_array(&self) -> [[bool; BOARD_WIDTH]; BOARD_HEIGHT] {
        self.arr
    }

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

    pub fn set_piece(&mut self, piece: &Placement, update_heights: bool) {
        for location in piece.abs_locations().unwrap() {
            self.add(location.row, location.col, update_heights);
        }
    }

    pub fn remove_piece(&mut self, piece: &Placement, update_heights: bool) {
        for location in piece.abs_locations().unwrap() {
            self.remove(location.row, location.col, update_heights);
        }
    }

    pub fn max_filled_height(&self) -> usize {
        *self.heights_for_each_column.iter().max().unwrap()
    }

    pub fn min_filled_height(&self) -> usize {
        *self.heights_for_each_column.iter().min().unwrap()
    }

    pub fn col_in_bounds(col: usize) -> Result<(), GameError> {
        if col < BOARD_WIDTH {
            Ok(())
        } else {
            Err(GameError::NotInBounds)
        }
    }

    pub fn row_in_bounds(row: usize) -> Result<(), GameError> {
        if row < BOARD_HEIGHT {
            Ok(())
        } else {
            Err(GameError::NotInBounds)
        }
    }

    pub fn in_bounds(row: usize, col: usize) -> Result<(), GameError> {
        Board::col_in_bounds(col)?;
        Board::row_in_bounds(row)?;

        Ok(())
    }

    pub fn new() -> Self {
        Default::default()
    }

    pub fn check_collision(&self, piece: &Placement) -> Result<(), GameError> {
        let locations = piece.abs_locations()?;
        for location in locations {
            if self.get(location.row, location.col) {
                return Err(GameError::Collision);
            }
        }
        Ok(())
    }

    pub fn check_piece_in_bounds(&self, piece: &Placement) -> Result<(), GameError> {
        let locations = piece.abs_locations()?;

        let in_bounds = locations
            .iter()
            .all(
                |loc| Board::in_bounds(loc.row, loc.col).is_ok()
            );

        if !in_bounds {
            return Err(GameError::NotInBounds);
        }

        Ok(())
    }

    pub fn check_valid_location(&self, piece: &Placement) -> Result<(), GameError> {
        self.check_piece_in_bounds(piece)?;
        self.check_collision(piece)?;
        Ok(())
    }

    pub fn check_grounded(&self, piece: &mut Placement) -> Result<(), GameError> {
        piece.move_by_vector(MoveVector(-1, 0));

        if let Err(e) = self.check_valid_placement(piece) {
            piece.move_by_vector(MoveVector(1, 0));
            return Err(e);
        }
        piece.move_by_vector(MoveVector(1, 0));
        Ok(())
    }

    pub fn check_valid_placement(&self, piece: &mut Placement) -> Result<(), GameError> {
        self.check_valid_location(piece)?;
        self.check_grounded(piece)?;
        Ok(())
    }

    pub fn all_clear(&self) -> bool {
        !self.get_row(0).contains(&true)
    }

    pub fn clear(&mut self) {
        self.heights_for_each_column = [0; BOARD_WIDTH];
        self.arr = [[false; BOARD_WIDTH]; BOARD_HEIGHT];
    }

    pub fn add_garbage(&mut self, garbage: GarbageItem, update_heights: bool) {
        // iterate downwards
        for row in (0..self.max_filled_height()).rev() {
            // move each row up
            self.set_row(row + garbage.amt, self.get_row(row), false);
        }

        // adding garbage
        for row in 0..garbage.amt {
            self.set_row(row, [true; 10], false);

            // making a hole at the column
            self.remove(row, garbage.col, false);
        }

        if update_heights {
            self.add_to_heights(garbage.amt);
        }
    }

    pub fn clear_lines(&mut self, update_heights: bool) -> usize {
        let full_rows = self.all_full_rows();
        let num_full_rows = self.all_full_rows().len();
        let highest = self.max_filled_height();

        for row in &full_rows {
            self.remove_row(*row, false);
        }

        for row in full_rows.iter().rev() {
            for r in *row..highest {
                self.set_row(r, self.get_row(r + 1), false);
            }
        }

        if update_heights {
            self.sub_to_heights(num_full_rows);
        }

        num_full_rows
    }

    fn add_to_heights(&mut self, amt: usize) {
        for col in 0..self.width {
            self.heights_for_each_column[col] += amt;
        }
    }

    fn sub_to_heights(&mut self, amt: usize) {
        for col in 0..self.width {
            self.heights_for_each_column[col] -= amt;
        }
    }

    pub fn top_out(&mut self, piece: &Placement, next: &Placement, max_height: usize) -> Result<(), GameError> {
        self.set_piece(piece, false);

        if self.check_collision(next).is_err() {
            return Err(GameError::TopOut);
        }

        self.remove_piece(piece, false);

        if piece.abs_locations()
            .unwrap()
            .iter()
            .all(
                |x| x.row >= max_height) {
            return Err(GameError::TopOut);
        }

        Ok(())
    }

    fn full_row(&self, row: usize) -> bool {
        self.arr[row].iter().all(|x| *x)
    }

    fn all_full_rows(&self) -> Vec<usize> {
        (0..self.max_filled_height())
            .into_iter()
            .filter(
                |x| self.full_row(*x)
            )
            .collect()
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

    pub fn to_string(&self, active_piece: &Placement) -> String {
        let mut out = String::new();

        let locations = active_piece.abs_locations().unwrap();
        for row in (0..self.height / 2 + 3).rev() {
            for col in 0..self.width {
                if self.get(row, col) {
                    out.push_str("■ ");
                } else if locations.contains(&Point { row, col }) {
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
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        for row in (0..self.height / 2 + 3).rev() {
            for col in 0..self.width {
                if self.get(row, col) {
                    write!(f, "■ ")?
                } else { write!(f, "□ ")? }
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
        assert!(Board::row_in_bounds(BOARD_HEIGHT).is_err());
        assert!(Board::row_in_bounds(5).is_ok());

        assert!(Board::col_in_bounds(BOARD_WIDTH).is_err());
        assert!(Board::col_in_bounds(5).is_ok());

        assert!(Board::in_bounds(1, 1).is_ok());
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

    #[test]
    fn set_piece() {
        let mut board = Board::new();
        let piece = create_preset_piece();
        board.set_piece(&piece, true);

        assert!(board.get(2, 3));
        assert!(board.get(2, 1));
        assert!(!board.get(4, 3));
    }

    fn create_preset_board() -> Board {
        let mut board = Board::new();

        board.add(1, 1, true);
        board.add(1, 2, true);
        board.add(5, 1, true);
        board.add(3, 7, true);

        board
    }

    fn create_preset_piece() -> Placement {
        Placement {
            piece_type: 4,
            rotation_state: 2,
            center: Point { row: 2, col: 2 },
        }
    }
}
