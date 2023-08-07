#![allow(dead_code)]

use crate::constants::board_constants::*;
use crate::constants::types::*;
use crate::piece::*;
use crate::queue::{piece_type_to_string_colored};
use crate::point_vector::{Point, PointVector};
use std::fmt::{Display, Formatter};
use colored::*;
use num::clamp;
use crate::weight::Weights;
use crate::constants::localbotgameplay::*;
use std::fs;

#[derive(Debug, Clone)]
pub struct Board {
    arr: BoardArray,
    garbage_in_queue: usize,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            arr: [0; BOARD_WIDTH],
            garbage_in_queue: 0,
        }
    }
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
        Default::default()
    }

    // getters

    pub fn get_arr(&self) -> BoardArray {
        self.arr
    }

    pub fn get_heights(&self) -> Vec<usize> {
        (0..BOARD_WIDTH).into_iter().map(|col| self.get_height(col)).collect()
    }

    fn set(&mut self, row: usize, col: usize, item: usize) {
        if item < 1 { // false
            self.remove(row, col)
        } else {
            self.add(row, col)
        }
    }

    pub fn get(&self, row: usize, col: usize) -> bool {
        self._get(row, col) > 0
    }

    fn _get(&self, row: usize, col: usize) -> usize {
        (self.get_col(col) >> row) % 2
    }

    pub fn get_row(&self, row: usize) -> Row {
        (0..BOARD_WIDTH).into_iter().map(|col| self._get(row, col) << col).sum()
    }

    pub fn get_col(&self, col: usize) -> Column {
        self.arr[col]
    }

    fn get_height(&self, col: usize) -> usize {
        (usize::BITS - self.get_col(col).leading_zeros()) as usize
    }

    pub fn get_max_height(&self) -> usize {
        (0..BOARD_WIDTH).into_iter().map(|col| self.get_height(col)).max().unwrap()
    }

    pub fn get_min_height(&self) -> usize {
         (0..BOARD_WIDTH).into_iter().map(|col| self.get_height(col)).min().unwrap()
    }

    // setters
    pub fn set_arr(&mut self, arr: BoardArray) {
        self.arr = arr;
    }

    pub fn add(&mut self, row: usize, col: usize) {
        self.arr[col] |= 1 << row;
    }

    pub fn add_list(&mut self, locations: Vec<Point>) {
        for Point(row, col) in locations {
            self.add(row as usize, col as usize)
        }
    }

    pub fn add_garbage(&mut self, col: usize, amount: usize) {
        let highest = BOARD_HEIGHT;

        for r in 0..highest-amount+1 {
            self._set_row(highest-r, self.get_row(highest-r-amount));
        }

        for n in 0..amount {
            for w in 0..BOARD_WIDTH {
                if w == col {
                    self.arr[w] &= !(1 << n);
                } else {
                    self.arr[w] |= 1 << n;
                }
            }
        }
    }

    pub fn remove(&mut self, row: usize, col: usize) {
        self.arr[col] &= !(1 << row);
    }

    pub fn remove_list(&mut self, locations: Vec<Point>) {
        for Point(row, col) in locations {
            self.remove(row as usize, col as usize)
        }
    }

    pub fn set_row(&mut self, row: usize, new_row: Vec<bool>) {
        for (col, &item) in new_row.iter().enumerate() {
            self.set(row, col, item as usize);
        }
    }

    fn _set_row(&mut self, row: usize, new_row: Row) {
        let mut col = 0;
        let mut new_row = new_row + (1 << BOARD_WIDTH);
        while col < BOARD_WIDTH && new_row > 0 {
            self.set(row, col, new_row % 2);
            new_row = new_row >> 1;
            col += 1;
        }
    }

    pub fn remove_row(&mut self, row: usize) {
        for col in 0..BOARD_WIDTH {
            self.remove(row, col);
        }
    }

    // piece interactions
    pub fn set_piece(&mut self, piece: &Piece) -> bool {
        let mut out = false;
        if let Some(locations) = piece.abs_locations() {
            out = !self.piece_collision(piece);
            self.add_list(locations);
        }
        out
    }

    pub fn remove_piece(&mut self, piece: &Piece) {
        if let Some(locations) = piece.abs_locations() {
            self.remove_list(locations);
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
        }
        true
    }

    pub fn piece_valid_location(&self, piece: &Piece) -> bool {
        self.piece_in_bounds(piece) && !self.piece_collision(piece)
    }

    pub fn piece_valid_placement(&self, piece: &Piece) -> bool {
        self.piece_valid_location(piece) && self.piece_grounded(piece)
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
        self.set_piece(piece);
        if self.piece_collision(next) {
            return true;
        }
        self.remove_piece(piece);
        !piece
            .abs_locations()
            .unwrap()
            .iter()
            .any(|&x| x.0 < MAX_PLACE_HEIGHT as i8)
    }

    pub fn all_clear(&self) -> bool {
        // note: may fail edge cases
        self.get_row(0) < 1
    }

    pub fn clear_lines(&mut self) -> usize {
        let full_rows = self.all_full_rows();
        let highest = self.get_max_height();
        let num_cleared = full_rows.len();

        for &row in &full_rows {
            self.remove_row(row);
        }

        for &row in full_rows.iter().rev() {
            for r in row..highest {
                self._set_row(r, self.get_row(r + 1));
            }
        }

        num_cleared
    }

    fn full_row(&self, row: usize) -> bool {
        // 1023 = 2^10 - 1
        self.get_row(row) == 1023
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

            for row in (0..self.get_height(col)).rev() {
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

    pub fn horizontal_holes_weighted(&self, weight: &Weights) -> f32 {
        let mut holes_count_total = 0;
        let mut holes_count_weighted = 0;
        let mut finalweight = 0.0;

        for row in 0..self.get_max_height() {
            // only counting when you find a filled block
            let mut counting = true;

            for col in 0..BOARD_WIDTH {
                let spot = self.get(row, col);
                // hole
                if !spot {
                    holes_count_total += 1;
                    if counting {
                        holes_count_weighted += 1;
                        counting = false;
                    }
                } else {
                    counting = true;
                }
            }
            finalweight += (holes_count_total as f32)*0.0 + weight.holes_per_row_weighted_weight.eval(holes_count_weighted as f32);
            holes_count_total = 0;
            holes_count_weighted = 0;
        }

        finalweight
    }

    pub fn count_horizontal_holes(&self) -> f32 {
        let mut holes_count_total = 0;
        let mut holes_count_weighted = 0;
        let mut finalcount = 0.0;

        for row in 0..self.get_max_height() {
            // only counting when you find a filled block
            let mut counting = true;

            for col in 0..BOARD_WIDTH {
                let spot = self.get(row, col);
                // hole
                if !spot {
                    holes_count_total += 1;
                    if counting {
                        holes_count_weighted += 1;
                        counting = false;
                    }
                } else {
                    counting = true;
                }
            }
            finalcount += holes_count_weighted as f32;
            println!("row {} has {} weighted holes, {} holes", row, holes_count_weighted, holes_count_total);
            holes_count_total = 0;
            holes_count_weighted = 0;
        }

        finalcount
    }

    fn check_hor_t(arr: Vec<usize>) -> bool {
        // only for horizontal t slots rn

        // 1 0 0
        // 0 0 0
        // 1 0 1

        // 0 0 1
        // 0 0 0
        // 1 0 1

        // 1 0 1
        // 0 0 0
        // 1 0 1

        // 1 0 1
        // 1 0 0
        // 1 0 1

        // 1 0 1
        // 0 0 1
        // 1 0 1

        arr == [0b101, 0b000, 0b001] || arr == [0b001, 0b000, 0b101] || arr == [0b101, 0b000, 0b101] ||
        arr == [0b111, 0b000, 0b101] || arr == [0b101, 0b000, 0b111]
    }
    fn check_special_t(arr: Vec<usize>) -> bool {
        // special tspins

        // 1 1 0 0 1
        // 1 0 0 0 1
        // 1 0 1 1 1
        // 1 0 0 1 1
        // 1 0 1 1 1

        arr == [0b11111, 0b10000, 0b00101, 0b00111, 0b11111] || arr == [0b11111, 0b00111, 0b00101, 0b10000, 0b11111] ||

        // 1 1 0 0 0
        // 1 0 0 0 0
        // 1 0 1 1 1
        // 1 0 0 1 1
        // 1 0 1 1 1

        arr == [0b11111, 0b10000, 0b00101, 0b00111, 0b00111] || arr == [0b00111, 0b00111, 0b00101, 0b10000, 0b11111] ||

        // 1 1 0 0 1
        // 1 0 0 0 1
        // 1 0 1 1 1
        // 1 0 0 1 1
        // 1 0 0 1 1

        arr == [0b11111, 0b10000, 0b00100, 0b00111, 0b11111] || arr == [0b11111, 0b00111, 0b00100, 0b10000, 0b11111] ||

        // 1 1 0 0 0
        // 1 0 0 0 0
        // 1 0 1 1 1
        // 1 0 0 1 1
        // 1 0 0 1 1

        arr == [0b11111, 0b10000, 0b00100, 0b00111, 0b00111] || arr == [0b00111, 0b00111, 0b00100, 0b10000, 0b11111]
    }
    pub fn t_slot(&self) -> usize {
        let h = self.get_max_height();
        let l = self.get_min_height();

        if h - l < 3 {
            return 0
        }

        let mut out = 0;
        for row in l..=(h-3) {
            let mask = 0b111;
            for columns in self.arr.windows(3) {
                // create a 3x3 grid
                let columns: Vec<usize> = columns.iter().map(|x| x >> row & mask).collect();

                // checks if it is a t slot
                out += Board::check_hor_t(columns) as usize;
            }
        }

        if h - l < 5 {
            return out
        }

        for row in l..=(h-5) {
            let mask = 0b11111;
            for columns in self.arr.windows(5) {
                // create a 5x5 grid
                let columns: Vec<usize> = columns.iter().map(|x| x >> row & mask).collect();

                // checks if it is a t slot
                out += Board::check_special_t(columns) as usize;
            }
        }

        out
    }
    pub fn special_t_slot(&self) -> usize {
        let h = self.get_max_height();
        let l = self.get_min_height();

        if h - l < 5 {
            return 0
        }

        let mut out = 0;
        for row in l..=(h-5) {
            let mask = 0b11111;
            for columns in self.arr.windows(5) {
                // create a 5x5 grid
                let columns: Vec<usize> = columns.iter().map(|x| x >> row & mask).collect();

                // checks if it is a t slot
                out += Board::check_special_t(columns) as usize;
            }
        }

        out
    }

    pub fn get_max_height_difference(&self) -> usize {
        self.get_max_height() - self.get_min_height()
    }

    pub fn get_adjacent_height_differences(&self) -> Vec<usize> {
        self.get_heights()
            .windows(2)
            .map(|w| w[0].abs_diff(w[1]))
            .collect()
    }

    pub fn get_parities(&self) -> (bool, bool) {
        let mut col_parity: i32 = 0;
        let mut checkerboard_parity: i32 = 0;
        for (i, col) in self.arr.iter().enumerate() {
            let ones = col.count_ones() as usize;
            let blacks = (col & ZERO_ONE).count_ones() as i32;
            let whites = (col & ONE_ZERO).count_ones() as i32;
            col_parity = col_parity + (((i%2) * ones) as i32)
                - ((((i+1)%2) * ones) as i32);
            checkerboard_parity = checkerboard_parity + ((i%2) as i32) * (blacks - whites)
                - (((i+1)%2) as i32) * (blacks - whites);
        }
        return (checkerboard_parity == 0, col_parity == 0)
    }

    pub fn get_mino_count(&self) -> usize {
        let mut out: usize = 0;
        for col in self.get_arr(){
            out += col.count_ones() as usize;
        }
        return out;
    }

    pub fn should_panic(&self) -> bool {
        if self.get_max_height() + self.garbage_in_queue > 10 {
            return true;
        }
        false
    }

    pub fn garbage_in_queue_amnt(&self) -> usize {
        self.garbage_in_queue
    }

    pub fn get_garbage_in_queue(&self) -> usize {
        let mut garbage = 0;
        if ALLOWLOCALGAMEPLAY {
            garbage = fs::read_to_string(LOCALGAMEPLAYFILEPATH).expect("e").chars().nth(BOTNUM).expect("e").to_string().parse::<usize>().unwrap();
        }
        garbage
    }

    pub fn update_board_garbage_amount(&mut self) {
        self.garbage_in_queue = self.get_garbage_in_queue();
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
                    out.push_str(&format!("{} ", piece_type_to_string_colored(active_piece.get_type())));
                } else {
                    out.push_str(&format!("{} ", ("▫".truecolor(clamp(8*row,0,255).try_into().unwrap(),clamp(100-4*row,0,255).try_into().unwrap(),clamp(40,0,255).try_into().unwrap()))));
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
    fn test() {
        let mut board = Board::new();
        board.add(1, 1);
        println!("{}", board);
        println!("{}", board.get(1, 1));
        println!("{}", board.get(2, 1));

        board.set_row(4, vec!(false, false, false, true, true, true, false, true, true, true));
        println!("{}", board);
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

        board.add_list(vec![Point(5, 2), Point(3, 2), Point(5, 3)]);
        assert_eq!(board.get_heights(), [0, 0, 6, 6, 0, 0, 0, 0, 0, 0]);
        board.remove_list(vec![Point(5, 2), Point(5, 3)]);
        assert_eq!(board.get_heights(), [0, 0, 4, 0, 0, 0, 0, 0, 0, 0]);

        // assert!(false);
    }

    #[test]
    fn test_heights_2() {
        let mut board = Board::new();

        board.set_row(8, vec!(true; BOARD_WIDTH));
        board.add_list(vec![Point(5, 2), Point(3, 2), Point(5, 3)]);
        assert_eq!(board.get_heights(), [9; BOARD_WIDTH]);
        board.remove_row(8);
        assert_eq!(board.get_heights(), [0, 0, 6, 6, 0, 0, 0, 0, 0, 0]);

        board.set_row(8, vec!(true; BOARD_WIDTH));
        board.add(9, 3);
        println!("{}", board);
        board.clear_lines();
        println!("{}", board);
        assert_eq!(board.get_heights(), [0, 0, 6, 9, 0, 0, 0, 0, 0, 0]);

        board.set_row(6, vec!(true; BOARD_WIDTH));
        board.set_row(7, vec!(true; BOARD_WIDTH));
        assert_eq!(board.get_heights(), [8, 8, 8, 9, 8, 8, 8, 8, 8, 8]);
        board.clear_lines();
        assert_eq!(board.get_heights(), [0, 0, 6, 7, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_parity() {
        let mut board = Board::new();
        assert_eq!(board.get_parities(), (true, true));
        // T PIECE
        board.add(0,1);
        board.add(1,1);
        board.add(1,2);
        board.add(2,1);
        println!("{}", board);
        assert_eq!(board.get_parities(), (false, false));

        board.remove_row(0);
        board.remove_row(1);
        board.remove_row(2);
        // L PIECE
        board.add(0,1);
        board.add(0,2);
        board.add(1,1);
        board.add(2,1);
        println!("{}", board);
        assert_eq!(board.get_parities(), (true, false));
        board.remove_row(0);
        board.remove_row(1);
        board.remove_row(2);

        // ONE OF THE CASES

        board.set_row(0, vec!(true, true, true, true, true, true, true, true, false, false));
        board.set_row(1, vec!(true, true, true, true, true, true, true, true, false, false));
        println!("{}", board);
        assert_eq!(board.get_parities(), (true, true));
        board.remove_row(0);
        board.remove_row(1);
        board.remove_row(2);

        // ONE OTHER OF THE CASES

        board.set_row(0, vec!(true, false, false, false, false, true, true, true, true, true));
        println!("{}", board);
        assert_eq!(board.get_parities(), (true, true));
        board.remove_row(0);
        board.remove_row(1);
        board.remove_row(2);
    }
}
