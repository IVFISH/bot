use crate::board::Board;
use crate::piece::Piece;

#[derive(Default, Copy, Clone, Debug)]
pub struct Game {
    pub board: Board,
    pub active: Piece,
    pub hold: Option<u8>,
}

impl Game {
    // constructors -----------------------------
    /// returns a game with an empty board and random queue
    pub fn new() -> Self {
        unimplemented!()
    }
}

pub fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
