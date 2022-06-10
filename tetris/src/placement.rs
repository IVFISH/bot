use crate::errors::GameError;
use crate::board::Board;

use std::{collections::HashMap, fmt::Display};


pub type RotationState = u8;
pub type RotationLocation = [[Mino; 4]; 4];

const PIECE_LOCATIONS: [RotationLocation; 7] = [L_ROTATIONS; 7];

const L_ROTATIONS: RotationLocation = [
    [Mino(1, 1), Mino(0, -1), Mino(0, 0), Mino(0, 1)],
    [Mino(-1, 1), Mino(1, 0), Mino(0, 0), Mino(-1, 0)],
    [Mino(-1, -1), Mino(0, 1), Mino(0, 0), Mino(0, -1)],
    [Mino(1, -1), Mino(-1, 0), Mino(0, 0), Mino(1, 0)]
];


pub enum Piece {
    I,
    O,
    T,
    L,
    J,
    S,
    Z,
}

pub struct Placement {

    piece_type: Piece,
    rotation_state: RotationState,
    center: Point

}

impl Placement {

    pub fn to_list(&self) -> [Point; 4] {
        unimplemented!()
    }

    pub fn move_by_vector(&mut self) {

    }

}

pub struct Point {
    pub row: usize,
    pub col: usize
}

pub struct MoveVector(i8, i8);

struct Mino(i8, i8);

impl MoveVector {

    fn add_to_point(&self, other: Point) -> Result<Point, GameError> {
        let row = self.row + other.row as i8;
        let col = self.row + other.col as i8;

        if row < 0 || col < 0 {
            return Err(GameError::NotInBounds);
        }

        let row = row as usize;
        let col = col as usize;

        if !Board::in_bounds(row, col) {
            return Err(GameError::NotInBounds);
        }

        Ok(Point {row, col})
    }
}