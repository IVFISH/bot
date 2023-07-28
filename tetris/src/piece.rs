#![allow(dead_code)]

#[derive(Debug, Copy, Clone, PartialEq)]
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
            row: 20,
            col: 5
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
    pub fn abs_locations(&self) -> Option<[[usize; 2]; 4]> {
        unimplemented!()
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
        unimplemented!()
    }

    /// sets the col of the piece
    /// checks if the col is in bounds
    /// to avoid bounds checking, directly set the col member
    pub fn set_col(&mut self, col: usize) {
        unimplemented!()
    }

    // mutators ---------------------------------
    /// moves a piece in the specified vector direction
    /// if the new position would be in bounds
    pub fn r#move(&mut self, dir_row: usize, dir_col: usize) {
        unimplemented!()
    }
    
    /// rotates a piece clockwise by the direction
    /// does not do any kicks (see Game's [] method)
    pub fn rotate(&mut self, dir: u8) {
        unimplemented!()
    }

    // static -----------------------------------
    /// bounds checking on the row
    pub fn in_bounds_row(row: usize) -> bool {
        unimplemented!()
    }

    /// bounds checking on the col
    pub fn in_bounds_col(col: usize) -> bool {
        unimplemented!()
    }

    /// whether a piece can be moved by a vector
    pub fn can_move(piece: &Self, dir_row: usize, dir_col: usize) -> bool {
        unimplemented!()
    }
}
