use crate::errors::GameError;

pub struct Placement {

    piece_type: char,
    rotation_state: i8,
    center: Point

}

impl Placement {

    pub fn to_list(&self) -> [Point; 4] {
        unimplemented!()
    }
}

pub struct Point {
    pub row: usize,
    pub col: usize
}

impl Point {
    pub fn add(&mut self, row: usize, col: usize) -> Result<Self, GameError> {
        self.row += row;
        self.col += col;

        todo!()
    }

    pub fn sub(&mut self, row: usize, col: usize) -> Result<Self, GameError> {
        self.col -= row;
        self.col -= col;

        todo!()
    }
}