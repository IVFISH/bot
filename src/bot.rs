#![allow(dead_code)]

use crate::board::Board;
use crate::command::{Command, COMMANDS};
use crate::constants::piece_constants::*;
use crate::controller::Controller;
use crate::game::Game;
use crate::piece::Piece;
use crate::placement::PlacementList;
use std::collections::HashSet;

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

        PlacementList::new(
            trivials,
            nontrivials,
            controller,
            self.game.active,
            &self.game.board,
        )
    }

    /// return the trivial placements as a vector of vec commands from the starting state
    fn trivial(&self, controller: &mut Controller) -> Vec<Vec<Command>> {
        let mut out = Vec::new();
        let mut piece = self.game.active;
        for rotation in 0..NUM_ROTATE_STATES {
            let mut rep = 0;
            controller.do_command_mut(
                Command::Rotate(rotation as u8),
                &mut piece,
                &self.game.board,
            );
            while controller.do_command(&Command::MoveHorizontal(1), &mut piece, &self.game.board) {
                out.push(vec![
                    Command::Rotate(rotation as u8),
                    Command::MoveHorizontal(rep),
                    Command::MoveDrop,
                ]);
                rep += 1;
            }
            piece = self.game.active; // reset the piece
            rep = 0; // reset the repetitions counter
            while controller.do_command(&Command::MoveHorizontal(-1), &mut piece, &self.game.board)
            {
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
            controller.do_commands(trivial, &mut piece, &self.game.board);
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
                controller.do_command(&command, &mut p, board);
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
    use crate::controller::tests::*;

    fn add_list(board: &mut Board, list: Vec<[usize; 2]>) {
        for [r, c] in list.into_iter() {
            board.set(r, c, 1);
        }
    }

    fn assert_placement_contains(placements: &PlacementList, piece: Piece) {
        assert!(placements.placements.iter().any(|p| p.piece == piece));
    }

    #[test]
    fn test_tucks() {
        let mut bot = Bot::new();
        let b = &mut bot.game.board;
        add_list(b, vec![[1, 7], [1, 8], [1, 9], [1, 0], [1, 1], [1, 2]]);
        bot.game.active = Piece::new(PIECE_O);

        let placements = bot.move_gen();
        assert_eq!(placements.trivials.len(), 9);
        assert_eq!(placements.nontrivials.len(), 9);
        assert_eq!(placements.placements.len(), 21);

        // checking for any duplicate pieces
        let mut pieces: Vec<_> = placements.placements.into_iter().map(|p| p.piece).collect();
        pieces.sort_unstable();
        pieces.dedup();
        assert_eq!(pieces.len(), 21);
    }

    #[test]
    fn test_z_spin() {
        let mut bot = Bot::new();
        bot.game.board = z_spin_board_1();
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
        let placements = bot.move_gen();
        let piece = Piece {
            r#type: PIECE_T,
            dir: 1,
            row: 1,
            col: 3,
        };
        assert_placement_contains(&placements, piece);
    }

    #[test]
    fn test_l_spin() {
        let mut bot = Bot::new();
        bot.game.board = l_spin_board_5();
        let placements = bot.move_gen();
        let piece = Piece {
            r#type: PIECE_L,
            dir: 1,
            row: 0,
            col: 1,
        };
        assert_placement_contains(&placements, piece);
    }
}
