#![allow(dead_code)]

use crate::constants::queue_constants::*;
use crate::constants::types::*;
use crate::piece::Piece;
use rand::Rng;
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use std::string::ParseError;

#[derive(Default, Clone)]
pub struct PieceQueue {
    queue: VecDeque<PieceType>,
    randomizer: BagType,
    seed: usize,
}

impl PieceQueue {
    pub fn new(optional_seed: Option<usize>) -> Self {
        let mut out = Self {
            seed: optional_seed.unwrap_or_else(|| rand::thread_rng().gen_range(0..MODULUS - 1)),
            ..Default::default()
        };

        out.next_bag();
        out
    }

    pub fn new_alt_randomizer(optional_seed: Option<usize>, randomizer: BagType) -> Self {
        Self {
            seed: optional_seed.unwrap_or_else(|| rand::thread_rng().gen_range(0..MODULUS - 1)),
            randomizer,
            ..Default::default()
        }
    }

    pub fn set_queue(&mut self, new_queue: VecDeque<PieceType>) {
        self.queue = new_queue;
    }

    pub fn peek(&self) -> PieceType {
        *self.queue.front().unwrap()
    }

    pub fn peek_index(&self, index: usize) -> PieceType { *self.queue.get(index).unwrap()}

    pub fn next(&mut self) -> Piece {
        if self.queue.len() < MIN_QUEUE_LENGTH + 1 {
            self.next_bag();
        }
        Piece::new(self.pop())
    }

    fn pop(&mut self) -> PieceType {
        self.queue.pop_front().unwrap()
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
        self.seed = self.seed * MULTIPLIER % MODULUS;
        (self.seed - 1) as f32 / MODULUS as f32
    }

    fn shuffle_seven(&mut self, mut arr: [PieceType; 7]) -> [PieceType; 7] {
        for i in (1..7).rev() {
            let r = (self.next_num() * (i as f32 + 1.0)) as usize;
            (arr[i], arr[r]) = (arr[r], arr[i])
        }
        arr
    }

    fn shuffle_fourteen(&mut self, mut arr: [PieceType; 14]) -> [PieceType; 14] {
        for i in (1..14).rev() {
            let r = (self.next_num() * (i as f32 + 1.0)) as usize;
            (arr[i], arr[r]) = (arr[r], arr[i])
        }
        arr
    }
}

pub fn piece_type_to_string(piece: PieceType) -> String {
    let arr = ["Z", "L", "O", "S", "I", "J", "T"];
    arr[piece].to_owned()
}

impl Display for PieceQueue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for piece in 0..MIN_QUEUE_LENGTH {
            write!(f, "{} ", piece_type_to_string(self.queue[piece]))?;
        }
        Ok(())
    }
}

#[derive(Clone, PartialEq)]
pub enum BagType {
    SevenBag,
    FourteenBag,
    Classic,
    Pairs,
    Mayhem,
}

impl Default for BagType {
    fn default() -> Self {
        Self::SevenBag
    }
}

impl FromStr for BagType {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let out = match s {
            "7-bag" => BagType::SevenBag,
            "14-bag" => BagType::FourteenBag,
            "classic" => BagType::Classic,
            "pairs" => BagType::Pairs,
            "total mayhem" => BagType::Mayhem,
            other => {
                eprintln!("unknown bag type '{}'", other);
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
        let mut queue = PieceQueue::new(None);
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
            assert_eq!(queue.next(), Piece::new(piece));
        }

        let mut queue = PieceQueue::new(Some(7000));
        // TSJOLIZ ITLSJZO
        let osk_queue = [6, 3, 5, 2, 1, 4, 0, 4, 6, 1, 3, 5, 0, 2];

        for piece in osk_queue {
            assert_eq!(queue.next(), Piece::new(piece));
        }
    }
}
