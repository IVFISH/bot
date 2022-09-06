#![allow(dead_code)]

use crate::constants::offset::*;
use crate::constants::piece_constants::*;
use crate::constants::rotation::*;
use crate::constants::types::*;
use crate::point_vector::*;
use std::fmt::{Display, Formatter};

#[derive(Default, Clone, Debug)]
pub struct Piece {
    piece_type: PieceType,
    rotation_state: RotationState,
    center: Point,
    last_kick: usize,
}

impl PartialEq for Piece {
    fn eq(&self, other: &Self) -> bool {
        self.piece_type == other.piece_type
            && self.rotation_state == other.rotation_state
            && self.center == other.center
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let locations = &PIECE_ROTATIONS[self.piece_type][self.rotation_state];

        let size: i8;

        if self.piece_type == 4 {
            size = 5;
        } else {
            size = 3;
        }
        let half_size = size / 2;

        for row in (0..size).rev() {
            for col in 0..size {
                let p = Point(row - half_size, col - half_size);
                if locations.contains(&p) {
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

impl Piece {
    // getters
    pub fn abs_locations(&self) -> Option<Vec<Point>> {
        // errors if theres a negative index

        let rotation_locations = &PIECE_ROTATIONS[self.piece_type][self.rotation_state];
        let mut out = Vec::with_capacity(4);

        for location in rotation_locations {
            if let Some(location) = location.add(&self.center) {
                out.push(location);
            } else {
                return None;
            }
        }

        Some(out)
    }

    pub fn get_type(&self) -> PieceType {
        self.piece_type
    }

    pub fn get_rotation_state(&self) -> RotationState {
        self.rotation_state
    }

    pub fn get_center(&self) -> Point {
        self.center
    }

    pub fn get_last_kick(&self) -> usize {self.last_kick }

    pub fn get_kicks(&self, dir: RotationDirection) -> Vec<PointVector> {
        let before = self.rotation_state;
        let kicks;
        if self.piece_type == 4 {
            // I piece is the special child
            if dir == 2 {
                kicks = FIVE_180_OFFSETS[before].to_vec()
            } else {
                kicks = FIVE_OFFSETS[before][dir / 2].to_vec();
            }
        } else if self.piece_type == 2 {
            // O piece is the other special child
            kicks = vec![O_OFFSETS[before][dir - 1]];
        } else {
            if dir == 2 {
                kicks = THREE_180_OFFSETS[before].to_vec()
            } else {
                kicks = THREE_OFFSETS[before][dir / 2].to_vec();
            }
        }
        kicks
    }

    pub fn get_rotation(&self) -> RotationState {
        self.rotation_state
    }

    pub fn get_row(&self) -> usize {
        self.center.0 as usize
    }

    pub fn get_col(&self) -> usize {
        self.center.1 as usize
    }

    // setters
    pub fn set_center(&mut self, point: Point) {
        self.center = point;
    }
    pub fn set_row(&mut self, row: i8) {
        self.center.0 = row
    }

    pub fn set_col(&mut self, col: i8) {
        self.center.1 = col
    }

    pub fn set_kick(&mut self, new_kick: usize) {
        self.last_kick = new_kick;
    }
    // methods
    pub fn new(piece_type: PieceType) -> Self {
        Self {
            piece_type,
            ..Default::default()
        }
    }

    // move
    pub fn moved(&mut self, v: PointVector) -> bool {
        if let Some(point) = v.add_to_point(&self.center) {
            self.center = point;
            return true;
        }
        false
    }

    pub fn ret_moved(&self, v: PointVector) -> Option<Self> {
        let mut piece = self.clone();
        if piece.moved(v) {
            return Some(piece);
        }
        None
    }

    // rotate
    pub fn rotate(&mut self, direction: RotationDirection) {
        self.rotation_state = (self.rotation_state + direction) % NUM_ROTATE_STATES;
    }

    pub fn ret_rotated(&self, direction: RotationDirection) -> Self {
        let mut out = self.clone();
        out.rotate(direction);
        out
    }
}
