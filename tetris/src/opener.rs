#![allow(dead_code)]

use std::collections::VecDeque;
use crate::constants::piece_constants::NUM_PIECES;
use crate::constants::types::*;
use crate::Piece;


#[derive(Default)]
pub struct Dependency {
    pub eshanv2: Vec<PieceType>
}

#[derive(Default)]
pub struct Opener {
    pub opener_sequence: OpenerSequence,
    pub dependencies: Dependencies,
    bag: BagNumber,
    pub bag_placement: BagPlacement,
    bag_progress: usize,
    pub piece_order: PieceOrder
}


impl Opener {
    pub fn new(opener_sequence: OpenerSequence, dependencies: Dependencies) -> Self {
        Self {
            opener_sequence,
            dependencies,
            ..Default::default()
        }
    }

    pub fn bag_number(&self) -> BagNumber {
        self.bag
    }

    pub fn next_placement(&mut self, queue: &PieceOrder) -> Piece {
        let out = self.bag_placement[self.piece_order[self.bag_progress]].clone();
        self.bag_progress += 1;
        if self.bag_progress == NUM_PIECES {
            self.bag_progress = 0;
            self.bag += 1;
            self.solve_bag(queue);
        }
        return out;
    }

    fn solve_dependency(queue: &PieceOrder, dependency: &Dependency) -> Option<PieceOrder> {
        let mut queue: VecDeque<PieceType> = queue.clone().try_into().unwrap();
        let mut dependency: VecDeque<PieceType> = dependency.eshanv2.clone().try_into().unwrap();
        let important_pieces = dependency.clone();
        let mut out = Vec::new();

        while queue.len() > 0 {
            if !important_pieces.contains(&queue[0]) {
                out.push(queue.pop_front().unwrap());
                continue
            } else if queue.len() > 1 && !important_pieces.contains(&queue[1]) {
                out.push(queue.remove(1).unwrap());
                continue
            }

            if let Some(target) = dependency.pop_front() {
                if queue[0] == target {
                    out.push(queue.pop_front().unwrap());
                } else if queue.len() > 1 && queue[1] == target {
                    out.push(queue.remove(1).unwrap());
                } else {
                    return None
                }
            } else {
                out.extend(queue);
                return Some(out)
            }
        }
        Some(out)
    }

    pub fn solve_bag(&mut self, queue: &PieceOrder) -> bool {
        for dependency in self.dependencies.iter() {
            if let Some(piece_order) = Self::solve_dependency(queue, dependency) {
                self.piece_order = piece_order;
                return true;
            }
        }
        false
    }

}