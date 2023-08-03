#![allow(dead_code)]

use crate::command::Command;
use crate::controller::Controller;
use crate::game::Game;
use crate::piece::Piece;
use itertools::chain;
use std::iter::zip;
use std::rc::Rc;

pub struct Placement {
    // the last piece in the move sequence
    pub piece: Piece,

    // references to the source for generating the move
    pub game: Rc<Game>,         // the starting game (depth=0)
    pub pieces: Rc<Vec<Piece>>, // all the prior pieces in lookahead

    // regenerating the first move in the sequence
    pub trivial_base: Rc<Vec<Command>>,
    pub nontrivial_extension: Rc<Vec<Command>>,
    pub nontrivial_index: usize, // this is the exclusive end index

                                 // add the score and any other info here
}

impl Placement {
    /// gets the command sequence to get to this placement
    /// starting from the spawn condition
    pub fn get_command_sequence(&self) -> Vec<Command> {
        let mut commands = Vec::new();
        for command in chain(
            self.trivial_base.iter(),
            &self.nontrivial_extension[0..self.nontrivial_index],
        ) {
            Self::add_command(&mut commands, *command);
        }
        commands
    }

    fn add_command(commands: &mut Vec<Command>, command: Command) {
        match command {
            Command::Backtrack(n) => commands.truncate(commands.len() - n),
            Command::Null | Command::MoveHorizontal(0) | Command::Rotate(0) => (),
            Command::MoveHorizontal(n) => {
                commands.extend(vec![Command::MoveHorizontal(1); n.abs() as usize])
            }
            _ => commands.push(command),
        }
    }
}

#[derive(Default)]
pub struct PlacementList {
    pub placements: Vec<Placement>,
    pub trivials: Vec<Rc<Vec<Command>>>,
    pub nontrivials: Vec<Rc<Vec<Command>>>,
}

impl PlacementList {
    pub fn get_placements(
        trivials: &Vec<Rc<Vec<Command>>>,
        nontrivials: &Vec<Rc<Vec<Command>>>,
        game: Rc<Game>,
        pieces: Rc<Vec<Piece>>,
    ) -> Vec<Placement> {
        let mut piece = game.active;
        let mut controller = Controller::new(&mut piece, &game.board);

        let mut placements = Vec::new();
        for (trivial, nontrivial) in zip(trivials, nontrivials) {
            controller.do_commands(trivial);
            for (i, command) in nontrivial.iter().enumerate() {
                controller.do_command_mut(*command);
                if let &Command::Backtrack(_) = command {
                    continue;
                }
                if controller.board.piece_grounded(controller.piece) {
                    placements.push(Placement {
                        piece: *controller.piece,
                        trivial_base: Rc::clone(trivial),
                        nontrivial_extension: Rc::clone(nontrivial),
                        nontrivial_index: i + 1,
                        game: Rc::clone(&game),
                        pieces: Rc::clone(&pieces),
                    });
                }
            }
            controller.reset();
        }
        placements
    }
}
