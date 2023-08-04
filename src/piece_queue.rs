// hi whoever is reading this <3
// piece_queue should just be an array of 14 pieces
// with a pointer (index) at the current piece
// every time index hits 7, generate
// the next 7 pieces and reset index to 0
// (but im too lazy to implement and i dont have wifi so cope)
// (uh if we get bored we can optimize this to be a single u64)
// (3 bits for each piece, x14 pieces = 42 bits)
// thinking about it more the bit is quite nice lmao

use crate::piece::Piece;

#[derive(Default, Copy, Clone, Debug)]
pub struct PieceQueue {
    n: usize,
}

impl PieceQueue {
    /// creates a new piece_queue with the given seed
    /// and sets the first 14 pieces
    pub fn new(_seed: usize) -> Self {
        Default::default()
    }

    /// returns the next piece in queue and shifts the queue
    /// pop_front
    pub fn next(&mut self) -> Piece {
        self.n += 1;
        Piece::new((self.n as u8 - 1) % 7)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_output() {
        let mut q = PieceQueue::new(1337);
        for _ in 0..14 {
            println!("{:?}", q.next());
        }
    }
}