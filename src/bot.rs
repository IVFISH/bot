#![allow(dead_code)]

use crate::command::{Command, COMMANDS};
use crate::constants::piece_constants::*;
use crate::controller::Controller;
use crate::game::Game;
use crate::piece::Piece;
use crate::placement::*;
use crate::placement_list::*;
use crate::pruner::*;
use crate::suggestion::*;
use auto_enums::auto_enum;
use rayon::prelude::*;
use rustc_hash::FxHashSet;
use std::iter::once;
use std::sync::Arc;

#[derive(Debug, Copy, Clone)]
pub struct Bot<P: Pruner> {
    pub game: Game,
    pub pruner: P,
}

impl<P: Pruner + std::marker::Sync> Default for Bot<P> {
    fn default() -> Self {
        Self::new()
    }
}

impl<P: Pruner + std::marker::Sync> Bot<P> {
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
    /// the API function for getting the next move
    /// this also updates the bot to whatever it did
    pub fn do_suggest(&mut self) -> Suggestion {
        self.r#do(4);
        Suggestion::new(self.game.board)
    }

    /// the API function for doing the next move
    /// this also updates the bot to whatever it did
    pub fn r#do(&mut self, depth: usize) {
        // todo fix magic numbers

        if let Some(chosen) = self.move_gen(depth).into_placements().find_any(|_| true) {
            let piece_encoding = (chosen.game.history >> (16 * (depth - 1)) & 0xFFFF) as u16;
            let piece = Piece::decode(piece_encoding);
            let held = (piece_encoding >> 14 & 1) != 0;
            // place the piece
            self.game.set_active(piece, held);
            self.game.place_active(held);
        } else {
            panic!("no pieces found")
        }
    }

    /// the API function for generating all current moves of depth
    /// for the current active piece, as well as after holding
    #[auto_enum(rayon::IndexedParallelIterator)]
    pub fn move_gen<'a>(&'a self, depth: usize) -> impl IntoPlacements + 'a {
        if depth == 0 {
            return rayon::iter::empty();
        }

        let one = Self::get_base_placements(&Placement::new(self.game)).into_par_iter();
        if depth == 1 {
            return one;
        }

        let two = Self::iterate_move_gen(one, &self.pruner);
        if depth == 2 {
            return two.into_par_iter();
        }

        let three = Self::iterate_move_gen(two, &self.pruner);
        if depth == 3 {
            return three.into_par_iter();
        }

        let four = Self::iterate_move_gen(three, &self.pruner);
        if depth == 4 {
            return four.into_par_iter();
        }

        let five = Self::iterate_move_gen(four, &self.pruner);
        if depth == 5 {
            return five.into_par_iter();
        }

        panic!("don't look at the code for movegen please (bot.rs:100)");
    }

    /// helper method for move gen that takes in a placementlist
    /// of generated placements (depth i) and returns a new
    /// placement list of depth i+1 (with and without hold)
    fn iterate_move_gen<'a, I>(
        placements: impl IntoPlacements<I = I>,
        pruner: &'a P,
    ) -> impl IntoPlacements + 'a
    where
        I: ParallelIterator<Item = Placement> + 'a,
    {
        placements
            .into_par_iter()
            .flat_map_iter(|p| Self::extend_placement(p, pruner))
    }

    /// helper method 2 for movegen
    /// given a starting placement (of depth i), returns a new list of placements
    /// of depth i+1 (with and without hold)
    /// note that this ruins placement
    fn extend_placement<'a>(placement: Placement, pruner: &'a P) -> Vec<Placement> {
        let mut piece = placement.game.active;
        let mut controller = Controller::new(&mut piece, &placement.game.board);
        let mut seen = Vec::new();

        Self::add_trivials(&mut seen, &mut controller);
        Self::add_nontrivials(&mut seen, &mut controller);

        let placement_cpy = placement.clone();
        let out = seen // turn the pieces into placements
            .into_iter()
            .map(move |piece| Self::make_placement(piece, false, placement_cpy.clone()))
            .filter(|piece| pruner.precondition(piece));

        if placement.game.get_hold_piece().r#type == placement.game.active.r#type {
            return out.collect();
        }

        let mut piece = placement.game.get_hold_piece();
        let mut controller = Controller::new(&mut piece, &placement.game.board);
        let mut seen = Vec::new();

        Self::add_trivials(&mut seen, &mut controller);
        Self::add_nontrivials(&mut seen, &mut controller);

        out.chain(
            // turn the pieces into placements
            seen.into_iter()
                .map(move |piece| Self::make_placement(piece, true, placement.clone()))
                .filter(|piece| pruner.precondition(piece)),
        )
        .collect()
    }

    fn make_placement(piece: Piece, held: bool, mut place_before: Placement) -> Placement {
        place_before.game.set_active(piece, held).place_active(held);
        place_before
    }

    fn get_dropped_piece(controller: &mut Controller) -> Piece {
        let cp = *controller.piece;
        controller.do_command(&Command::MoveDrop);
        let out = *controller.piece;
        controller.update_piece(cp);
        out
    }

    /// extends the seen hashset by the trivials
    /// note: if the max placed height < 16, then there is no need to check for valid placements
    fn add_trivials(seen: &mut Vec<Piece>, controller: &mut Controller) {
        for rotation in 0..NUM_ROTATE_STATES as u8 {
            if !controller.do_command_mut(Command::Rotate(rotation)) {
                continue;
            }
            seen.push(Self::get_dropped_piece(controller));
            while controller.do_command(&Command::MoveHorizontal(1)) {
                seen.push(Self::get_dropped_piece(controller));
            }
            controller.update_piece(controller.peek().unwrap().1); // reset the piece
            while controller.do_command(&Command::MoveHorizontal(-1)) {
                seen.push(Self::get_dropped_piece(controller));
            }
            controller.undo();

            if controller.piece.r#type == PIECE_O {
                // don't generate new trivials for O
                break;
            }
        }
    }

    /// extends the seen hashset by the grounded nontrivials
    fn add_nontrivials(seen: &mut Vec<Piece>, controller: &mut Controller) {
        let mut dfs_stack: Vec<_> = seen.clone();
        let mut seen_all: FxHashSet<_> = seen.iter().cloned().collect(); // includes the not-grounded ones
        while let Some(p) = dfs_stack.pop() {
            for command in COMMANDS.iter() {
                controller.update_piece(p);
                controller.do_command(command);
                if seen_all.contains(controller.piece) {
                    continue;
                }
                seen_all.insert(*controller.piece);
                dfs_stack.push(*controller.piece);
                if controller.board.piece_grounded(controller.piece) {
                    seen.push(*controller.piece);
                }
            }
        }
    }

    fn get_base_placements(start: &Placement) -> Vec<Placement> {
        let mut piece = start.game.active;
        let controller = &mut Controller::new(&mut piece, &start.game.board);

        let pairs = &mut Self::get_base_trivials(controller);
        Self::get_base_nontrivials(pairs, controller);

        let out = pairs.iter().map(|(p, cmds)| Placement {
            game: Self::make_placement(*p, false, start.clone()).game,
            base_command: Arc::new(cmds.clone()),
        });

        if start.game.get_hold_piece().r#type == start.game.active.r#type {
            return out.collect();
        }

        let mut piece = start.game.get_hold_piece();
        let controller = &mut Controller::new(&mut piece, &start.game.board);

        let pairs = &mut Self::get_base_trivials(controller);
        Self::get_base_nontrivials(pairs, controller);

        out.chain(pairs.iter().map(|(p, cmds)| Placement {
            game: Self::make_placement(*p, true, start.clone()).game,
            base_command: Arc::new(cmds.clone()),
        }))
        .collect()
    }

    fn get_base_trivials(controller: &mut Controller) -> Vec<(Piece, Vec<Command>)> {
        let mut out: Vec<(Piece, Vec<Command>)> = Vec::new();
        for rotation in 0..NUM_ROTATE_STATES {
            if !controller.do_command_mut(Command::Rotate(rotation as u8)) {
                continue;
            }
            let mut commands: Vec<Command> = vec![Command::Rotate(rotation as u8)];
            out.push((
                Self::get_dropped_piece(controller),
                commands
                    .iter()
                    .chain(once(&Command::MoveDrop))
                    .cloned()
                    .collect(),
            ));
            while controller.do_command(&Command::MoveHorizontal(1)) {
                commands.push(Command::MoveHorizontal(1));
                out.push((
                    Self::get_dropped_piece(controller),
                    commands
                        .iter()
                        .chain(once(&Command::MoveDrop))
                        .cloned()
                        .collect(),
                ));
            }
            controller.update_piece(controller.peek().unwrap().1); // reset the piece
            let mut commands: Vec<Command> = vec![Command::Rotate(rotation as u8)];
            while controller.do_command(&Command::MoveHorizontal(-1)) {
                commands.push(Command::MoveHorizontal(-1));
                out.push((
                    Self::get_dropped_piece(controller),
                    commands
                        .iter()
                        .chain(once(&Command::MoveDrop))
                        .cloned()
                        .collect(),
                ));
            }
            controller.undo();

            if controller.piece.r#type == PIECE_O {
                // don't generate new trivials for O
                break;
            }
        }
        out
    }

    fn get_base_nontrivials(seen: &mut Vec<(Piece, Vec<Command>)>, controller: &mut Controller) {
        let mut dfs_stack: Vec<_> = seen.clone();
        let mut seen_all: FxHashSet<_> = seen.iter().map(|(p, _)| p).cloned().collect(); // includes the not-grounded ones
        while let Some((p, cmd)) = dfs_stack.pop() {
            for command in COMMANDS.iter() {
                controller.update_piece(p);
                controller.do_command(command);
                if seen_all.contains(controller.piece) {
                    continue;
                }
                seen_all.insert(*controller.piece);
                let to_add = (
                    *controller.piece,
                    cmd.iter().chain(once(command)).cloned().collect(),
                );
                dfs_stack.push(to_add.clone());
                if controller.board.piece_grounded(controller.piece) {
                    seen.push(to_add.clone());
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
        let mut bot = Bot::<NoPruner>::new();
        let b = &mut bot.game.board;
        add_list(b, vec![[2, 7], [2, 8], [2, 9], [2, 0], [2, 1], [2, 2]]);
        bot.game.active = Piece::new(PIECE_T);

        // remove all held pieces (not t)
        let pieces: Vec<_> = bot
            .move_gen(1)
            .into_placements()
            .map(|p| p.get_last_piece())
            .filter(|p| p.r#type == PIECE_T)
            .collect();
        assert_eq!(pieces.len(), 48);

        // checking for any duplicate pieces
        let pieces: FxHashSet<_> = pieces.into_iter().collect();
        assert_eq!(pieces.len(), 48);
        // checking for any invalid pieces
        let b = &mut bot.game.board;
        assert!(pieces.iter().all(|piece| b.piece_can_set(piece)));
    }

    #[test]
    fn test_tucks_o() {
        let mut bot = Bot::<NoPruner>::new();
        let b = &mut bot.game.board;
        add_list(b, vec![[2, 7], [2, 8], [2, 9], [2, 0], [2, 1], [2, 2]]);
        bot.game.active = Piece::new(PIECE_O);

        // remove all held pieces (not o)
        let pieces: Vec<_> = bot
            .move_gen(1)
            .into_placements()
            .map(|p| p.get_last_piece())
            .filter(|p| p.r#type == PIECE_O)
            .collect();
        assert_eq!(pieces.len(), 15);

        // checking for any duplicate pieces
        let pieces: FxHashSet<_> = pieces.into_iter().collect();
        assert_eq!(pieces.len(), 15);
        // checking for any invalid pieces
        let b = &mut bot.game.board;
        assert!(pieces.iter().all(|piece| b.piece_can_set(piece)));
    }

    #[test]
    fn test_z_spin() {
        let mut bot = Bot::<NoPruner>::new();
        bot.game.board = z_spin_board_1();
        bot.game.active = Piece::new(PIECE_Z);
        let placements = bot.move_gen(1).into_placements();
        let piece = Piece {
            r#type: PIECE_Z,
            dir: 2,
            row: 1,
            col: 4,
        };
        assert_placement_contains(placements, piece);
    }

    #[test]
    fn test_tst_spin() {
        let mut bot = Bot::<NoPruner>::new();
        bot.game.board = tst_board();
        bot.game.active = Piece::new(PIECE_T);
        let placements = bot.move_gen(1).into_placements();
        let piece = Piece {
            r#type: PIECE_T,
            dir: 3,
            row: 1,
            col: 3,
        };
        assert_placement_contains(placements, piece);
    }

    #[test]
    fn test_l_spin() {
        let mut bot = Bot::<NoPruner>::new();
        bot.game.board = l_spin_board_5();
        println!("{}", bot.game.board);
        bot.game.active = Piece::new(PIECE_L);
        let placements = bot.move_gen(1).into_placements();
        let piece = Piece {
            r#type: PIECE_L,
            dir: 1,
            row: 1,
            col: 1,
        };
        assert_placement_contains(placements, piece);
    }

    #[test]
    fn test_number_placements_generated() {
        // NOTE this is dependent on the queue
        // assumes commit 90232f86f194a8e819f89ea80124da7e01ef9b59 is correct
        let bot = Bot::<NoPruner>::with_seed(4);
        let desired_q = [1, 4, 5, 6];
        assert!(bot.game.active.r#type == 2);
        assert!(desired_q
            .into_iter()
            .enumerate()
            .all(|(i, p)| p == bot.game.queue.peek_ahead(i as u8)));
        assert_eq!(bot.move_gen(3).into_placements().count(), 118_151);

        let bot = Bot::<NoPruner>::with_seed(19);
        let desired_q = [3, 6, 5, 1];
        assert!(bot.game.active.r#type == 4);
        assert!(desired_q
            .into_iter()
            .enumerate()
            .all(|(i, p)| p == bot.game.queue.peek_ahead(i as u8)));
        assert!(bot.move_gen(3).into_placements().count() == 333_078);
        // add some more stuff to test :D
    }
}
