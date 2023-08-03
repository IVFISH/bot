#![allow(dead_code)]

use crate::command::{Command, COMMANDS};
use crate::constants::piece_constants::*;
use crate::controller::Controller;
use crate::game::Game;
use crate::piece::Piece;
use crate::placement::PlacementList;
use std::collections::HashSet;

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
        let mut seen = HashSet::new();
        let (trivials, trivial_pieces) = Self::trivial(&mut seen, &mut controller);
        let nontrivials = Self::nontrivial(&mut controller, &mut seen, trivial_pieces);

        PlacementList::new(trivials, nontrivials, controller)
    }

    /// return the trivial placements as a vector of vec commands from the starting state
    fn trivial(
        seen: &mut HashSet<Piece>,
        controller: &mut Controller,
    ) -> (Vec<Vec<Command>>, Vec<Piece>) {

        let mut out = Vec::new();
        let mut out_piece = Vec::new();

        for rotation in 0..NUM_ROTATE_STATES {
            let mut rep = 1;
            controller.do_command_mut(Command::Rotate(rotation as u8));
            Self::add_dropped_piece(controller, seen, &mut out_piece);
            out.push(vec![Command::Rotate(rotation as u8), Command::MoveDrop]);

            while controller.do_command(&Command::MoveHorizontal(1)) {
                Self::add_dropped_piece(controller, seen, &mut out_piece);
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
                Self::add_dropped_piece(controller, seen, &mut out_piece);
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

    fn add_dropped_piece(
        controller: &mut Controller,
        seen: &mut HashSet<Piece>,
        pieces: &mut Vec<Piece>,
    ) {
        let start_row = controller.piece.row;
        controller.do_command(&Command::MoveDrop);
        seen.insert(*controller.piece);
        pieces.push(*controller.piece);
        controller.piece.set_row(start_row)
    }

    /// extend the trivial placements by recursing through inputs that bring
    /// pieces to unseen states. this returns the list of new inputs that
    /// leads to the current state
    fn nontrivial(
        controller: &mut Controller,
        seen: &mut HashSet<Piece>,
        pieces: Vec<Piece>,
    ) -> Vec<Vec<Command>> {
        let mut out = Vec::new();

        for piece in pieces.into_iter() {
            controller.update_piece(piece);
            out.push(Self::nontrivial_(controller, seen));
        }
        controller.reset();
        out
    }

    /// helper method for [`bot.nontrivial`]
    /// extends a single trivial placeement by recursing through inputs
    /// precondition: the piece is at the location led to by the trivial
    /// this leaves the location of the piece at its final recurse
    fn nontrivial_(controller: &mut Controller, seen: &mut HashSet<Piece>) -> Vec<Command> {
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

        let placements = bot.move_gen();
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

        let placements = bot.move_gen();
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
        assert_placement_contains(&placements, piece);
    }
}
