#![allow(dead_code)]

use crate::constants::board_constants::*;
use crate::piece::Piece;
use arrayvec::ArrayVec;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Default, Copy, Eq, PartialEq, Hash)]
pub struct Board {
    pub arr: [u32; BOARD_WIDTH],
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
            writeln!(f)?
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
    #[inline]
    pub fn get_heights(&self) -> [usize; BOARD_WIDTH] {
        let mut heights = [0; BOARD_WIDTH];
        for (col, height) in heights.iter_mut().enumerate() {
            *height = self.get_height(col);
        }
        heights
    }

    /// returns the index of the first empty row in column col
    #[inline]
    pub fn get_height(&self, col: usize) -> usize {
        Self::height(self.arr[col])
    }

    /// returns the index of the first empty row in a col below the given row
    #[inline]
    pub fn get_height_below(&self, col: usize, row: usize) -> usize {
        Self::height(self.arr[col] & !(u32::MAX << row))
    }

    /// returns the state (0 or 1) at the grid's row and col
    #[inline]
    pub fn get(&self, row: usize, col: usize) -> bool {
        (self.arr[col] >> row & 1) == 1
    }

    // setters ----------------------------------
    /// sets the cell at the row and col to the specified state
    #[inline]
    pub fn set(&mut self, row: usize, col: usize, state: usize) {
        if state == 0 {
            self.remove(row, col)
        } else {
            self.add(row, col)
        }
    }

    #[inline]
    pub fn set_row(&mut self, row: usize, data: [bool; BOARD_WIDTH]) {
        for (col, state) in data.into_iter().enumerate() {
            self.set(row, col, state as usize);
        }
    }

    #[inline]
    pub fn remove_row(&mut self, row: usize) {
        self.set_row(row, [false; BOARD_WIDTH]);
    }

    // piece API --------------------------------
    /// sets the four minos of the piece
    #[inline]
    pub fn set_piece(&mut self, piece: &Piece) {
        if let Some(locs) = piece.abs_locations() {
            for [row, col] in locs {
                self.add(row, col);
            }
        }
    }

    /// removes the minos that occupy the piece's location
    #[inline]
    pub fn remove_piece(&mut self, piece: &Piece) {
        if let Some(locs) = piece.abs_locations() {
            for [row, col] in locs {
                self.remove(row, col);
            }
        }
    }

    /// returns whether the piece minos can be shifted downwards
    #[inline]
    pub fn piece_grounded(&self, piece: &Piece) -> bool {
        if let Some(locs) = piece.abs_locations() {
            locs.iter()
                .any(|&[row, col]| row == 0 || self.get(row - 1, col))
        } else {
            false
        }
    }

    /// returns whether the piece has a collision inside the grid
    #[inline]
    pub fn piece_collision(&self, piece: &Piece) -> bool {
        if let Some(locs) = piece.abs_locations() {
            locs.iter().any(|&[row, col]| self.get(row, col))
        } else {
            false
        }
    }

    /// returns whether the piece has no collision and is grounded
    #[inline]
    pub fn piece_can_set(&self, piece: &Piece) -> bool {
        !self.piece_collision(piece) && self.piece_grounded(piece)
    }

    /// returns the max amount of rows that a piece can move down
    /// (as a negative number), output is from (-height, 0]
    #[inline]
    pub fn piece_max_down(&self, piece: &Piece) -> i8 {
        if let Some(locs) = piece.abs_locations() {
            locs.into_iter()
                .map(|[row, col]| (row - self.get_height_below(col, row)) as i8)
                .min()
                .unwrap()
        } else {
            0
        }
    }

    // statistics -------------------------------
    /// the row of the highest placed mino
    #[inline]
    pub fn get_max_height(arr: &[u32]) -> usize {
        Self::height(*arr.iter().max().unwrap())
    }

    /// the row of the lowest placed mino
    #[inline]
    pub fn get_min_height(arr: &[u32]) -> usize {
        Self::height(*arr.iter().min().unwrap())
    }

    /// The total amount of holes
    #[inline]
    pub fn get_total_holes(arr: &[u32]) -> u32 {
        arr.iter()
            .map(|col| col.count_zeros() - col.leading_zeros())
            .sum()
    }

    /// The total number of unique holes. Connected holes within the same column are counted as one.
    #[inline]
    pub fn get_messiness(arr: &[u32]) -> usize {
        arr.iter()
            .map(|&e| {
                let mut e = e >> e.trailing_ones(); // copy as mut
                let mut count = 0;
                while e != 0 {
                    count += 1;
                    e >>= e.trailing_zeros();
                    e >>= e.trailing_ones();
                }
                count
            })
            .sum()
    }

    /// returns the amount of t-slots (with an accessible overhang)
    /// present in the current board
    // TODO: SPEED THIS UP WITH SIMD
    #[inline]
    #[allow(clippy::unnecessary_fold)] // using any is 40% slower
    pub fn t_slot(arr: &[u32]) -> usize {
        const SIZE: usize = 3;
        (Self::get_min_height(arr)..=(Self::get_max_height(arr) - SIZE))
            .map(|row| {
                arr.windows(SIZE)
                    .fold(false, |t_slot, cols| t_slot || Self::check_hor_t(cols, row))
            })
            .filter(|b| *b)
            .count()
    }

    /// returns the sum of the adjacent differences between column heights
    // TODO: Maybe mirror columns 1 and 8 to cols -1 and 10 to weight edge cols evenly
    #[inline]
    pub fn get_adjacent_height_differences(arr: &[u32]) -> usize {
        arr.windows(2)
            .map(|w| Self::height(w[0]).abs_diff(Self::height(w[1])))
            .sum()
    }

    /// returns the sum of the adjacent differences between column heights IGNORING WELLS
    // TODO: Maybe mirror columns 1 and 8 to cols -1 and 10 to weight edge cols evenly
    #[inline]
    pub fn get_stack_height_differences(arr: &[u32]) -> usize {
        let min = Self::get_min_height(arr);
        arr.iter()
        .filter(|&&x| Self::height(x) != min)
        .for_each(|x| print!("{} ", x));
        println!();

        arr.iter()
            .filter(|&&x| Self::height(x) != min)
            .collect::<ArrayVec<_, { BOARD_WIDTH - 1}>>()
            .windows(2)
            .map(|w| Self::height(*w[0]).abs_diff(Self::height(*w[1])))
            .sum()
    }

    /// returns the checkerboard parity of the board (differences between checkerboard)
    /// see https://docs.google.com/document/d/1udtq235q2SdoFYwMZNu-GRYR-4dCYMkp0E8_Hw1XTyg/edit
    /// for an explanation of parity and PC theory
    #[inline]
    pub fn checkerboard_parity(arr: &[u32]) -> i8 {
        const MASK: u32 = 0xAAAAAAAA; // 1010..
        arr.chunks(2)
            .map(|cols| {
                (cols[0] & MASK).count_ones() as i8 - (cols[0] & (MASK >> 1)).count_ones() as i8
                    + (cols[1] & (MASK >> 1)).count_ones() as i8
                    - (cols[1] & MASK).count_ones() as i8
            })
            .sum()
    }

    /// returns the columnar parity of the board (differences between columns)
    /// see https://docs.google.com/document/d/1udtq235q2SdoFYwMZNu-GRYR-4dCYMkp0E8_Hw1XTyg/edit
    /// for an explanation of parity and PC theory
    #[inline]
    pub fn columnar_parity(arr: &[u32]) -> i8 {
        arr.chunks(2)
            .map(|cols| ((cols[0]).count_ones() as i8) - (cols[1].count_ones() as i8))
            .sum()
    }

    /// returns the amount of non-empty cells on the board
    #[inline]
    pub fn cell_count(arr: &[u32]) -> u32 {
        arr.iter().map(|c| c.count_ones()).sum()
    }

    /// partitions the board into sections that are split by columns of height row
    /// the full columns are not included in the partition
    /// includes the range of the included columns by
    // TODO: maybe figure out how to not return a vec -- save some allocations
    // ArrayVec might be nice -- capacity-limited vec allocated on stack
    #[inline]
    pub fn partition(&self, row: usize) -> Vec<&[u32]> {
        let mut out = Vec::new();
        let mut prev = 0;
        let iter = self
            .arr
            .iter()
            .enumerate()
            .filter(|(_, c)| Self::height(**c) == row + 1);
        for (split, _) in iter {
            if split > prev {
                out.push(&self.arr[prev..split]);
            }
            prev = split + 1;
        }
        if prev != BOARD_WIDTH {
            out.push(&self.arr[prev..BOARD_WIDTH]);
        }
        out
    }

    // versus -----------------------------------
    /// returns whether the board is empty
    #[inline]
    pub fn all_clear(&self) -> bool {
        self.arr.iter().sum::<u32>() == 0
    }

    /// clears all filled lines on the board and moves down
    /// the blocks above those lines
    #[inline]
    pub fn clear_lines(&mut self) -> u32 {
        let full_rows = self.arr.into_iter().reduce(|x, y| x & y).unwrap();
        let mut rows = full_rows; // copy
        while rows != 0 {
            let r = rows.trailing_zeros();
            let mask = (1 << r) - 1;
            for col in self.arr.iter_mut() {
                *col = *col & mask | *col >> 1 & !mask
            }
            rows &= !(1 << r);
            rows >>= 1;
        }
        full_rows
    }

    /// method for undoing a clear_lines
    /// rows is the output of clear_lines
    #[inline]
    pub fn insert_rows(&mut self, rows: usize) {
        let mut rows = rows;
        while rows != 0 {
            let row = rows.trailing_zeros();
            self.insert_full_line(row as usize);
            rows &= !(1 << row);
        }
    }

    /// inserts a line on the row and moves everything above up
    #[inline]
    pub fn insert_full_line(&mut self, row: usize) {
        // move all the rows up by 1
        // clear the first row lines
        // add back the original row lines
        // set row to full
        for i in 0..BOARD_WIDTH {
            let mut col = self.arr[i];
            col &= !((1 << row) - 1);
            col <<= 1;
            col |= col & ((1 << row) - 1);
            col |= 1 << row;
            self.arr[i] = col;
        }
    }

    // private methods --------------------------
    /// sets the state at the row and col to 0
    #[inline]
    fn remove(&mut self, row: usize, col: usize) {
        self.arr[col] &= !(1 << row);
    }

    /// sets the state at the row and col to 1
    #[inline]
    fn add(&mut self, row: usize, col: usize) {
        self.arr[col] |= 1 << row;
    }

    /// the height of a col
    #[inline]
    fn height(col: u32) -> usize {
        (u32::BITS - col.leading_zeros()) as usize
    }

    /// whether a 3x3 grid is a horizontal t-slot
    /// with the bottom at the given row. for example:
    /// 1 0 0  |  0 0 1
    /// 0 0 0  |  0 0 0
    /// 1 0 1  |  1 0 1
    #[inline]
    fn check_hor_t(arr: &[u32], row: usize) -> bool {
        const MASK: u32 = 0b111;
        let c1 = arr[0] >> row & MASK;
        let c2 = arr[1] >> row & MASK;
        let c3 = arr[2] >> row & MASK;
        [c1, c2, c3] == [0b101, 0b000, 0b001] || [c1, c2, c3] == [0b001, 0b000, 0b101]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_api::functions::*;

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

        board.set_row(7, [true; BOARD_WIDTH]);
        println!("{}", board);
        assert_eq!(board.get_heights(), [8, 8, 8, 9, 8, 8, 8, 8, 8, 8]);
        println!("{}", board);
        board.clear_lines();
        println!("{}", board);
        assert_eq!(board.get_heights(), [0, 0, 6, 8, 0, 0, 0, 0, 0, 0]);
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
    }

    #[test]
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

    #[test]
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

    #[test]
    fn test_total_holes() {
        let board = Board::new();
        assert_eq!(Board::get_total_holes(&board.arr), 0);

        #[rustfmt::skip]
        let boardstr = [
            "..........",
            ".x........",
            "x.x.......",
        ];
        let board = board_from_string(&boardstr);
        assert_eq!(Board::get_total_holes(&board.arr), 1);

        #[rustfmt::skip]
        let boardstr = [
            "......x...",
            ".xxxxx.xx.",
            "x.x.......",
        ];
        let board = board_from_string(&boardstr);
        assert_eq!(Board::get_total_holes(&board.arr), 8);
    }

    #[test]
    fn test_messiness() {
        let board = Board::new();
        assert_eq!(Board::get_total_holes(&board.arr), 0);

        #[rustfmt::skip]
        let boardstr = [
            "..........",
            ".x........",
            "x.x.......",
        ];
        let board = board_from_string(&boardstr);
        assert_eq!(Board::get_messiness(&board.arr), 1);

        #[rustfmt::skip]
        let boardstr = [
            "..........",
            ".x........",
            "x.x.......",
            "x.x.......",
            "x.x.......",
            "x.x.......",
            "x.x.......",
        ];
        let board = board_from_string(&boardstr);
        assert_eq!(Board::get_messiness(&board.arr), 1);

        #[rustfmt::skip]
        let boardstr = [
            "..........",
            ".x........",
            "x.x.......",
            "x.x.......",
            "xxx.......",
            "x.x.......",
            "x.x.......",
        ];
        let board = board_from_string(&boardstr);
        assert_eq!(Board::get_messiness(&board.arr), 2);

        #[rustfmt::skip]
        let boardstr = [
            "..........",
            ".x........",
            "x.x.......",
            "x..x......",
            "xxx.x.....",
            "x.x.x.....",
            "x.x.x.....",
        ];
        let board = board_from_string(&boardstr);
        assert_eq!(Board::get_messiness(&board.arr), 4);

        #[rustfmt::skip]
        let boardstr = [
            "......x...",
            ".xxxxx.xx.",
            "x.x.......",
        ];
        let board = board_from_string(&boardstr);
        assert_eq!(Board::get_messiness(&board.arr), 7);
    }

    #[test]
    fn test_adjacent_heigts() {
        let board = Board::new();
        assert_eq!(Board::get_adjacent_height_differences(&board.arr), 0);

        #[rustfmt::skip]
        let boardstr = [
            "x.........",
            "x.........",
            "x.........",
        ];
        let board = board_from_string(&boardstr);
        assert_eq!(Board::get_adjacent_height_differences(&board.arr), 3);

        #[rustfmt::skip]
        let boardstr = [
            "..x.......",
            "..x.......",
            "..xx......",
        ];
        let board = board_from_string(&boardstr);
        assert_eq!(Board::get_adjacent_height_differences(&board.arr), 6);

        #[rustfmt::skip]
        let boardstr = [
            "..x..x....",
            "..x.x.....",
            "..xxx.....",
        ];
        let board = board_from_string(&boardstr);
        assert_eq!(Board::get_adjacent_height_differences(&board.arr), 10);

        let board = versus_board_tall();
        println!("{}", board);
        assert_eq!(Board::get_adjacent_height_differences(&board.arr), 16);
    }

    #[test]
    fn test_stack_heigts() {
        let board = Board::new();
        assert_eq!(Board::get_stack_height_differences(&board.arr), 0);

        #[rustfmt::skip]
        let boardstr = [
            "..........",
            "xxxxxx.xxx",
            "xxxxxx.xxx",
        ];
        let board = board_from_string(&boardstr);
        assert_eq!(Board::get_stack_height_differences(&board.arr), 0);

        #[rustfmt::skip]
        let boardstr = [
            ".......x..",
            "xxxxxx.xxx",
            "xxxxxx.xxx",
        ];
        let board = board_from_string(&boardstr);
        assert_eq!(Board::get_stack_height_differences(&board.arr), 2);

        let board = versus_board_tall();
        println!("{}", board);
        assert_eq!(Board::get_stack_height_differences(&board.arr), 6);
    }

    #[test]
    fn test_parity() {
        let mut board = Board::new();
        board.arr[0] = 0b1010;
        board.arr[1] = 0b0101;
        assert_eq!(Board::checkerboard_parity(&board.arr), 4);
        assert_eq!(Board::columnar_parity(&board.arr), 0);

        board.arr[1] = 0b1010;
        assert_eq!(Board::checkerboard_parity(&board.arr).abs(), 0);

        board.arr[2] = 0b1110;
        assert_eq!(Board::checkerboard_parity(&board.arr).abs(), 1);
        assert_eq!(Board::columnar_parity(&board.arr).abs(), 3);

        // adding a horizontal t-piece
        board.arr[2] = 0b100;
        board.arr[3] = 0b110;
        board.arr[4] = 0b100;
        assert_eq!(Board::checkerboard_parity(&board.arr).abs(), 2);
        assert_eq!(Board::columnar_parity(&board.arr), 0);
    }

    #[test]
    fn test_t_slot() {
        #[rustfmt::skip]
        let boardstr = [
            "..x.......",
            "..........",
            "x.x.......",
        ];
        let mut board = board_from_string(&boardstr);
        assert_eq!(Board::t_slot(&board.arr), 1);

        board.arr[0] <<= 10;
        board.arr[1] <<= 10;
        board.arr[2] <<= 10;
        assert_eq!(Board::t_slot(&board.arr), 1);

        board.arr[6] = 0b001;
        board.arr[5] = 0b000;
        board.arr[4] = 0b101;
        assert_eq!(Board::t_slot(&board.arr), 2);

        board.arr[2] = 0;
        assert_eq!(Board::t_slot(&board.arr), 1);

        // Multiple tslot on a row
        #[rustfmt::skip]
        let boardstr = [
            "xx..xx..xx",
            "x...x...xx",
            "xx.xxx.xxx",
        ];
        let board = board_from_string(&boardstr);
        assert_eq!(Board::t_slot(&board.arr), 1);

        // Offset tslots
        #[rustfmt::skip]
        let boardstr = [
            "xx.....x..",
            "xx..x...xx",
            "x...xx.xxx",
            "xx.xxxxxxx",
        ];
        let board = board_from_string(&boardstr);
        assert_eq!(Board::t_slot(&board.arr), 2);

        // Not a tslot
        #[rustfmt::skip]
        let boardstr = [
            "xxxx...xxx",
            "xxx.....xx",
            "xxxx.x.xxx",
        ];
        let board = board_from_string(&boardstr);
        // TODO: Figure out if this should be 0?
        assert_eq!(Board::t_slot(&board.arr), 1);
    }

    #[test]
    fn test_partition() {
        let board = Board {
            arr: [1, 42, 3, 31, 4, 8, 2, 3, 18, 7],
        };
        let boards = board.partition(4);
        assert_eq!(boards.len(), 3);
        assert_eq!(boards[0].len(), 3);
        assert_eq!(boards[1].len(), 4);
        assert_eq!(boards[2].len(), 1);
    }
}
