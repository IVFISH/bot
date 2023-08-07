#![allow(dead_code)]

use crate::command::{Command, COMMANDS};
use crate::constants::piece_constants::*;
use crate::controller::Controller;
use crate::game::Game;
use crate::piece::Piece;
use crate::placement::*;
use crate::placement_list::*;
use crate::pruner::*;
use itertools::concat;
use rayon::prelude::*;
use std::collections::HashSet;

#[derive(Debug, Copy, Clone)]
pub struct Bot<P: Pruner> {
    pub game: Game,
    pub pruner: P,
}

impl<P: Pruner> Bot<P> {
    // constructors -----------------------------
    pub fn new() -> Self {
        Self {
            game: Game::random(),
            pruner: P::new(),
        }
    }

    pub fn with_seed(seed: usize) -> Self {
        Self {
            game: Game::new(seed),
            pruner: P::new(),
        }
    }

    // move generation --------------------------
    /// the API function for generating all current moves
    /// for the current active piece, as well as after holding
    pub fn move_gen(&self, depth: usize) -> PlacementList {
        // no hold
        let game = self.game;
        let mut piece = game.active;
        let controller = Controller::new(&mut piece, &self.game.board);
        let mut placements = Self::move_gen_one_depth(controller, false, game, &self.pruner);

        // hold
        let game = *self.game.clone().hold();
        let controller = Controller::new(&mut piece, &self.game.board);
        placements.extend(Self::move_gen_one_depth(controller, true, game, &self.pruner));

        for d in 1..depth {
            // println!("Finished with depth {}.", d);
            placements = Self::iterate_move_gen(placements, &self.pruner);
        }
        // println!("Finished with depth {}.", depth);
        placements
    }

    /// the one-depth version of move-gen
    fn move_gen_one_depth(
        mut controller: Controller,
        held: bool,
        game_before: Game,
        pruner: &P
    ) -> PlacementList {
        // find all the new pieces
        let mut seen = HashSet::new();
        Self::add_trivials(&mut seen, &mut controller);
        Self::add_nontrivials(&mut seen, &mut controller);

        // generate the new placements here
        let placements = seen
            .into_iter()
            .map(|piece| Self::make_placement(piece, held, game_before))
            .collect();
        PlacementList::new(placements, pruner)
    }

    /// helper method for move gen that takes in a placementlist
    /// of generated placements (depth i) and returns a new
    /// placement list of depth i+1 (with and without hold)
    fn iterate_move_gen(placements: PlacementList, pruner: &P) -> PlacementList {
        // to make this parallel: change iter to par_iter (but this might make slow cause each
        // operation is in the us range
        let placements = placements
            .placements
            .iter()
            .flat_map(|p| Self::extend_placement(p))
            .collect();
        // concat(placements.placements.iter().map(|p| Self::extend_placement(p)))
        PlacementList::new(placements, pruner)
    }

    /// helper method 2 for movegen
    /// given a starting placement (of depth i), returns a new list of placements
    /// of depth i+1 (with and without hold)
    fn extend_placement(placement: &Placement) -> Vec<Placement> {
        // get the starting position to extend placements from
        let game_before = placement.game_after; // copy
        let mut piece = game_before.active; // copy
        let mut controller = Controller::new(&mut piece, &game_before.board);
        // find all the new pieces
        let mut seen = HashSet::new();
        Self::add_trivials(&mut seen, &mut controller);
        Self::add_nontrivials(&mut seen, &mut controller);

        // generate the new placements here
        let mut out: Vec<_> = seen
            .into_iter()
            .map(|piece| Self::make_placement(piece, false, game_before))
            .collect();

        // get the starting position to extend placements from
        let game_before = *placement.game_after.clone().hold();
        let mut piece = game_before.active; // copy
        let mut controller = Controller::new(&mut piece, &game_before.board);
        // find all the new pieces
        let mut seen = HashSet::new();
        Self::add_trivials(&mut seen, &mut controller);
        Self::add_nontrivials(&mut seen, &mut controller);

        // generate the new placements here
        out.extend(
            seen.into_iter()
                .map(|piece| Self::make_placement(piece, true, game_before))
                .collect::<Vec<_>>(),
        );
        out
    }

    fn make_placement(piece: Piece, held: bool, game_before: Game) -> Placement {
        Placement {
            piece,
            held,
            game_before,
            game_after: *game_before
                .clone()
                .set_active(piece, held)
                .place_active(held),
        }
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
            if !controller.do_command_mut(Command::Rotate(rotation as u8)) {
                continue;
            }
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

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::test_api::functions::*;
//
//     // #[test]
//     fn test_tucks_t() {
//         let mut bot = Bot::new();
//         let b = &mut bot.game.board;
//         add_list(b, vec![[2, 7], [2, 8], [2, 9], [2, 0], [2, 1], [2, 2]]);
//         bot.game.active = Piece::new(PIECE_T);
//
//         let placements = bot.move_gen(1);
//         assert_eq!(placements.trivials.len(), 34);
//         assert_eq!(placements.nontrivials.len(), 34);
//         assert_eq!(placements.placements.len(), 48);
//
//         // checking for any duplicate pieces
//         let pieces: HashSet<_> = placements.placements.into_iter().map(|p| p.piece).collect();
//         assert_eq!(pieces.len(), 48);
//         // checking for any invalid pieces
//         let b = &mut bot.game.board;
//         assert!(pieces.iter().all(|piece| b.piece_can_set(piece)));
//     }
//
//     #[test]
//     fn test_tucks_o() {
//         let mut bot = Bot::new();
//         let b = &mut bot.game.board;
//         add_list(b, vec![[2, 7], [2, 8], [2, 9], [2, 0], [2, 1], [2, 2]]);
//         bot.game.active = Piece::new(PIECE_O);
//
//         let placements = bot.move_gen(1);
//         assert_eq!(placements.trivials.len(), 9);
//         assert_eq!(placements.nontrivials.len(), 9);
//         assert_eq!(placements.placements.len(), 15);
//
//         // checking for any duplicate pieces
//         let pieces: HashSet<_> = placements.placements.into_iter().map(|p| p.piece).collect();
//         assert_eq!(pieces.len(), 15);
//         // checking for any invalid pieces
//         let b = &mut bot.game.board;
//         assert!(pieces.iter().all(|piece| b.piece_can_set(piece)));
//     }
//
//     #[test]
//     fn test_z_spin() {
//         let mut bot = Bot::new();
//         bot.game.board = z_spin_board_1();
//         bot.game.active = Piece::new(PIECE_Z);
//         let placements = bot.move_gen(1);
//         let piece = Piece {
//             r#type: PIECE_Z,
//             dir: 2,
//             row: 1,
//             col: 4,
//         };
//         assert_placement_contains(&placements, piece);
//     }
//
//     #[test]
//     fn test_tst_spin() {
//         let mut bot = Bot::new();
//         bot.game.board = tst_board();
//         bot.game.active = Piece::new(PIECE_T);
//         let placements = bot.move_gen(1);
//         let piece = Piece {
//             r#type: PIECE_T,
//             dir: 3,
//             row: 1,
//             col: 3,
//         };
//         assert_placement_contains(&placements, piece);
//     }
//
//     #[test]
//     fn test_l_spin() {
//         let mut bot = Bot::new();
//         bot.game.board = l_spin_board_5();
//         bot.game.active = Piece::new(PIECE_L);
//         let placements = bot.move_gen(1);
//         let piece = Piece {
//             r#type: PIECE_L,
//             dir: 1,
//             row: 1,
//             col: 1,
//         };
//         assert_placement_contains(&placements, piece);
//     }
// }
