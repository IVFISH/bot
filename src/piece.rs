#![allow(dead_code)]

use crate::constants::board_constants::*;
use crate::constants::piece_constants::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Piece {
    pub r#type: u8,
    pub dir: u8,
    pub row: usize,
    pub col: usize,
}

impl Default for Piece {
    fn default() -> Self {
        Self {
            r#type: 0,
            dir: 0,
            row: SPAWN_ROW,
            col: SPAWN_COL,
        }
    }
}

impl Piece {
    // constructors -----------------------------
    /// returns a new piece of the type in the default spawn location
    pub fn new(piece_type: u8) -> Self {
        Self {
            r#type: piece_type,
            ..Default::default()
        }
    }

    // getters ----------------------------------
    /// returns None if the positions are out of bounds
    /// returns an array of 4 row, col pairs
    pub fn abs_locations(&self) -> Option<[[usize; 2]; PIECE_SIZE]> {
        let mut out = [[0; 2]; 4];
        for (i, &[row, col]) in PIECE_ROTATIONS[self.r#type as usize][self.dir as usize]
            .iter()
            .enumerate()
        {
            let r = row + (self.row as i8);
            let c = col + (self.col as i8);
            if !Self::in_bounds(r, c) {
                return None;
            }
            out[i] = [r as usize, c as usize];
        }
        Some(out)
    }

    /// returns the kicks for a piece
    /// when the piece is rotated direction dir
    pub fn get_kicks(&self, dir: u8) -> Vec<[usize; 2]> {
        unimplemented!()
    }

    // setters ----------------------------------
    /// sets the row of the piece
    /// checks if the row is in bounds
    /// to avoid bounds checking, directly set the row member
    pub fn set_row(&mut self, row: usize) {
        if Self::u_row_in_bounds(row) {
            self.row = row;
        }
    }

    /// sets the col of the piece
    /// checks if the col is in bounds
    /// to avoid bounds checking, directly set the col member
    pub fn set_col(&mut self, col: usize) {
        if Self::u_col_in_bounds(col) {
            self.col = col;
        }
    }

    // mutators ---------------------------------
    /// moves a piece in the specified vector direction
    /// if the new position would be in bounds
    pub fn r#move(&mut self, dir_row: i8, dir_col: i8) {
        if Self::can_move(self, dir_row, dir_col) {
            self.row = (self.row as i8 + dir_row) as usize;
            self.col = (self.col as i8 + dir_col) as usize;
        }
    }

    /// rotates a piece clockwise by the direction
    /// does not do any kicks (see Game's [] method)
    pub fn rotate(&mut self, dir: u8) {
        if Self::can_rotate(self, dir) {
            self.dir = (self.dir + dir) % 4;
        }
    }

    // static -----------------------------------
    /// whether a piece can be moved by a vector
    pub fn can_move(piece: &Self, dir_row: i8, dir_col: i8) -> bool {
        let mut p = *piece; // copy
        p.row = ((piece.row as i8) + dir_row) as usize;
        p.col = ((piece.col as i8) + dir_col) as usize;
        p.abs_locations().is_some()
    }

    /// whether a piece can be rotated by a vector
    pub fn can_rotate(piece: &Self, dir: u8) -> bool {
        let mut p = *piece; // copy
        p.dir = (p.dir + dir) % 4;
        p.abs_locations().is_some()
    }

    // private helpers --------------------------
    fn u_in_bounds(row: usize, col: usize) -> bool {
        Self::u_row_in_bounds(row) && Self::u_col_in_bounds(col)
    }

    fn u_row_in_bounds(row: usize) -> bool {
        row < BOARD_HEIGHT
    }

    fn u_col_in_bounds(col: usize) -> bool {
        col < BOARD_WIDTH
    }

    fn in_bounds(row: i8, col: i8) -> bool {
        Self::row_in_bounds(row) && Self::col_in_bounds(col)
    }

    fn row_in_bounds(row: i8) -> bool {
        0 <= row && row < (BOARD_HEIGHT as i8)
    }

    fn col_in_bounds(col: i8) -> bool {
        0 <= col && col < (BOARD_WIDTH as i8)
    }
}

#[cfg(test)]
mod tests {
    use crate::constants::piece_constants::*;
    use crate::piece::*;

    fn assert_location_eq(mut locations: Option<[[usize; 2]; 4]>, sols: [[usize; 2]; 4]) {
        if let Some(mut locs) = locations {
            locs.sort();
            assert_eq!(locs, sols)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn test_absolute_location_in_bounds() {
        let mut piece = Piece::new(PIECE_T);
        piece.row = 5;
        piece.col = 3;
        assert_location_eq(piece.abs_locations(), [[5, 2], [5, 3], [5, 4], [6, 3]])
    }

    #[test]
    fn test_spawn_location() {
        let mut piece = Piece::new(PIECE_I);
        assert_eq!(piece.row, SPAWN_ROW);
        assert_eq!(piece.col, SPAWN_COL);
        assert_location_eq(piece.abs_locations(), [[21, 3], [21, 4], [21, 5], [21, 6]])
    }

    #[test]
    fn test_rotate_loop() {
        let mut piece = Piece::new(PIECE_L);
        piece.row = 1;
        piece.col = 2;
        assert_location_eq(piece.abs_locations(), [[1, 1], [1, 2], [1, 3], [2, 3]]);
        piece.rotate(1);
        assert_location_eq(piece.abs_locations(), [[0, 2], [0, 3], [1, 2], [2, 2]]);
        piece.rotate(1);
        assert_location_eq(piece.abs_locations(), [[0, 1], [1, 1], [1, 2], [1, 3]]);
        piece.rotate(1);
        assert_location_eq(piece.abs_locations(), [[0, 2], [1, 2], [2, 1], [2, 2]]);
        piece.rotate(1);
        assert_location_eq(piece.abs_locations(), [[1, 1], [1, 2], [1, 3], [2, 3]]);
    }

    #[test]
    fn move_out_of_bounds() {
        let mut piece = Piece::new(PIECE_S);
        for i in 0..3 {
            assert!(Piece::can_move(&piece, 0, -1));
            piece.r#move(0, -1);
        }
        assert!(!Piece::can_move(&piece, 0, -1));
        assert_location_eq(piece.abs_locations(), [[21, 0], [21, 1], [22, 1], [22, 2]]);
        piece = Piece::new(PIECE_S);
        for i in 0..4 {
            assert!(Piece::can_move(&piece, 0, 1));
            piece.r#move(0, 1);
        }
        assert!(!Piece::can_move(&piece, 0, 1));
        assert_location_eq(piece.abs_locations(), [[21, 7], [21, 8], [22, 8], [22, 9]]);
    }

    #[test]
    fn rotate_out_of_bounds() {
        let mut piece = Piece::new(PIECE_Z);
        assert!(Piece::can_rotate(&piece, 1));
        piece.row = 0;
        assert!(!Piece::can_rotate(&piece, 1));
    }
}
