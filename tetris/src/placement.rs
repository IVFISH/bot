use crate::errors::GameError;
use crate::board::Board;

use std::{collections::HashMap, fmt::Display};
use std::convert::TryInto;

pub const PIECE_SIZE: usize = 4;

pub type RotationState = usize;
pub type RotationDirection = usize;

pub type RotationLocation = [[Mino; PIECE_SIZE]; 4];

pub type Piece = usize;

const PIECE_ROTATIONS: [RotationLocation; 7] =
    [Z_ROTATIONS, L_ROTATIONS, O_ROTATIONS, S_ROTATIONS, I_ROTATIONS, J_ROTATIONS, T_ROTATIONS];

const Z_ROTATIONS: RotationLocation = [
    [Mino(1, -1), Mino(1, 0), Mino(0, 0), Mino(0, 1)],
    [Mino(1, 1), Mino(0, 1), Mino(0, 0), Mino(-1, 0)],
    [Mino(-1, 1), Mino(-1, 0), Mino(0, 0), Mino(0, -1)],
    [Mino(-1, -1), Mino(0, -1), Mino(0, 0), Mino(1, 0)]
];


const L_ROTATIONS: RotationLocation = [
    [Mino(1, 1), Mino(0, -1), Mino(0, 0), Mino(0, 1)],
    [Mino(-1, 1), Mino(1, 0), Mino(0, 0), Mino(-1, 0)],
    [Mino(-1, -1), Mino(0, 1), Mino(0, 0), Mino(0, -1)],
    [Mino(1, -1), Mino(-1, 0), Mino(0, 0), Mino(1, 0)]
];

const O_ROTATIONS: RotationLocation = [
    [Mino(1, 0), Mino(1, 1), Mino(0, 0), Mino(0, 1)],
    [Mino(0, 1), Mino(-1, 1), Mino(0, 0), Mino(-1, 0)],
    [Mino(-1, 0), Mino(-1, -1), Mino(0, 0), Mino(0, -1)],
    [Mino(0, -1), Mino(1, -1), Mino(0, 0), Mino(1, 0)]
];

const S_ROTATIONS: RotationLocation = [
    [Mino(1, 0), Mino(1, 1), Mino(0, -1), Mino(0, 0)],
    [Mino(0, 1), Mino(-1, 1), Mino(1, 0), Mino(0, 0)],
    [Mino(-1, 0), Mino(-1, -1), Mino(0, 1), Mino(0, 0)],
    [Mino(0, -1), Mino(1, -1), Mino(-1, 0), Mino(0, 0)]
];

const I_ROTATIONS: RotationLocation = [
    [Mino(0, -1), Mino(0, 0), Mino(0, 1), Mino(0, 2)],
    [Mino(1, 0), Mino(0, 0), Mino(-1, 0), Mino(-2, 0)],
    [Mino(0, 1), Mino(0, 0), Mino(0, -1), Mino(0, -2)],
    [Mino(-1, 0), Mino(0, 0), Mino(1, 0), Mino(2, 0)]
];

const J_ROTATIONS: RotationLocation = [
    [Mino(1, -1), Mino(0, -1), Mino(0, 0), Mino(0, 1)],
    [Mino(1, 1), Mino(1, 0), Mino(0, 0), Mino(-1, 0)],
    [Mino(-1, 1), Mino(0, 1), Mino(0, 0), Mino(0, -1)],
    [Mino(-1, -1), Mino(-1, 0), Mino(0, 0), Mino(1, 0)]
];

const T_ROTATIONS: RotationLocation = [
    [Mino(1, 0), Mino(0, -1), Mino(0, 0), Mino(0, 1)],
    [Mino(0, 1), Mino(1, 0), Mino(0, 0), Mino(-1, 0)],
    [Mino(-1, 0), Mino(0, 1), Mino(0, 0), Mino(0, -1)],
    [Mino(0, -1), Mino(-1, 0), Mino(0, 0), Mino(1, 0)]
];

pub struct Placement {

    pub piece_type: Piece,
    pub rotation_state: RotationState,
    pub center: Point

}

impl Placement {

    pub fn abs_locations(&self) -> [Point; PIECE_SIZE]{

        let out: [Point; PIECE_SIZE] =

            PIECE_ROTATIONS[self.piece_type][self.rotation_state]
                .iter()
                .map(
                    |x| x.add(&self.center).unwrap()
                )
                .collect::<Vec<Point>>()
                .try_into()
                .unwrap_or_else(|v: Vec<Point>| panic!("crash and burn"));

        out

    }

    pub fn move_by_vector(&mut self, v: MoveVector) -> bool {
         if let Ok(p) = v.add_to_point(&self.center) {
             self.center = p;
             return true;
        }
        return false;
    }

    pub fn left(&mut self) -> bool {
        self.move_by_vector(MoveVector(0, -1))
    }

    pub fn right(&mut self) -> bool {
        self.move_by_vector(MoveVector(0, 1))
    }

    pub fn down(&mut self) -> bool {
        self.move_by_vector(MoveVector(-1, 0))
    }

    fn up(&mut self) -> bool {
        self.move_by_vector(MoveVector(1, 0))
    }

    pub fn rotate(&mut self, direction: RotationDirection) {
        self.rotation_state = (self.rotation_state + direction) % PIECE_SIZE;
    }


}

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub row: usize,
    pub col: usize
}

pub struct MoveVector(i8, i8);

#[derive(Debug)]
struct Mino(i8, i8);

impl Mino {
    fn add(&self, other: &Point) -> Result<Point, GameError> {
        let row = self.0 + other.row as i8;
        let col = self.1 + other.col as i8;

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

impl MoveVector {

    fn add_to_point(&self, other: &Point) -> Result<Point, GameError> {
        let row = self.0 + other.row as i8;
        let col = self.1 + other.col as i8;

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
