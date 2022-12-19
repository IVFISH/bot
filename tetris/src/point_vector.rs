#![allow(dead_code)]

use crate::constants::board_constants::*;
use crate::constants::piece_constants::*;

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Point(pub i8, pub i8);

impl Default for Point {
    fn default() -> Self {
        Self(SPAWN_ROW, SPAWN_COL)
    }
}

impl Point {
    pub fn add(&self, other: &Self) -> Point {
        Point(self.0 + other.0, self.1 + other.1)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PointVector(pub i8, pub i8);

impl PointVector {
    pub fn add_to_point(&self, other: &Point) -> Option<Point> {
        let row = self.0 + other.0;
        let col = self.1 + other.1;

        if row >= 0 && row < BOARD_HEIGHT as i8 && col >= 0 && col < BOARD_WIDTH as i8 {
            return Some(Point(row, col));
        }

        None
    }

    pub fn unsafe_add_to_point(&self, other: &Point) -> Option<Point> {
        let row = self.0 + other.0;
        let col = self.1 + other.1;

        if row >= 0 && row < BOARD_HEIGHT as i8 && col >= 0 && col < BOARD_WIDTH as i8 {
            return Some(Point(row, col));
        }

        None
    }

    pub fn negative(&self) -> Self {
        Self(-self.0, -self.1)
    }
}
