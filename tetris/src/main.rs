mod board;
mod placement;

use board::Board;

fn main() {
    let board = Board::new();

    println!("{}", board);

}
