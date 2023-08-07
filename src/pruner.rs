use crate::placement::*;
use crate::board::*;
use crate::constants::board_constants::*;

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
    fn prune(&self, placements: Vec<Placement>) -> Vec<Placement> {
        placements.into_iter().filter(|p| self.condition(&p)).collect()
    }
}

pub struct AllClearPruner {
    height: usize

}

impl AllClearPruner {
    /// the sum of the # of odd line clears and t-pieces must be even
    /// https://docs.google.com/document/d/1udtq235q2SdoFYwMZNu-GRYR-4dCYMkp0E8_Hw1XTyg/edit
    fn checkerboard_rule(&self, board: &[u64], start: usize, end: usize, game: &Game) -> bool {
        // get count of all odd line clears
        // get range of possible t pieces (including placed)
        // range > 0 or parity is the same
        
        // range = 0 if the bag has >1 ts
        // range = 1 if the bag of pieces has <=1 ts
        let n = Board::cell_count(board) >> 2; // divide by PIECE_SIZE
        let count_t_history = 0u8;
        let count_t_queue = 0u8;
        let mut history = game.history;
        while history > 0 {
            count_t_history += (history & 0b111 == PIECE_T) as u8;
            history >>= 16;
        }
        for i in 0..=n {
            // can do this with a filter and count
            count_t_queue += (game.queue.peek_ahead(i) == PIECE_T) as u8;
        }
        let odd_clears = 0; // TODO
        count_t_queue < 2 || (count_t_history ^ count_t_queue ^ odd_clears & 1 == 0)
    }
    
    /// the sum of the number of L, J, and vertical T pieces must be even
    /// https://docs.google.com/document/d/1udtq235q2SdoFYwMZNu-GRYR-4dCYMkp0E8_Hw1XTyg/edit
    fn columnar_rule(&self, board: &[u64], start: usize, end: usize, game: &Game) -> bool {
        // get count of current # of L, J, vertical T within the partition
        // get range of possible l, j, vertical T placements
        // range > 1 or parity is the same

        // number of vertical Ts = min(# cols with 3 height, max #ts)
        let n = Board::cell_count(board) >> 2; // divide by PIECE_SIZE
        let count_t_queue = 0u8;
        let count_lj_queue = 0u8;
        for i in 0..=n {
            match game.queue.peek_ahead(i) {
                PIECE_T => count_t_queue += 1,
                PIECE_L | PIECE_J => count_lj_queue += 1,
                _ => (),
            }
        }
        let count_3_height = board.iter().filter(|c| c == 0b1000 || c == 0b0001).count();
        let count_vert_t_queue = std::cmp::min(count_t_queue, count_3_height);
        let count_ljt_queue = count_vert_t_queue + count_lj_queue;
        
        // get counts of l, j, vert_t in history
        let mut history = game.history;
        let mut count_ljt_history = 0u8;
        while history > 0 {
            let kind = history & 0b111;
            // 2nd digit of rot state = 1 --> vertical
            count_ljt_history += (kind == PIECE_T && (history & 0b01000 == 1)) 
                || (kind == PIECE_L)
                || (kind == PIECE_J) as u8;
            history >>= 16;
        }

        count_ljt_queue < 2 || (count_ljt_queue ^ count_ljt_history & 1 == 0)
    }
    
    /// returns if there are any piece dependencies
    /// that are not in the queue
    /// (not sure how this one works yet)
    fn piece_dependencies(&self, board: &[u64], start: usize, end: usize) -> bool {
        true
    }
    
    /// checks if the number of empty minos
    /// is a multiple of `constants::piece_constants::PIECE_SIZE`
    fn mino_rule(&self, board: &[u64]) -> bool {
        // if we are using tetraminos
        // we can just check if the last 2 bits are 0
        Board::cell_count(board) & 0b11 == 0
    }
    
    // checks if the maximum cell height is <= height
    fn height_rule(&self, board: &[u64]) -> bool {
        Board::get_max_height(board) <= self.height
    }

    // do all the conditions
    fn all_rules(&self, board: &[u64], start: usize, end: usize, game: &Game) -> bool {
        self.checkerboard_rule(b, start, end, game) 
                 && self.columnar_rule(b, start, end, game)
                 && self.piece_dependencies(b, start, end)
                 && self.mino_rule(b)
                 && self.height_rule(b))
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

    fn condition(&self, placement: &Placement) -> bool {
        let game = placement.game_after;
        let boards = game.board.partition();
        self.all_rules(&game.board.arr, 0, BOARD_WIDTH, &game) 
            && boards
            .into_iter()
            .all(|(b, start, end)| self.all_rules(b, start, end, &game))
    }
}

pub struct NoPruner {

}

impl Pruner for NoPruner {
    fn new() -> Self {
        Self {}
    }

    fn condition(&self, _placement: &Placement) -> bool {
        false
    }

    fn prune(&self, placements: Vec<Placement>) -> Vec<Placement> {
        placements
    }
}
