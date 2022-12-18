#![allow(dead_code)]

use std::collections::VecDeque;
use crate::constants::piece_constants::NUM_PIECES;
use crate::constants::types::*;
use crate::{Piece, Point};
use crate::queue::PieceQueue;


#[derive(Default)]
pub struct Dependency {
    pub dependency: Vec<PieceType>
}

#[derive(PartialEq, Debug)]
pub enum OpenerStatus {
    New,
    Active,
    Invalid,
}

pub struct Opener {
    pub opener_sequence: OpenerSequence,
    pub dependencies: Dependencies,
    bag: BagNumber,
    bag_progress: usize,
    pub piece_order: PieceOrder,
    pub status: OpenerStatus,
}

impl Default for OpenerStatus {
    fn default() -> Self {
        OpenerStatus::New
    }
}


impl Default for Opener {
    fn default() -> Self {
        Self {
            opener_sequence: vec!([
                Piece { piece_type: 0, rotation_state: 0, center: Point(1, 4), last_kick: 0 },
                Piece { piece_type: 1, rotation_state: 1, center: Point(1, 0), last_kick: 0 },
                Piece { piece_type: 2, rotation_state: 0, center: Point(0, 8), last_kick: 0 },
                Piece { piece_type: 3, rotation_state: 1, center: Point(1, 6), last_kick: 0 },
                Piece { piece_type: 4, rotation_state: 0, center: Point(0, 4), last_kick: 0 },
                Piece { piece_type: 5, rotation_state: 2, center: Point(3, 4), last_kick: 0 },
                Piece { piece_type: 6, rotation_state: 0, center: Point(0, 4), last_kick: 0 }, // always invalid
            ]),
            dependencies: vec!(Dependency{dependency: vec![4, 0, 5, 6]}),
            bag: 0,
            bag_progress: 0,
            piece_order: vec![],
            status: Default::default()
        }
    }
}

impl Opener {
    pub fn new(opener_sequence: OpenerSequence, dependencies: Dependencies) -> Self {
        Self {
            opener_sequence,
            dependencies,
            ..Default::default()
        }
    }

    pub fn init(&mut self, queue: &PieceOrder) {
        if self.solve_bag(queue) {
            self.status = OpenerStatus::Active
        } else {
            self.status = OpenerStatus::Invalid
        }
        println!("With queue {:?}, Opener is {:?}", queue, self.status);
    }

    pub fn bag_number(&self) -> BagNumber {
        self.bag
    }

    pub fn next_placement(&mut self, queue: &PieceOrder) -> Piece {
        let out = self.opener_sequence[self.bag][self.piece_order[self.bag_progress]].clone();
        self.bag_progress += 1;
        if self.bag_progress == NUM_PIECES {
            self.bag_progress = 0;
            self.bag += 1;
            if (self.bag > self.opener_sequence.len()) || self.solve_bag(queue) {
                self.status = OpenerStatus::Invalid
            }
        }
        return out;
    }

    fn satisfy_dependency(queue: &PieceOrder, dependency: &Dependency) -> bool {
        queue
            .iter()
            .filter(|piece| dependency.dependency.contains(piece))
            .zip(&dependency.dependency)
            .all(|(a, b)| a == b)
    }

    fn satisfy_dependencies(queue: &PieceOrder, dependencies: &Dependencies) -> bool {
        dependencies
            .iter()
            .all(|dependency| Self::satisfy_dependency(queue, dependency))
    }

    fn queue_variations(queue: PieceOrder, hold: PieceType) -> Vec<PieceOrder> {
        if queue.len() < 1 {
            vec![vec![hold]]
        } else {
            let mut out = Vec::new();
            let mut cdr = queue.clone();
            let car = cdr.remove(0);

            for mut variation in Self::queue_variations(cdr.clone(), hold) {
                variation.insert(0, car);
                out.push(variation);
            }
            for mut variation in Self::queue_variations(cdr.clone(), car) {
                variation.insert(0, hold);
                out.push(variation);
            }

            out
        }
    }

    pub fn solve_bag(&mut self, queue: &PieceOrder) -> bool {
        let mut queue = queue.clone();
        let hold = queue.remove(0);

        let variations = Self::queue_variations(queue, hold);
        let mut filtered = variations
            .into_iter()
            .filter(|queue| Self::satisfy_dependencies(queue, &self.dependencies))
            .collect::<Vec<PieceOrder>>();
        if let Some(queue) = filtered.pop() {
            self.piece_order = queue;
            true
        } else {
            false
        }
    }

}