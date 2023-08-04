// hi whoever is reading this <3
// piece_queue should just be an array of 14 pieces
// with a pointer (index) at the current piece
// every time index hits 7, generate
// the next 7 pieces and reset index to 0
// (but im too lazy to implement and i dont have wifi so cope)
// (uh if we get bored we can optimize this to be a single u64)
// (3 bits for each piece, x14 pieces = 42 bits)
// thinking about it more the bit is quite nice lmao

use crate::constants::queue_constants::*;
use crate::piece::Piece;
use itertools::chain;

#[derive(Copy, Clone, Debug)]
pub struct PieceQueue {
    data: u64,
    seed: usize,
    index: u8,
}

impl PieceQueue {
    /// creates a new piece_queue with the given seed
    /// and sets the first 14 pieces
    pub fn new(seed: usize) -> Self {
        let mut out = Self {
            seed,
            index: 0,
            data: 0,
        };

        // set the first 21 pieces
        for (i, piece) in chain!(
            out.shuffle_seven(),
            out.shuffle_seven(),
            out.shuffle_seven()
        )
        .enumerate()
        {
            out.data |= piece << (i * PIECE_BITS);
        }

        out
    }

    /// returns the next piece in queue and shifts the queue
    pub fn next(&mut self) -> Piece {
        let out = Piece::new((self.data >> (self.index as usize * PIECE_BITS) & 0b111) as u8);
        self.index += 1;
        if self.index >= 7 {
            self.next_bag();
        }
        out
    }

    /// removes the first 7 pieces stored in data and then
    /// adds 7 more pieces generated by the tetris
    /// random seed. this also updates the seed
    /// precondition: index = 7, all 21 pieces are initialized
    fn next_bag(&mut self) {
        // removes the first seven pieces
        self.data >>= 7 * PIECE_BITS;
        self.index = 0;

        // append bag to queue
        for (i, piece) in self.shuffle_seven().into_iter().enumerate() {
            self.data |= piece << (14 + i) * PIECE_BITS;
        }
    }

    /// creates a shuffled 7-bag (mutates the seed)
    fn shuffle_seven(&mut self) -> [u64; 7] {
        // generates the next bag
        let mut arr = [0, 1, 2, 3, 4, 5, 6];
        for i in (1..7).rev() {
            let r = (self.next_num() * (i as f32 + 1.0)) as usize;
            [arr[i], arr[r]] = [arr[r], arr[i]];
        }
        arr
    }

    /// sets the next seed
    fn next_num(&mut self) -> f32 {
        self.seed = self.seed * MULTIPLIER % MODULUS;
        (self.seed - 1) as f32 / MODULUS as f32
    }
}

#[cfg(test)]
mod piece_queue_tests {
    use super::*;

    #[test]
    fn show_output() {
        // run with -- --show-output
        let mut queue = PieceQueue::new(4);
        println!("{:#b}", queue.data);
        for _ in 0..15 {
            println!("{:?}", queue.next());
        }
    }

    #[test]
    fn test_match_with_osk() {
        let mut queue = PieceQueue::new(15);
        // ITOSLJZS JOTZLIL
        let osk_queue = [4, 6, 2, 3, 1, 5, 0, 3, 5, 2, 6, 0, 1, 4];

        for piece in osk_queue {
            assert_eq!(queue.next(), Piece::new(piece));
        }

        let mut queue = PieceQueue::new(7000);
        // TSJOLIZITLSJZO

        let osk_queue = [6, 3, 5, 2, 1, 4, 0, 4, 6, 1, 3, 5, 0, 2];
        for piece in osk_queue {
            assert_eq!(queue.next(), Piece::new(piece));
        }
    }
}