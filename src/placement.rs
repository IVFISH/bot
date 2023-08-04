#![allow(dead_code)]

use crate::command::Command;
use crate::controller::Controller;
use crate::constants::board_constants::*;
use crate::game::Game;
use crate::piece::Piece;
use itertools::chain;
use std::iter::zip;
use std::rc::Rc;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use fumen;

pub struct Placement {
    // the last piece in the move sequence
    pub piece: Piece,
    pub held: bool,

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

    /// returns the fumen string that represents the
    /// series of pieces that the placement stores
    pub fn get_fumen(&self) -> String {
        fn to_fumen_piece(piece: Piece) -> fumen::Piece {
            let rotation = match piece.dir {
                0 => fumen::RotationState::North,
                1 => fumen::RotationState::East,
                2 => fumen::RotationState::South,
                _ => fumen::RotationState::West,
            };

            let kind = match piece.r#type {
                0 => fumen::PieceType::Z,
                1 => fumen::PieceType::L,
                2 => fumen::PieceType::O,
                3 => fumen::PieceType::S,
                4 => fumen::PieceType::I,
                5 => fumen::PieceType::J,
                _ => fumen::PieceType::T,
            };

            fumen::Piece {
                x: piece.col as u32,
                y: piece.row as u32,
                kind,
                rotation,
            }
        }

        fn to_fumen(game: Game) -> fumen::Fumen {
            let mut fumen = fumen::Fumen::default();
            let page = fumen.add_page();
            for row in (0..VISIBLE_BOARD_HEIGHT).rev() {
                for col in 0..BOARD_WIDTH {
                    if game.board.get(row, col) {
                        page.field[row][col] = fumen::CellColor::Grey;

                    }
                }
            }
            fumen
        }

        let mut f = to_fumen(*self.game);
        for piece in self.pieces.iter() {
            f.add_page().piece = Some(to_fumen_piece(*piece));
        }
        f.add_page().piece = Some(to_fumen_piece(self.piece));
        f.encode()
    }
}

#[derive(Default)]
pub struct PlacementList {
    pub placements: Vec<Placement>,
    pub trivials: Vec<Rc<Vec<Command>>>,
    pub nontrivials: Vec<Rc<Vec<Command>>>,
}

impl PlacementList {
    /// creates a vector of placements
    /// from the trivials and non-trivials
    /// with piece validity checking
    pub fn get_placements(
        trivials: &Vec<Rc<Vec<Command>>>,
        nontrivials: &Vec<Rc<Vec<Command>>>,
        game: Rc<Game>,
        pieces: Rc<Vec<Piece>>,
        held: bool,
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
                        held,
                        game: Rc::clone(&game),
                        pieces: Rc::clone(&pieces),
                    });
                }
            }
            controller.reset();
        }
        placements
    }

    /// debugging tool to write all the fumens to a json file
    pub fn write_fumens(&self, filename: &str) {
        let fumens = self.placements.iter().map(|p| p.get_fumen()).collect::<Vec<_>>().join("\n");
        let path = Path::new(filename);
        let mut file = File::create(&path).unwrap();
        let _ = file.write_all(fumens.as_bytes());
    }

    // plan to make a visualizer for the fumens:
    // options:
    // - play sequential [final=false] (plays all the gifs in order, only show final)
    // - play manual (opens keyboard listener to iterate frames based on left/right keys)
    // - play n (plays the nth fumen on repeat)
    // - filter n [k=1] (only keep the fumens that start with the first k pieces of nth fumen
    // - jump n (jumps to the nth fumen (in filtered list), continues playing if playing)
    // - restore (remove all filters)
    // - stop (stops playing)
    // display:
    // - how many fumens were generated
    // - which fumen index currently on
    // - the current game
}
