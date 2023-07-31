#![allow(dead_code)]

use crate::board::Board;
use crate::command::Command;
use crate::controller::Controller;
use crate::piece::Piece;
use std::iter::zip;
use std::rc::Rc;

pub struct Placement {
    pub piece: Piece,

    // references to the source for generating the move
    pub trivial_base: Rc<Vec<Command>>,
    pub nontrivial_extension: Rc<Vec<Command>>,
    pub nontrivial_index: usize, // this is the exclusive end index

                                 // add the score and any other info here
}

pub struct PlacementList {
    pub placements: Vec<Placement>,
    pub trivials: Vec<Rc<Vec<Command>>>,
    pub nontrivials: Vec<Rc<Vec<Command>>>,
}

impl PlacementList {
    pub fn new(
        trivials: Vec<Vec<Command>>,
        nontrivials: Vec<Vec<Command>>,
        controller: Controller,
        piece: Piece,
        board: &Board,
    ) -> Self {
        // wrap all the vectors of commands in Rc
        let trivials: Vec<Rc<Vec<Command>>> = trivials.into_iter().map(|v| Rc::new(v)).collect();
        let nontrivials: Vec<Rc<Vec<Command>>> =
            nontrivials.into_iter().map(|v| Rc::new(v)).collect();
        let placements = Self::get_placements(&trivials, &nontrivials, controller, piece, board);
        Self {
            placements,
            trivials,
            nontrivials,
        }
    }

    fn get_placements(
        trivials: &Vec<Rc<Vec<Command>>>,
        nontrivials: &Vec<Rc<Vec<Command>>>,
        controller: Controller,
        mut piece: Piece,
        board: &Board,
    ) -> Vec<Placement> {
        let mut placements = Vec::new();
        for (trivial, nontrivial) in zip(trivials, nontrivials) {
            for (i, command) in nontrivial.iter().enumerate() {
                controller.do_command(command, &mut piece, board);
                placements.push(Placement {
                    piece,
                    trivial_base: Rc::clone(trivial),
                    nontrivial_extension: Rc::clone(nontrivial),
                    nontrivial_index: i + 1,
                });
            }
        }
        placements
    }
}
