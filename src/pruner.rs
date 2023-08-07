use crate::board::*;
use crate::constants::board_constants::*;
use crate::constants::piece_constants::*;
use crate::game::*;
use crate::placement::*;
use std::collections::HashSet;

pub trait Pruner {
    /// constructor method
    fn new() -> Self;

    /// required condition for pruning
    /// return true means that it should not be pruned
    /// assumes that the placement is valid to set
    fn condition(&self, placement: &Placement) -> bool;

    /// pruning method that prunes based on the condition
    /// should not generally be overridden
    /// returns the filtered list
    fn prune(&self, placements: HashSet<Placement>) -> HashSet<Placement> {
        placements
            .into_iter()
            .filter(|p| self.condition(&p))
            .collect()
    }
}

pub struct AllClearPruner {
    height: usize,
}

impl AllClearPruner {
    /// returns whether the placement contains a PC
    fn is_pc(&self, placement: &Placement) -> bool {
        placement.game_after.board.all_clear()
    }

    /// the sum of the # of odd line clears and t-pieces must be even
    /// https://docs.google.com/document/d/1udtq235q2SdoFYwMZNu-GRYR-4dCYMkp0E8_Hw1XTyg/edit
    fn checkerboard_rule(&self, board: &[u64], game: &Game, height: usize) -> bool {
        // checkerboard rule is only for areas of multiples of 2
        if (board.len() * height & 1 != 0) {
            return true;
        }

        // return if there are enough T-pieces or [odd] line clears left to correct for the parity
        let parity = Board::checkerboard_parity(board).abs();
        let n = (board.len() * height) as u32 - Board::cell_count(board) >> 2;
        let count_t = (0..=n)
            .filter(|i| game.queue.peek_ahead(*i as u8) == PIECE_T)
            .count();
        parity <= (count_t + self.height) as i8
    }

    /// the sum of the number of L, J, and vertical T pieces must be even
    /// https://docs.google.com/document/d/1udtq235q2SdoFYwMZNu-GRYR-4dCYMkp0E8_Hw1XTyg/edit
    fn columnar_rule(&self, board: &[u64], game: &Game, height: usize) -> bool {
        // columnar rule is only for areas of multiples of 4
        if board.len() * height & 0b11 != 0 {
            return true;
        }

        let parity = Board::columnar_parity(board).abs();
        let n = (board.len() * height) as u32 - Board::cell_count(board) >> 2;
        let count = (0..=n)
            .filter(|i| {
                let piece = game.queue.peek_ahead(*i as u8);
                piece == PIECE_T || piece == PIECE_L || piece == PIECE_J
            })
            .count();
        parity <= count as i8
    }

    /// returns if there are any piece dependencies
    /// that are not in the queue
    /// (not sure how this one works yet)
    fn piece_dependencies(&self, board: &[u64], height: usize) -> bool {
        true
    }

    /// checks if the number of empty minos
    /// is a multiple of `constants::piece_constants::PIECE_SIZE`
    fn mino_rule(&self, board: &[u64], height: usize) -> bool {
        // if we are using tetraminos
        // we can just check if the last 2 bits are 0
        let empty = (board.len() * height) as u32 - Board::cell_count(board);
        empty & 0b11 == 0
    }

    /// checks if the maximum cell height is <= height
    fn height_rule(&self, board: &[u64], height: usize) -> bool {
        Board::get_max_height(board) <= height
    }

    /// do all the conditions
    fn all_rules(&self, board: &[u64], game: &Game, height: usize) -> bool {
        self.height_rule(board, height)
            && self.checkerboard_rule(board, game, height)
            && self.columnar_rule(board, game, height)
            && self.piece_dependencies(board, height)
            && self.mino_rule(board, height)
    }

    /// do conditions for partials
    fn partition_rules(&self, board: &[u64], game: &Game, height: usize) -> bool {
        self.mino_rule(board, height)
    }
}

impl Default for AllClearPruner {
    fn default() -> Self {
        Self { height: 4 }
    }
}

impl Pruner for AllClearPruner {
    fn new() -> Self {
        Self::default()
    }

    fn prune(&self, placements: HashSet<Placement>) -> HashSet<Placement> {
        let (pc, no_pc): (HashSet<_>, HashSet<_>) = placements
            .into_iter()
            .filter(|p| self.condition(&p))
            .partition(|p| self.is_pc(&p));
        if pc.is_empty() {
            no_pc
        } else {
            pc
        }
    }

    fn condition(&self, placement: &Placement) -> bool {
        let game = placement.game_after;
        let height = match Board::cell_count(&game.board.arr) & 0b11 {
            2 => 3,
            0 => 4,
            _ => 0,
        };

        height == 0
            || self.all_rules(&game.board.arr, &game, height)
                && game
                    .board
                    .partition(height)
                    .into_iter()
                    .all(|b| self.partition_rules(b, &game, height))
    }
}

pub struct NoPruner {}

impl Pruner for NoPruner {
    fn new() -> Self {
        Self {}
    }

    fn condition(&self, _placement: &Placement) -> bool {
        false
    }

    fn prune(&self, placements: HashSet<Placement>) -> HashSet<Placement> {
        placements
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::piece::*;
    use crate::test_api::functions::*;

    #[test]
    fn placing_above_height() {
        let pruner = AllClearPruner::new();
        let placement1 = Placement {
            piece: Piece::new(0),
            held: false,
            game_before: Game::random(),
            game_after: Game::random(),
        };
        assert!(pruner.condition(&placement1));

        let mut placement2 = placement1.clone();
        placement2.game_after.board = tst_board();
        assert!(!pruner.condition(&placement2));

        let mut placements = (0..10).map(|_| placement1.clone()).collect::<HashSet<_>>();
        placements.insert(placement2.clone());
        let placements = pruner.prune(placements);
        assert_eq!(placements.len(), 9);
        assert!(placements
            .into_iter()
            .all(|p| p.game_after.board == placement1.game_after.board));
    }
}
