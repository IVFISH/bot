mod board;
mod placement;
mod errors;

use board::Board;

fn main() {
    let board = Board::new();

    println!("{}", board);

}
