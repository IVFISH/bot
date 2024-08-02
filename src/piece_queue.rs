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
use crate::constants::queue_constants::*;

type PieceType = u8;

#[derive(Copy, Clone, Debug)]
pub struct PieceQueue {
    seed: usize,
    i: usize,
    queue: [u8; 14],
}

impl Default for PieceQueue {
    fn default() -> Self {
        Self::new(1337)
    }
}


impl PieceQueue {
    /// creates a new piece_queue with the given seed
    /// and sets the first 14 pieces
    pub fn new(seed: usize) -> Self {
        let mut out = Self { seed, i:0, queue:[255; 14]};
        let first_bag = &out.shuffle_seven()[0..7];
        out.queue[0..7].copy_from_slice(first_bag);

        let second_bag = &out.shuffle_seven()[0..7];
        out.queue[7..14].copy_from_slice(second_bag);
        // src[..limit].copy_from_slice(&dst[..limit]);
        out
    }

    /// returns the next piece in queue and shifts the queue
    /// pop_front
    pub fn next(&mut self) -> Piece {
        let p = Piece::new(self.queue[self.i]);
        self.i += 1;
        if self.i % 7 == 0 {
            let next_bag = &self.shuffle_seven()[0..7];
            self.queue[self.i-7..self.i].copy_from_slice(next_bag);
            self.i %= 14;
        }
        p
    }

    fn next_num(&mut self) -> f32 {
        self.seed = self.seed * MULTIPLIER % MODULUS;
        (self.seed - 1) as f32 / MODULUS as f32
    }

    fn shuffle_seven(&mut self) -> [PieceType; 7] {
        let mut arr = [0, 1, 2, 3, 4, 5, 6];
        for i in (1..7).rev() {
            let r = (self.next_num() * (i as f32 + 1.0)) as usize;
            (arr[i], arr[r]) = (arr[r], arr[i])
        }
        arr
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

    #[test]
    fn test_seven_bag() {
        let mut queue = PieceQueue::new(124274);
        for _ in 0..10 {
            let mut sum = 0;
            for _ in 0..7 {
                sum += queue.next().r#type;
            }
            assert_eq!(sum, 21)
        }
        
    }

    #[test]
    fn test_match_with_osk() {
        let mut queue = PieceQueue::new(15);
        // ITOSLJZS JOTZLIL
        let osk_queue = [4, 6, 2, 3, 1, 5, 0, 3, 5, 2, 6, 0, 1, 4];

        println!("{:?}", queue);
        for piece in osk_queue {
            assert_eq!(queue.next(), Piece::new(piece));
        }

        let mut queue = PieceQueue::new(7000);
        // TSJOLIZ ITLSJZO
        let osk_queue = [6, 3, 5, 2, 1, 4, 0, 4, 6, 1, 3, 5, 0, 2];

        println!("{:?}", queue);
        for piece in osk_queue {
            assert_eq!(queue.next(), Piece::new(piece));
        }
    }
}
