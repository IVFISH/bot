use std::fmt::{Display, Formatter};
use crate::Placement;
use crate::placement::piece_data::Piece;

pub struct PieceQueue {
    min_queue_length: usize,

    queue: Vec<Piece>,
    randomizer: RNG,

    // starts with seed
    num: usize,

    // uses lehmer random number generator (MINSTD)
    a: usize,
    m: usize,
}

impl Display for PieceQueue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for piece in 0..self.min_queue_length {
            write!(f, "{} ", self.queue[piece])?;
        }
        Ok(())
    }
}

impl Default for PieceQueue {
    fn default() -> Self {
        Self {
            min_queue_length: 6,
            queue: vec![],
            randomizer: RNG::SevenBag,
            num: 1,
            a: 16807,
            m: 2147483647
        }
    }
}

impl PieceQueue {

    #[allow(unused)]
    pub fn new(randomizer: RNG) -> Self {
        Self {
            randomizer,
            ..Default::default()
        }
    }

    pub fn next(&mut self) -> Piece {
        let out = self.queue.pop().unwrap();

        if self.queue.len() < self.min_queue_length {
            self.next_bag();
        }

        out
    }

    pub fn peek(&self) -> Piece {
        self.queue[0]
    }

    fn next_bag(&mut self) {
        match self.randomizer {
            RNG::SevenBag => self.seven_bag(),
            RNG::FourteenBag => self.fourteen_bag(),
            RNG::Classic => self.classic(),
            RNG::Pairs => self.pairs(),
            RNG::Mayhem => self.total_mayhem()
        }
    }

    fn next_num(&mut self) -> usize {
        self.num = self.a * self.num % self.m;

        (self.num - 1) / self.m
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

    fn shuffle_seven(&mut self, mut arr: [Piece; 7]) -> [Piece; 7] {
        for i in (0..7).rev() {
            let r = self.next_num() * (i + 1);
            (arr[i], arr[r]) = (arr[r], arr[i])
        }
        arr
    }

    fn shuffle_fourteen(&mut self, mut arr: [Piece; 14]) -> [Piece; 14] {
        for i in (0..14).rev() {
            let r = self.next_num() * (i + 1);
            (arr[i], arr[r]) = (arr[r], arr[i])
        }
        arr
    }
}

pub struct GarbageQueue {}

pub enum RNG {
    SevenBag,
    FourteenBag,
    Classic,
    Pairs,
    Mayhem,
}