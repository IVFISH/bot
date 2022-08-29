use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use rand::Rng;

use crate::errors::GameError;
use crate::placement::piece_data::Piece;

#[derive(Clone)]
pub struct PieceQueue {
    min_queue_length: usize,

    pub queue: VecDeque<Piece>,
    randomizer: BagType,

    // starts with seed
    num: usize,

    // uses lehmer random number generator (MINSTD)
    a: usize,
    m: usize,
}

impl Display for PieceQueue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for piece in 0..self.min_queue_length {
            write!(f, "{} ", PieceQueue::int_to_piece(self.queue[piece]))?;
        }
        Ok(())
    }
}

impl Default for PieceQueue {
    fn default() -> Self {
        Self {
            min_queue_length: 6,
            queue: VecDeque::new(),
            randomizer: BagType::SevenBag,
            num: 1,
            a: 16807,
            m: 2147483647,
        }
    }
}

impl PieceQueue {
    pub fn new(optional_seed: Option<usize>) -> Self {
        let num = optional_seed.unwrap_or_else(|| rand::thread_rng().gen_range(0..2147483646));

        Self {
            num,
            ..Default::default()
        }
    }

    pub fn manual_queue_set(&mut self, new_queue: VecDeque<Piece>) {
        self.queue = new_queue;
    }

    pub fn int_to_piece(piece: Piece) -> String {
        let arr = ["Z", "L", "O", "S", "I", "J", "T"];
        arr[piece].to_owned()
    }

    pub fn new_alt_randomizer(seed: usize, randomizer: BagType) -> Self {
        Self {
            num: seed,
            randomizer,
            ..Default::default()
        }
    }

    pub fn next(&mut self) -> Piece {
        if self.queue.len() < self.min_queue_length + 1 {
            self.next_bag();
        }

        let out = self.queue.pop_front().unwrap();

        out
    }

    pub fn peek(&self) -> Piece {
        self.queue[0]
    }

    fn next_bag(&mut self) {
        match self.randomizer {
            BagType::SevenBag => self.seven_bag(),
            BagType::FourteenBag => self.fourteen_bag(),
            BagType::Classic => self.classic(),
            BagType::Pairs => self.pairs(),
            BagType::Mayhem => self.total_mayhem(),
        }
    }

    fn seven_bag(&mut self) {
        let arr = self.shuffle_seven([0, 1, 2, 3, 4, 5, 6]);
        self.queue.extend(arr.iter());
    }

    fn fourteen_bag(&mut self) {
        let arr = self.shuffle_fourteen([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]);
        self.queue.extend(arr.iter());
    }

    fn classic(&mut self) {
        unimplemented!()
    }

    fn pairs(&mut self) {
        unimplemented!()
    }

    fn total_mayhem(&mut self) {
        unimplemented!()
    }

    fn next_num(&mut self) -> f32 {
        self.num = self.a * self.num % self.m;

        let out = (self.num - 1) as f32 / self.m as f32;
        out
    }

    fn shuffle_seven(&mut self, mut arr: [Piece; 7]) -> [Piece; 7] {
        for i in (1..7).rev() {
            let r = (self.next_num() * (i as f32 + 1.0)) as usize;
            (arr[i], arr[r]) = (arr[r], arr[i])
        }
        arr
    }

    fn shuffle_fourteen(&mut self, mut arr: [Piece; 14]) -> [Piece; 14] {
        for i in (1..14).rev() {
            let r = (self.next_num() * (i as f32 + 1.0)) as usize;
            (arr[i], arr[r]) = (arr[r], arr[i])
        }
        arr
    }
}

#[derive(PartialEq, Clone)]
pub struct GarbageItem {
    pub amt: usize,
    pub col: usize,
}

impl GarbageItem {
    pub fn new(amt: usize) -> Self {
        let rand_in_10 = rand::thread_rng().gen_range(0..10);

        Self {
            amt,
            col: rand_in_10,
        }
    }
}

#[derive(Clone)]
pub struct GarbageQueue {}

impl Default for GarbageQueue {
    fn default() -> Self {
        Self {}
    }
}

#[derive(Clone)]
pub enum BagType {
    SevenBag,
    FourteenBag,
    Classic,
    Pairs,
    Mayhem,
}

impl FromStr for BagType {
    type Err = GameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let out = match s {
            "7-bag" => BagType::SevenBag,
            "14-bag" => BagType::FourteenBag,
            "classic" => BagType::Classic,
            "pairs" => BagType::Pairs,
            "total mayhem" => BagType::Mayhem,
            other => {
                // return Err(GameError::UnknownBag);
                eprintln!("unknown bagtype '{}'", other);
                BagType::SevenBag
            }
        };

        Ok(out)
    }
}

#[cfg(test)]
mod piece_queue_tests {
    use super::*;

    #[test]
    fn test_seven_bag() {
        let mut queue = PieceQueue::default();
        let bag = queue.shuffle_seven([0, 1, 2, 3, 4, 5, 6]);

        for piece in 0..7 {
            assert!(bag.contains(&piece));
        }

        for _ in 0..10 {
            let new_bag = queue.shuffle_seven([0, 1, 2, 3, 4, 5, 6]);
            assert_ne!(bag, new_bag);
        }
    }

    #[test]
    fn test_match_with_osk() {
        let mut queue = PieceQueue::new(Some(15));
        // ITOSLJZS JOTZLIL
        let osk_queue = [4, 6, 2, 3, 1, 5, 0, 3, 5, 2, 6, 0, 1, 4];

        for piece in osk_queue {
            assert_eq!(queue.next(), piece);
        }

        let mut queue = PieceQueue::new(Some(7000));
        // TSJOLIZ ITLSJZO
        let osk_queue = [6, 3, 5, 2, 1, 4, 0, 4, 6, 1, 3, 5, 0, 2];

        for piece in osk_queue {
            assert_eq!(queue.next(), piece);
        }
    }
}

#[cfg(test)]
mod garbage_item_test {
    use super::*;

    #[test]
    fn test_random() {
        const N: usize = 10;
        let mut arr = vec![];
        for _ in 0..N {
            arr.push(GarbageItem::new(1));
        }

        // checks that not everything is the same
        assert!(!arr.windows(2).all(|w| w[0] == w[1]));
    }
}
