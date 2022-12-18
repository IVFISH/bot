#![allow(dead_code)]

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
    pub fn new(opener_sequence :OpenerSequence, dependencies: Dependencies) -> Self {
        Self {
            opener_sequence,
            dependencies,
            ..Default::default()
        }
    }

    pub fn bag_number(&self) -> BagNumber {
        self.bag
    }

    pub fn next_move(&mut self) -> &Piece {
        let out = &self.bag_placement[self.piece_order[self.bag_progress]];
        self.bag_progress += 1;
        if self.bag_progress = NUM_PIECES {
            self.bag_progress = 0;
            self.bag += 1;
            self.solve_bag();
        }
        return out;
    }

    pub fn solve_bag(&self, queue: pieceOrder) -> bool {
        todo!()
    }

}