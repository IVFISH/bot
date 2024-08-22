#![allow(dead_code)]

use crate::constants::board_constants::*;
use crate::constants::piece_constants::*;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Piece {
    pub r#type: u8,
    pub dir: u8,
    pub row: usize,
    pub col: usize,
    pub spin: SpinType,
}

impl Eq for Piece {}

impl PartialEq for Piece {
    fn eq(&self, other: &Self) -> bool {
        if self.r#type == other.r#type && self.r#type == PIECE_O {
            self.sorted_abs_locations() == other.sorted_abs_locations()
        } else {
            self.r#type == other.r#type
                && self.dir == other.dir
                && self.col == other.col
                && self.row == other.row
        }
    }
}

impl Hash for Piece {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self.r#type {
            PIECE_O => self.sorted_abs_locations().hash(state),
            _ => {
                self.r#type.hash(state);
                self.dir.hash(state);
                self.row.hash(state);
                self.col.hash(state);
            }
        }
    }
}

impl Default for Piece {
    fn default() -> Self {
        Self {
            r#type: 255, // Defualt to bad piece
            dir: 0,
            row: SPAWN_ROW,
            col: SPAWN_COL,
            spin: SpinType::None,
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

    /// returns a sorted version of the ['Piece.abs_locations']
    pub fn sorted_abs_locations(&self) -> Option<[[usize; 2]; PIECE_SIZE]> {
        self.abs_locations().map(|mut locs| {
            locs.sort();
            locs
        })
    }

    /// returns the kicks for a piece
    /// when the piece is rotated direction dir
    /// this must be used with the initial rotation state
    /// (call this before doing rotate)
    pub const fn get_kicks(&self, dir: u8) -> &[[i8; 2]] {
        let d = self.dir as usize;
        let dir = dir as usize;

        // Order cases in most likely order to minimize branch mispredictions
        if dir != 0 {
            // match isn't faster than if, but i like it more
            match self.r#type {
                PIECE_O => &O_OFFSETS[d][dir - 1],
                PIECE_I => {
                    if dir != 2 {
                        &FIVE_OFFSETS[d][dir / 2]
                    } else {
                        &FIVE_180_OFFSETS[d]
                    }
                }
                _ => {
                    if dir != 2 {
                        &THREE_OFFSETS[d][dir / 2]
                    } else {
                        &THREE_180_OFFSETS[d]
                    }
                }
            }
        } else {
            // No rotation, don't kick
            &[[0, 0]]
        }
    }

    /// returns the lowest row this piece is on
    pub fn bottom_row(&self) -> Option<usize> {
        self.abs_locations().map(|pos| {
            pos.into_iter()
                .map(|[r, _]| r)
                .min_by(|r1, r2| r1.cmp(r2))
                .unwrap()
        })
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
    pub fn r#move(&mut self, dir_row: i8, dir_col: i8) -> &mut Self {
        if Self::can_move(self, dir_row, dir_col) {
            self.move_unchecked(dir_row, dir_col);
        }
        self
    }

    /// rotates a piece clockwise by the direction
    /// does not do any kicks (see Game's [] method)
    pub fn rotate(&mut self, dir: u8) -> &mut Self {
        if Self::can_rotate(self, dir) {
            self.dir = (self.dir + dir) % 4;
        }
        self
    }

    /// rotates a piece clockwise by the direction
    /// moves a piece in the specified vector direction
    pub fn rotate_with_kicks(&mut self, dir: u8, dir_row: i8, dir_col: i8) -> &mut Self {
        if Self::can_rotate_kick(self, dir, dir_row, dir_col) {
            self.rotate_with_kicks_unchecked(dir, dir_row, dir_col);
        };
        self
    }

    pub fn move_unchecked(&mut self, dir_row: i8, dir_col: i8) -> &mut Self {
        self.row = (self.row as i8 + dir_row) as usize;
        self.col = (self.col as i8 + dir_col) as usize;
        self.spin = SpinType::None;
        self
    }

    pub fn rotate_with_kicks_unchecked(&mut self, dir: u8, dir_row: i8, dir_col: i8) -> &mut Self {
        self.dir = (self.dir + dir) % 4;
        self.row = (self.row as i8 + dir_row) as usize;
        self.col = (self.col as i8 + dir_col) as usize;
        self
    }

    // static -----------------------------------
    /// whether a piece can be moved by a vector
    pub fn can_move(piece: &Self, dir_row: i8, dir_col: i8) -> bool {
        piece.abs_locations().map_or(false, |locations| {
            locations
                .into_iter()
                .all(|[row, col]| Self::in_bounds(row as i8 + dir_row, col as i8 + dir_col))
        })
    }

    /// whether a piece can be rotated in a direction
    pub fn can_rotate(piece: &Self, dir: u8) -> bool {
        let mut p = *piece; // copy
        p.dir = (p.dir + dir) % 4;
        p.abs_locations().is_some()
    }

    /// whether a piece is valid after being rotated and kicked
    /// does not check intermediate states
    pub fn can_rotate_kick(piece: &Self, dir: u8, dir_row: i8, dir_col: i8) -> bool {
        let mut p = *piece;
        p.dir = (p.dir + dir) % 4;
        Self::in_bounds(p.row as i8 + dir_row, p.col as i8 + dir_col)
            .then(|| {
                p.row = (p.row as i8 + dir_row) as usize;
                p.col = (p.col as i8 + dir_col) as usize;
            })
            .is_some()
            && p.abs_locations().is_some()
    }

    // private helpers --------------------------
    const fn u_in_bounds(row: usize, col: usize) -> bool {
        Self::u_row_in_bounds(row) && Self::u_col_in_bounds(col)
    }

    const fn u_row_in_bounds(row: usize) -> bool {
        row < BOARD_HEIGHT
    }

    const fn u_col_in_bounds(col: usize) -> bool {
        col < BOARD_WIDTH
    }

    const fn in_bounds(row: i8, col: i8) -> bool {
        Self::row_in_bounds(row) && Self::col_in_bounds(col)
    }

    const fn row_in_bounds(row: i8) -> bool {
        0 <= row && row < (BOARD_HEIGHT as i8)
    }

    const fn col_in_bounds(col: i8) -> bool {
        0 <= col && col < (BOARD_WIDTH as i8)
    }
}

#[cfg(test)]
mod tests {
    use crate::constants::piece_constants::*;
    use crate::piece::*;
    use crate::test_api::functions::*;
    use std::collections::HashSet;

    #[test]
    fn test_absolute_location_in_bounds() {
        let mut piece = Piece::new(PIECE_T);
        piece.row = 5;
        piece.col = 3;
        assert_location_eq(piece.abs_locations(), [[5, 2], [5, 3], [5, 4], [6, 3]])
    }

    #[test]
    fn test_spawn_location() {
        let piece = Piece::new(PIECE_I);
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
        for _ in 0..3 {
            assert!(Piece::can_move(&piece, 0, -1));
            piece.r#move(0, -1);
        }
        assert!(!Piece::can_move(&piece, 0, -1));
        assert_location_eq(piece.abs_locations(), [[21, 0], [21, 1], [22, 1], [22, 2]]);
        piece = Piece::new(PIECE_S);
        for _ in 0..4 {
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

    // #[test]
    fn o_hash() {
        let piece_1 = Piece::new(PIECE_O);
        let mut piece_2 = Piece::new(PIECE_O);
        let [dir_row, dir_col] = piece_2.get_kicks(1)[0];
        piece_2.rotate_with_kicks(1, dir_row, dir_col);

        assert_eq!(piece_1, piece_2);
        assert_eq!(calculate_hash(&piece_1), calculate_hash(&piece_2));

        let mut set = HashSet::new();
        set.insert(piece_1);
        assert!(set.contains(&piece_2));

        piece_2.r#move(-2, 0);
        assert_ne!(piece_1, piece_2);
        assert_ne!(calculate_hash(&piece_1), calculate_hash(&piece_2));
        assert!(!set.contains(&piece_2));
    }
}
