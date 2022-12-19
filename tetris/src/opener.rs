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
    pub dependencies_list: Vec<Vec<Dependencies>>,
    bag: BagNumber,
    bag_progress: usize,
    variant: usize,
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
        // NDT
        // const NDT00: [Piece; 7] = [
        //     Piece{piece_type: 0, rotation_state: 0, center: Point(1,8), last_kick: 0},
        //     Piece{piece_type: 1, rotation_state: 0, center: Point(0,4), last_kick: 0},
        //     Piece{piece_type: 2, rotation_state: 0, center: Point(0,0), last_kick: 0},
        //     Piece{piece_type: 3, rotation_state: 0, center: Point(1,4), last_kick: 0},
        //     Piece{piece_type: 4, rotation_state: 1, center: Point(2,6), last_kick: 0},
        //     Piece{piece_type: 5, rotation_state: 0, center: Point(0,8), last_kick: 0},
        //     Piece{piece_type: 6, rotation_state: 0, center: Point(3,4), last_kick: 0},
        // ];
        //
        // let DEP00: Dependencies = vec![
        //     Dependency{ dependency: vec![1, 3, 6] }, // L<S<T
        //     Dependency{ dependency: vec![5, 0] },    // J<Z
        // ];
        //
        // const NDT01: [Piece; 7] = [
        //     Piece{piece_type: 0, rotation_state: 3, center: Point(1,4), last_kick: 0},
        //     Piece{piece_type: 1, rotation_state: 1, center: Point(1,7), last_kick: 0},
        //     Piece{piece_type: 2, rotation_state: 0, center: Point(0,0), last_kick: 0},
        //     Piece{piece_type: 3, rotation_state: 1, center: Point(1,8), last_kick: 0},
        //     Piece{piece_type: 4, rotation_state: 1, center: Point(2,6), last_kick: 0},
        //     Piece{piece_type: 5, rotation_state: 3, center: Point(1,5), last_kick: 0},
        //     Piece{piece_type: 6, rotation_state: 0, center: Point(3,4), last_kick: 0},
        // ];
        //
        // let DEP01: Dependencies = vec![
        //     Dependency{ dependency: vec![5, 6] },    // J<T
        //     Dependency{ dependency: vec![1, 4] },    // L<I
        // ];
        //
        // const NDT10: [Piece; 7] = [
        //     Piece{piece_type: 0, rotation_state: 1, center: Point(5,6), last_kick: 0},
        //     Piece{piece_type: 1, rotation_state: 3, center: Point(5,3), last_kick: 0},
        //     Piece{piece_type: 2, rotation_state: 0, center: Point(3,7), last_kick: 0},
        //     Piece{piece_type: 3, rotation_state: 1, center: Point(5,4), last_kick: 0},
        //     Piece{piece_type: 4, rotation_state: 1, center: Point(4,9), last_kick: 0},
        //     Piece{piece_type: 5, rotation_state: 1, center: Point(3,0), last_kick: 0},
        //     Piece{piece_type: 6, rotation_state: 2, center: Point(2,2), last_kick: 0},
        // ];
        // let DEP10: Dependencies = vec![ // can be improved, idc
        //     Dependency{ dependency: vec![2, 0] },   // O<Z
        //     Dependency{ dependency: vec![0, 6] },   // T must be last
        //     Dependency{ dependency: vec![1, 6] },
        //     Dependency{ dependency: vec![2, 6] },
        //     Dependency{ dependency: vec![3, 6] },
        //     Dependency{ dependency: vec![4, 6] },
        //     Dependency{ dependency: vec![5, 6] },
        // ];
        //
        // const NDT11: [Piece; 7] = [
        //     Piece{piece_type: 0, rotation_state: 1, center: Point(5,6), last_kick: 0},
        //     Piece{piece_type: 1, rotation_state: 3, center: Point(5,3), last_kick: 0},
        //     Piece{piece_type: 2, rotation_state: 0, center: Point(3,7), last_kick: 0},
        //     Piece{piece_type: 3, rotation_state: 1, center: Point(5,4), last_kick: 0},
        //     Piece{piece_type: 4, rotation_state: 1, center: Point(4,9), last_kick: 0},
        //     Piece{piece_type: 5, rotation_state: 1, center: Point(3,0), last_kick: 0},
        //     Piece{piece_type: 6, rotation_state: 2, center: Point(2,2), last_kick: 0},
        // ];
        // let DEP11: Dependencies = vec![ // can be improved, idc
        //                                 Dependency{ dependency: vec![2, 4] },   // O<I
        //                                 Dependency{ dependency: vec![0, 6] },   // T must be last
        //                                 Dependency{ dependency: vec![1, 6] },
        //                                 Dependency{ dependency: vec![2, 6] },
        //                                 Dependency{ dependency: vec![3, 6] },
        //                                 Dependency{ dependency: vec![4, 6] },
        //                                 Dependency{ dependency: vec![5, 6] },
        // ];
        //
        //  Self {
        //      opener_sequence: vec![vec![NDT00, NDT01], vec![NDT10, NDT11]],
        //      dependencies_list: vec![vec![DEP00, DEP01], vec![DEP10, DEP11]],
        //      bag: 0,
        //      bag_progress: 0,
        //      variant: 0,
        //      piece_order: vec![],
        //      status: Default::default()
        //  }


        Self {
            opener_sequence: vec![vec![[
                Piece { piece_type: 0, rotation_state: 0, center: Point(1, 4), last_kick: 0 },
                Piece { piece_type: 1, rotation_state: 1, center: Point(1, 0), last_kick: 0 },
                Piece { piece_type: 2, rotation_state: 0, center: Point(0, 8), last_kick: 0 },
                Piece { piece_type: 3, rotation_state: 1, center: Point(1, 6), last_kick: 0 },
                Piece { piece_type: 4, rotation_state: 0, center: Point(0, 4), last_kick: 0 },
                Piece { piece_type: 5, rotation_state: 2, center: Point(3, 4), last_kick: 0 },
                Piece { piece_type: 6, rotation_state: 2, center: Point(1, 2), last_kick: 0 }]]], // always invalid
            dependencies_list: vec![vec!(vec![Dependency{dependency: vec![4, 0, 5, 6]}, Dependency{dependency: vec![1,6]}, Dependency{dependency: vec![2,6]}])],
            bag: 0,
            bag_progress: 0,
            variant: 0,
            piece_order: vec![],
            status: Default::default()
        }
    }
}

impl Opener {
    pub fn new(opener_sequence: OpenerSequence, dependencies_list: Vec<Vec<Dependencies>>) -> Self {
        Self {
            opener_sequence,
            dependencies_list,
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
        let out = self.opener_sequence[self.bag][self.variant][self.piece_order[self.bag_progress]].clone();
        self.bag_progress += 1;
        if self.bag_progress == NUM_PIECES {
            self.bag_progress = 0;
            self.bag += 1;
            println!("{}, {}", self.bag, self.opener_sequence.len());
            if (self.bag >= self.opener_sequence.len() || !self.solve_bag(queue)) {
                self.status = OpenerStatus::Invalid;
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
        for i in 1..self.opener_sequence[self.bag].len() {
            let mut queue = queue.clone();
            let hold = queue.remove(0);

            let variations = Self::queue_variations(queue, hold);
            let mut filtered = variations
                .into_iter()
                .filter(|queue| Self::satisfy_dependencies(queue, &self.dependencies_list[self.bag][i]))
                .collect::<Vec<PieceOrder>>();
            if let Some(queue) = filtered.pop() {
                self.piece_order = queue;
                self.variant = i;
                return true;
            }
        }
        false
    }

}