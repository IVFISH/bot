#![allow(dead_code)]

use crate::command::Command;
use crate::piece::Piece;
use crate::controller::Controller;
use crate::board::Board;
use std::iter::zip;

pub struct Placement<'a> {
    pub piece: Piece,

    // references to the source for generating the move
    pub trivial_base: &'a Vec<Command>,
    pub nontrivial_extension: &'a Vec<Command>,
    pub nontrivial_index: usize, // this is the exclusive end index

    // add the score and any other info here
}

pub struct PlacementList<'a> {
    pub placements: Vec<Placement<'a>>,
    pub trivials: Vec<Vec<Command>>,
    pub nontrivials: Vec<Vec<Command>>,
}

impl<'a> PlacementList<'a> {
    pub fn new(trivials: Vec<Vec<Command>>, nontrivials: Vec<Vec<Command>>) -> Self {
        Self {
            placements: Vec::new(),
            trivials,
            nontrivials,
        }
    }

    pub fn fill_placements(&'a mut self, mut controller: Controller, mut piece: Piece, board: &Board) {
        for (trivial, nontrivial) in zip(self.trivials.iter(), self.nontrivials.iter()) {
            for (i, command) in nontrivial.iter().enumerate() {
                controller.do_command(*command, &mut piece, board, false);
                self.placements.push(Placement {
                    piece,
                    trivial_base: trivial,
                    nontrivial_extension: nontrivial,
                    nontrivial_index: i + 1,
                });
            }
        }

    }
}
