#![allow(dead_code)]

use crate::command::{Command, COMMANDS};
use crate::constants::piece_constants::*;
use crate::controller::Controller;
use crate::game::Game;
use crate::piece::Piece;
use crate::placement::*;
use std::collections::HashSet;
use std::rc::Rc;

#[derive(Debug, Default)]
pub struct Bot {
    pub game: Game,
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
        let mut piece = self.game.active; // this piece is moved around to generate moves
        let mut controller = Controller::new(&mut piece, &self.game.board);
        let mut placements = self.trivial(&mut controller);
        self.nontrivial(&mut placements, &mut controller);
        placements
    }

    /// return the trivial placements as a vector of vec commands from the starting state
    fn trivial(&self, controller: &mut Controller) -> PlacementList {
        let mut out = PlacementList::default();
        for rotation in 0..NUM_ROTATE_STATES {
            let mut rep = 1;
            controller.do_command_mut(Command::Rotate(rotation as u8));
            out.trivials.push(Rc::new(vec![Command::Rotate(rotation as u8), Command::MoveDrop]));
            out.placements.push(Placement {
                piece: *controller.piece, 
                trivial_base: Rc::clone(out.trivials.last().unwrap()),
                nontrivial_extension: Rc::new(Vec::new()),
                nontrivial_index: 0
            });
            while controller.do_command(&Command::MoveHorizontal(1)) {
                out.trivials.push(Rc::new(vec![
                    Command::Rotate(rotation as u8),
                    Command::MoveHorizontal(rep),
                    Command::MoveDrop,
                ]));
                out.placements.push(Placement {
                    piece: *controller.piece, 
                    trivial_base: Rc::clone(out.trivials.last().unwrap()),
                    nontrivial_extension: Rc::new(Vec::new()),
                    nontrivial_index: 0
                });
                rep += 1;
            }
            *controller.piece = controller.peek().unwrap().1; // reset the piece
            rep = 1; // reset the repetitions counter
            while controller.do_command(&Command::MoveHorizontal(-1)) {
                out.trivials.push(Rc::new(vec![
                    Command::Rotate(rotation as u8),
                    Command::MoveHorizontal(-rep),
                    Command::MoveDrop,
                ]));
                out.placements.push(Placement {
                    piece: *controller.piece, 
                    trivial_base: Rc::clone(out.trivials.last().unwrap()),
                    nontrivial_extension: Rc::new(Vec::new()),
                    nontrivial_index: 0
                });
                rep += 1;
            }
            controller.undo();

            if controller.piece.r#type == PIECE_O {
                // don't generate new trivials for O
                break;
            }
        }
        out
    }

    /// extend the trivial placements by recursing through inputs that bring
    /// pieces to unseen states. this returns the list of new inputs that
    /// leads to the current state
    fn nontrivial(
        &self,
        placements: &mut PlacementList,
        controller: &mut Controller,
    )  {
        let mut seen = HashSet::new();

        let trivials = placements.trivials.clone();
        for (i, trivial) in trivials.iter().enumerate() {
            controller.do_commands(trivial);
            self.nontrivial_(placements, controller, &mut seen, i);
            controller.reset();
        }
    }

    /// helper method for [`bot.nontrivial`]
    /// extends a single trivial placeement by recursing through inputs
    /// precondition: the piece is at the location led to by the trivial
    /// this leaves the location of the piece at its final recurse
    fn nontrivial_(&self, placements: &mut PlacementList,
                   controller: &mut Controller, seen: &mut HashSet<Piece>, i: usize) {
        if seen.contains(controller.piece) {
            return;
        }
        seen.insert(*controller.piece);

        let mut dfs_stack = vec![*controller.piece];
        let mut out_stack = vec![Command::Null];
        let mut out = Vec::new();
        let mut data = Vec::new(); // piece and their nontrivial_index

        while !dfs_stack.is_empty() {
            // push the backtrack commands
            let mut backtrack_counter = 0;
            while let Some(Command::Backtrack(c)) = out_stack.last() {
                backtrack_counter += c;
                out_stack.pop();
            }
            if backtrack_counter != 0 {
                out.push(Command::Backtrack(backtrack_counter));
            }

            // push the current command
            out.push(out_stack.pop().unwrap());
            data.push((*controller.piece, out.len()));

            // dfs (add to stack)
            let p = dfs_stack.pop().unwrap();
            out_stack.push(Command::Backtrack(1));
            for command in COMMANDS.into_iter() {
                // update the controller to use this new piece
                controller.update_piece(p);
                controller.do_command(&command);
                let p = *controller.piece;
                if seen.contains(&p) {
                    continue;
                }
                seen.insert(p);
                dfs_stack.push(p);
                out_stack.push(command);
            }
        }
        let out = Rc::new(out);
        let trivial_base = Rc::clone(&placements.trivials[i]);
        for (piece, nontrivial_index) in data.into_iter() {
            if controller.board.piece_grounded(&piece) {
                placements.placements.push(Placement {
                    piece,
                    trivial_base: Rc::clone(&trivial_base),
                    nontrivial_extension: Rc::clone(&out),
                    nontrivial_index
                });
            }

        }
        placements.nontrivials.push(out);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_api::functions::*;

    #[test]
    fn test_tucks_t() {
        let mut bot = Bot::new();
        let b = &mut bot.game.board;
        add_list(b, vec![[2, 7], [2, 8], [2, 9], [2, 0], [2, 1], [2, 2]]);
        bot.game.active = Piece::new(PIECE_T);

        let placements = bot.move_gen();
        assert_eq!(placements.trivials.len(), 34);
        // assert_eq!(placements.nontrivials.len(), 34);
        assert_eq!(placements.placements.len(), 48);

        // checking for any duplicate pieces
        let pieces: HashSet<_> = placements.placements.into_iter().map(|p| p.piece).collect();
        assert_eq!(pieces.len(), 48);
        // checking for any invalid pieces
        let b = &mut bot.game.board;
        assert!(pieces.iter().all(|piece| b.piece_can_set(piece)));
    }

    #[test]
    fn test_tucks_o() {
        let mut bot = Bot::new();
        let b = &mut bot.game.board;
        add_list(b, vec![[2, 7], [2, 8], [2, 9], [2, 0], [2, 1], [2, 2]]);
        bot.game.active = Piece::new(PIECE_O);

        let placements = bot.move_gen();
        assert_eq!(placements.trivials.len(), 9);
        // assert_eq!(placements.nontrivials.len(), 9);
        assert_eq!(placements.placements.len(), 15);

        // checking for any duplicate pieces
        let pieces: HashSet<_> = placements.placements.into_iter().map(|p| p.piece).collect();
        assert_eq!(pieces.len(), 15);
        // checking for any invalid pieces
        let b = &mut bot.game.board;
        assert!(pieces.iter().all(|piece| b.piece_can_set(piece)));
    }

    #[test]
    fn test_z_spin() {
        let mut bot = Bot::new();
        bot.game.board = z_spin_board_1();
        bot.game.active = Piece::new(PIECE_Z);
        let placements = bot.move_gen();
        let piece = Piece {
            r#type: PIECE_Z,
            dir: 2,
            row: 1,
            col: 4,
        };
        assert_placement_contains(&placements, piece);
    }

    #[test]
    fn test_tst_spin() {
        let mut bot = Bot::new();
        bot.game.board = tst_board();
        bot.game.active = Piece::new(PIECE_T);
        let placements = bot.move_gen();
        let piece = Piece {
            r#type: PIECE_T,
            dir: 3,
            row: 1,
            col: 3,
        };
        assert_placement_contains(&placements, piece);
    }

    #[test]
    fn test_l_spin() {
        let mut bot = Bot::new();
        bot.game.board = l_spin_board_5();
        bot.game.active = Piece::new(PIECE_L);
        let placements = bot.move_gen();
        let piece = Piece {
            r#type: PIECE_L,
            dir: 1,
            row: 1,
            col: 1,
        };
        let pieces: Vec<_> = placements.placements.iter().map(|x| x.piece).collect();
        for x in pieces {
            println!("{:?}", x);
        }
        assert_placement_contains(&placements, piece);
    }
}
