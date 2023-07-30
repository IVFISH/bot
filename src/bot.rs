#![allow(dead_code)]

use crate::board::Board;
use crate::command::{Command, COMMANDS};
use crate::constants::board_constants::*;
use crate::constants::bot_constants::*;
use crate::constants::piece_constants::*;
use crate::controller::Controller;
use crate::game::Game;
use crate::piece::Piece;
use crate::placement::{Placement, PlacementList};
use std::collections::HashSet;
use std::iter::zip;

#[derive(Debug, Default)]
pub struct Bot {
    game: Game,
}

impl Bot {
    // constructors -----------------------------
    pub fn new() -> Self {
        Default::default()
    }

    // move generation --------------------------
    /// the API function for generating all current moves
    /// for the current active piece, as well as after holding
    pub fn move_gen(&self) -> PlacementList {
        let mut controller = Controller::new();
        let trivials = self.trivial(&mut controller);
        let nontrivials = self.nontrivial(&trivials, &mut controller);

        let mut out = PlacementList::new(trivials, nontrivials);
        // out.fill_placements(controller, self.game.active, &self.game.board);
        out
    }

    /// return the trivial placements as a vector of vec commands from the starting state
    fn trivial(&self, controller: &mut Controller) -> Vec<Vec<Command>> {
        let mut out = Vec::new();
        let mut piece = self.game.active;
        for rotation in 0..NUM_ROTATE_STATES {
            let mut rep = 0;
            controller.do_command(
                Command::Rotate(rotation as u8),
                &mut piece,
                &self.game.board,
                true,
            );
            while controller.do_command(
                Command::MoveHorizontal(1),
                &mut piece,
                &self.game.board,
                false,
            ) {
                out.push(vec![
                    Command::Rotate(rotation as u8),
                    Command::MoveHorizontal(rep),
                    Command::MoveDrop,
                ]);
                rep += 1;
            }
            piece = self.game.active; // reset the piece
            rep = 0; // reset the repetitions counter
            while controller.do_command(
                Command::MoveHorizontal(-1),
                &mut piece,
                &self.game.board,
                false,
            ) {
                out.push(vec![
                    Command::Rotate(rotation as u8),
                    Command::MoveHorizontal(-rep),
                    Command::MoveDrop,
                ]);
                rep += 1;
            }
            controller.undo(&mut piece);
        }
        out
    }

    /// extend the trivial placements by recursing through inputs that bring
    /// pieces to unseen states. this returns the list of new inputs that
    /// leads to the current state
    fn nontrivial(
        &self,
        trivials: &Vec<Vec<Command>>,
        controller: &mut Controller,
    ) -> Vec<Vec<Command>> {
        let mut seen = HashSet::new();
        let mut out = Vec::new();

        for trivial in trivials.iter() {
            let mut piece = self.game.active;
            controller.do_commands(trivial, &mut piece, &self.game.board, false);
            out.push(self.nontrivial_(controller, &mut seen, piece, &self.game.board));
        }
        out
    }

    /// helper method for [`bot.nontrivial`]
    /// extends a single trivial placeement by recursing through inputs
    /// precondition: the piece is at the location led to by the trivial
    fn nontrivial_(
        &self,
        controller: &mut Controller,
        seen: &mut HashSet<Piece>,
        piece: Piece,
        board: &Board,
    ) -> Vec<Command> {
        let mut dfs_stack = vec![piece];
        let mut out_stack = vec![Command::Null];
        let mut out = Vec::new();

        while let Some(mut p) = dfs_stack.pop() {
            // push the backtrack commands
            let mut backtrack_counter = 0;
            while let Some(Command::Backtrack(c)) = out_stack.last() {
                backtrack_counter += c;
            }
            if backtrack_counter != 0 {
                out.push(Command::Backtrack(backtrack_counter));
            }

            // push the current command
            out.push(*out_stack.last().unwrap());

            // dfs (add to stack)
            for command in COMMANDS.into_iter() {
                controller.do_command(command, &mut p, board, false);
                if seen.contains(&p) {
                    continue;
                }
                seen.insert(p);
                dfs_stack.push(p);
                out_stack.push(command);
            }
        }

        out
    }
}
