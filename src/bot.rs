#![allow(dead_code)]

use crate::command::{Command, COMMANDS};
use crate::constants::piece_constants::*;
use crate::controller::Controller;
use crate::game::Game;
use crate::piece::Piece;
use crate::placement::*;
use crate::placement_list::*;
use crate::pruner::*;
use crate::evaluator::*;
use crate::suggestion::*;
use rayon::prelude::*;
use rustc_hash::FxHashSet;

#[derive(Debug, Copy, Clone)]
pub struct Bot<P: Pruner, E: Evaluator> {
    pub game: Game,
    pub pruner: P,
    pub evaluator: E
}

impl<P: Pruner + std::marker::Sync, E:Evaluator + std::marker::Sync> Default for Bot<P, E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<P: Pruner + std::marker::Sync, E: Evaluator + std::marker::Sync> Bot<P, E> {
    // constructors -----------------------------
    pub fn new() -> Self {
        Self {
            game: Game::random(),
            pruner: P::new(),
            evaluator: E::new(),
        }
    }

    pub fn with_seed(seed: usize) -> Self {
        Self {
            game: Game::new(seed),
            pruner: P::new(),
            evaluator: E::new(),
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
        // TODO: Figure out why this crashes at depth 1
        let chosen = self.move_gen(depth).placements.first().unwrap().clone();
        let piece = chosen.base_piece;
        let held = piece.r#type != self.game.active.r#type;
        // place the piece
        self.game.set_active(piece, held);
        self.game.place_active();
    }

    /// the API function for generating all current moves of depth
    /// for the current active piece, as well as after holding
    pub fn move_gen(&self, depth: usize) -> PlacementList {
        let start = Placement::new(self.game);

        let mut placements = PlacementList {
            placements: Self::get_base_placements(&start),
        }; // depth 1

        // placements = (1..depth).fold(placements, |p, _| {
        //     Self::iterate_move_gen(p, &self.pruner)
        // });
        for _ in 1..depth {
            placements = Self::iterate_move_gen(placements, &self.pruner);
        }

        placements
    }

    /// helper method for move gen that takes in a placementlist
    /// of generated placements (depth i) and returns a new
    /// placement list of depth i+1 (with and without hold)
    fn iterate_move_gen(placements: PlacementList, pruner: &P) -> PlacementList {
        let placements = placements
            .placements
            .into_par_iter()
            .flat_map_iter(|p| Self::extend_placement(&p, pruner))
            .collect();
        PlacementList::new(placements, pruner)
    }

    /// helper method 2 for movegen
    /// given a starting placement (of depth i), returns a new list of placements
    /// of depth i+1 (with and without hold)
    /// note that this ruins placement
    fn extend_placement(placement: &Placement, pruner: &P) -> Vec<Placement> {
        let mut piece = placement.game.active;
        let mut controller = Controller::new(&mut piece, &placement.game.board);
        let mut seen = Vec::new();

        Self::add_trivials(&mut seen, &mut controller);
        Self::add_nontrivials(&mut seen, &mut controller);

        let out = seen // turn the pieces into placements
            .into_iter()
            .map(|piece| Self::make_placement(piece, false, placement))
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
                .map(|piece| Self::make_placement(piece, true, placement))
                .filter(|piece| pruner.precondition(piece)),
        )
        .collect()
    }

    fn make_placement(piece: Piece, held: bool, place_before: &Placement) -> Placement {
        let mut new_placement = place_before.clone();
        new_placement.game.set_active(piece, held).place_active();
        new_placement.eval = E::eval(&new_placement.game);
        new_placement
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

        let seen = &mut Self::get_base_trivials(controller);
        Self::get_base_nontrivials(seen, controller);

        let out = seen.iter().map(|&p| Placement::new_base(
            Self::make_placement(p, false, start).game,
            p,
        ));

        if start.game.get_hold_piece().r#type == start.game.active.r#type {
            return out.collect();
        }

        let mut piece = start.game.get_hold_piece();
        let controller = &mut Controller::new(&mut piece, &start.game.board);

        let seen = &mut Self::get_base_trivials(controller);
        Self::get_base_nontrivials(seen, controller);

        out.chain(seen.iter().map(|&p| Placement::new_base(
            Self::make_placement(p, true, start).game,
            p,
        )))
        .collect()
    }

    fn get_base_trivials(controller: &mut Controller) -> Vec<Piece> {
        let mut out: Vec<Piece> = Vec::new();
        for rotation in 0..NUM_ROTATE_STATES {
            if !controller.do_command_mut(Command::Rotate(rotation as u8)) {
                continue;
            }
            let mut commands: Vec<Command> = vec![Command::Rotate(rotation as u8)];
            out.push(Self::get_dropped_piece(controller));
            while controller.do_command(&Command::MoveHorizontal(1)) {
                commands.push(Command::MoveHorizontal(1));
                out.push(Self::get_dropped_piece(controller));
            }
            controller.update_piece(controller.peek().unwrap().1); // reset the piece
            let mut commands: Vec<Command> = vec![Command::Rotate(rotation as u8)];
            while controller.do_command(&Command::MoveHorizontal(-1)) {
                commands.push(Command::MoveHorizontal(-1));
                out.push(Self::get_dropped_piece(controller));
            }
            controller.undo();

            if controller.piece.r#type == PIECE_O {
                // don't generate new trivials for O
                break;
            }
        }
        out
    }

    fn get_base_nontrivials(seen: &mut Vec<Piece>, controller: &mut Controller) {
        let mut dfs_stack: Vec<_> = seen.clone();
        let mut seen_all: FxHashSet<_> = seen.iter().copied().collect(); // includes the not-grounded ones
        while let Some(p) = dfs_stack.pop() {
            for command in COMMANDS.iter() {
                controller.update_piece(p);
                controller.do_command(command);
                if seen_all.contains(controller.piece) {
                    continue;
                }
                seen_all.insert(*controller.piece);
                let to_add = *controller.piece;
                dfs_stack.push(to_add);
                if controller.board.piece_grounded(controller.piece) {
                    seen.push(to_add);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_api::functions::*;

    #[test]
    fn test_tucks_t() {
        let mut bot = Bot::<NoPruner, NoEvaluator>::new();
        let b = &mut bot.game.board;
        add_list(b, vec![[2, 7], [2, 8], [2, 9], [2, 0], [2, 1], [2, 2]]);
        bot.game.active = Piece::new(PIECE_T);

        // remove all held pieces (not t)
        let pieces: Vec<_> = bot
            .move_gen(1)
            .placements
            .iter()
            .map(|p| p.base_piece)
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
        let mut bot = Bot::<NoPruner, NoEvaluator>::new();
        let b = &mut bot.game.board;
        add_list(b, vec![[2, 7], [2, 8], [2, 9], [2, 0], [2, 1], [2, 2]]);
        bot.game.active = Piece::new(PIECE_O);

        // remove all held pieces (not o)
        let pieces: Vec<_> = bot
            .move_gen(1)
            .placements
            .iter()
            .map(|p| p.base_piece)
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
        let mut bot = Bot::<NoPruner, NoEvaluator>::new();
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
        let mut bot = Bot::<NoPruner, NoEvaluator>::new();
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
        let mut bot = Bot::<NoPruner, NoEvaluator>::new();
        bot.game.board = l_spin_board_5();
        println!("{}", bot.game.board);
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

    #[test]
    fn test_number_placements_generated() {
        // NOTE this is dependent on the queue
        // assumes commit 90232f86f194a8e819f89ea80124da7e01ef9b59 is correct
        let bot = Bot::<NoPruner, NoEvaluator>::with_seed(4);
        let desired_q = [1, 4, 5, 6];
        assert!(bot.game.active.r#type == 2);
        assert!(desired_q
            .into_iter()
            .enumerate()
            .all(|(i, p)| p == bot.game.queue.peek_ahead(i as u8)));
        assert_eq!(bot.move_gen(3).placements.len(), 118_151);

        let bot = Bot::<NoPruner, NoEvaluator>::with_seed(19);
        let desired_q = [3, 6, 5, 1];
        assert!(bot.game.active.r#type == 4);
        assert!(desired_q
            .into_iter()
            .enumerate()
            .all(|(i, p)| p == bot.game.queue.peek_ahead(i as u8)));
        assert!(bot.move_gen(3).placements.len() == 333_078);
        // add some more stuff to test :D
    }
}
