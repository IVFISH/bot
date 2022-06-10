mod board;
mod placement;
mod errors;

use board::Board;
use crate::placement::{MoveVector, Placement, Point};

fn main() {
    // let board = Board::new();
    //
    // println!("{}", board);

    let mut piece = Placement {
        piece_type: 0,
        rotation_state: 0,
        center: Point {row: 1, col: 1}
    };

    println!("{:?}", piece.abs_locations());

    piece.right();

    println!("{:?}", piece.abs_locations());


}
