#![allow(dead_code)]

use crate::command::{Command, COMMANDS};
use crate::constants::piece_constants::*;
use crate::controller::Controller;
use crate::game::Game;
use crate::piece::Piece;
use crate::placement::*;
use itertools::concat;
use std::collections::HashSet;
use std::rc::Rc;

#[derive(Debug, Default)]
pub struct Bot {
    pub game: Game,
}

impl Bot {
    // constructors -----------------------------
    pub fn new() -> Self {
        Self {
            game: Game::random()
        }
    }

    // move generation --------------------------
    /// the API function for generating all current moves
    /// for the current active piece, as well as after holding
    pub fn move_gen(&self, depth: usize) -> PlacementList {
        let mut placements = self.move_gen_one_depth();
        for _ in 1..depth {
            placements = Self::iterate_move_gen(placements);
        }
        placements
    }

    /// the one-depth version of move-gen
    fn move_gen_one_depth(&self) -> PlacementList {
        let mut piece = self.game.active; // copy
        let mut controller = Controller::new(&mut piece, &self.game.board);

        // find all the command sequences
        let mut seen = HashSet::new();
        let (trivials, pieces) = self.trivial(&mut controller, &mut seen);
        let nontrivials = self.nontrivial(&mut controller, &mut seen, pieces);

        // generates the placement list
        let game = Rc::new(self.game);
        let pieces = Rc::new(Vec::new());
        let trivials = trivials.into_iter().map(|p| Rc::new(p)).collect();
        let nontrivials = nontrivials.into_iter().map(|p| Rc::new(p)).collect();
        let placements = PlacementList::get_placements(&trivials, &nontrivials, game, pieces);
        PlacementList {
            placements,
            trivials,
            nontrivials,
        }
    }

    /// helper method for move gen that takes in a placementlist
    /// of generated placements (depth i) and returns a new
    /// placement list of depth i+1
    fn iterate_move_gen(placements: PlacementList) -> PlacementList {
        let new_placements = concat(
            placements
                .placements
                .iter()
                .map(|p| Self::extend_placement(p)),
        );

        PlacementList {
            placements: new_placements,
            trivials: placements.trivials,       // move
            nontrivials: placements.nontrivials, // move
        }
    }

    /// helper method 2 for movegen
    /// given a starting placement (of depth i), returns a new list of placements
    /// of depth i+1
    fn extend_placement(placement: &Placement) -> Vec<Placement> {
        // get the starting position to extend placements from
        let mut game = *placement.game; // copy
        let mut pieces = (*placement.pieces).clone();
        pieces.push(placement.piece);
        for piece in pieces.iter() {
            game.update_active(*piece);
            assert!(game.active == *piece); // just a sanity check <3
            game.place_active();
        }

        let mut piece = game.active; // copy
        let mut controller = Controller::new(&mut piece, &game.board);

        // find all the new pieces
        let mut seen = HashSet::new();
        Self::add_trivials(&mut seen, &mut controller);
        Self::add_nontrivials(&mut seen, &mut controller);

        // generate the new placements here
        let mut out = Vec::new();
        let pieces = Rc::new(pieces);
        for piece in seen.into_iter() {
            out.push(Placement {
                piece,
                game: Rc::clone(&placement.game),
                pieces: Rc::clone(&pieces),
                trivial_base: Rc::clone(&placement.trivial_base),
                nontrivial_extension: Rc::clone(&placement.nontrivial_extension),
                nontrivial_index: placement.nontrivial_index,
            });
        }
        out
    }

    /// return the trivial placements as a vector of vec commands from the starting state
    fn trivial(
        &self,
        controller: &mut Controller,
        seen: &mut HashSet<Piece>,
    ) -> (Vec<Vec<Command>>, Vec<Piece>) {
        let mut out = Vec::new();
        let mut out_piece = Vec::new();
        for rotation in 0..NUM_ROTATE_STATES {
            let mut rep = 1;
            controller.do_command_mut(Command::Rotate(rotation as u8));
            let p = Self::get_dropped_piece(controller);
            seen.insert(p);
            out_piece.push(p);
            out.push(vec![Command::Rotate(rotation as u8), Command::MoveDrop]);
            while controller.do_command(&Command::MoveHorizontal(1)) {
                let p = Self::get_dropped_piece(controller);
                seen.insert(p);
                out_piece.push(p);
                out.push(vec![
                    Command::Rotate(rotation as u8),
                    Command::MoveHorizontal(rep),
                    Command::MoveDrop,
                ]);
                rep += 1;
            }
            *controller.piece = controller.peek().unwrap().1; // reset the piece
            rep = 1; // reset the repetitions counter
            while controller.do_command(&Command::MoveHorizontal(-1)) {
                let p = Self::get_dropped_piece(controller);
                seen.insert(p);
                out_piece.push(p);
                out.push(vec![
                    Command::Rotate(rotation as u8),
                    Command::MoveHorizontal(-rep),
                    Command::MoveDrop,
                ]);
                rep += 1;
            }
            controller.undo();

            if controller.piece.r#type == PIECE_O {
                // don't generate new trivials for O
                break;
            }
        }
        (out, out_piece)
    }

    fn get_dropped_piece(controller: &mut Controller) -> Piece {
        let cp = *controller.piece;
        controller.do_command(&Command::MoveDrop);
        let out = *controller.piece;
        controller.update_piece(cp);
        out
    }

    /// extends the seen hashset by the trivials
    fn add_trivials(seen: &mut HashSet<Piece>, controller: &mut Controller) {
        for rotation in 0..NUM_ROTATE_STATES {
            controller.do_command_mut(Command::Rotate(rotation as u8));
            seen.insert(Self::get_dropped_piece(controller));
            while controller.do_command(&Command::MoveHorizontal(1)) {
                seen.insert(Self::get_dropped_piece(controller));
            }
            *controller.piece = controller.peek().unwrap().1; // reset the piece
            while controller.do_command(&Command::MoveHorizontal(-1)) {
                seen.insert(Self::get_dropped_piece(controller));
            }
            controller.undo();

            if controller.piece.r#type == PIECE_O {
                // don't generate new trivials for O
                break;
            }
        }
    }

    /// extend the trivial placements by recursing through inputs that bring
    /// pieces to unseen states. this returns the list of new inputs that
    /// leads to the current state
    fn nontrivial(
        &self,
        controller: &mut Controller,
        seen: &mut HashSet<Piece>,
        pieces: Vec<Piece>,
    ) -> Vec<Vec<Command>> {
        let mut out = Vec::new();

        for piece in pieces.into_iter() {
            controller.update_piece(piece);
            out.push(self.nontrivial_(controller, seen));
        }
        controller.reset();
        out
    }

    /// helper method for [`bot.nontrivial`]
    /// extends a single trivial placeement by recursing through inputs
    /// precondition: the piece is at the location led to by the trivial
    /// this leaves the location of the piece at its final recurse
    fn nontrivial_(&self, controller: &mut Controller, seen: &mut HashSet<Piece>) -> Vec<Command> {
        let mut out = Vec::new();

        let mut dfs_stack = vec![*controller.piece];
        let mut out_stack = vec![Command::Null];

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
        out
    }

    /// extends the seen hashset by the grounded nontrivials
    fn add_nontrivials(seen: &mut HashSet<Piece>, controller: &mut Controller) {
        let mut dfs_stack: Vec<_> = seen.clone().into_iter().collect();
        let mut seen_all = seen.clone(); // includes the not-grounded ones
        while let Some(p) = dfs_stack.pop() {
            for command in COMMANDS.into_iter() {
                controller.update_piece(p);
                controller.do_command(&command);
                if seen_all.contains(controller.piece) {
                    continue;
                }
                seen_all.insert(*controller.piece);
                if controller.board.piece_grounded(controller.piece) {
                    seen.insert(*controller.piece);
                    dfs_stack.push(*controller.piece);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_api::functions::*;

    // #[test]
    fn test_tucks_t() {
        let mut bot = Bot::new();
        let b = &mut bot.game.board;
        add_list(b, vec![[2, 7], [2, 8], [2, 9], [2, 0], [2, 1], [2, 2]]);
        bot.game.active = Piece::new(PIECE_T);

        let placements = bot.move_gen(1);
        assert_eq!(placements.trivials.len(), 34);
        assert_eq!(placements.nontrivials.len(), 34);
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

        let placements = bot.move_gen(1);
        assert_eq!(placements.trivials.len(), 9);
        assert_eq!(placements.nontrivials.len(), 9);
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
        let placements = bot.move_gen(1);
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
        let placements = bot.move_gen(1);
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
        let placements = bot.move_gen(1);
        let piece = Piece {
            r#type: PIECE_L,
            dir: 1,
            row: 1,
            col: 1,
        };
        assert_placement_contains(&placements, piece);
    }
}
