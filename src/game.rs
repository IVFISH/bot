#![allow(dead_code)]

use crate::board::Board;
use crate::piece::Piece;
use crate::piece_queue::PieceQueue;

#[derive(Copy, Clone, Debug)]
pub struct Game {
    pub board: Board,
    pub active: Piece,
    pub hold: Option<u8>,
    pub queue: PieceQueue,
}

impl Game {
    // constructors -----------------------------
    /// returns a game with an empty board and random queue
    pub fn new(seed: usize) -> Self {
        let mut queue = PieceQueue::new(seed);
        Self {
            active: queue.next(),
            queue,
            hold: None,
            board: Board::default(),
        }
    }

    /// returns a game with a random seed
    pub fn random() -> Self {
        let random_seed = 1;
        Self::new(random_seed)
    }

    // mutators ---------------------------------
    /// places the current active piece onto the board
    /// this also updates the queue and the new active
    /// (this does not check for validity of placement)
    pub fn place_active(&mut self) {
        self.board.set_piece(&self.active);
        self.board.clear_lines();
        self.active = self.queue.next();
    }

    /// swaps the hold and active piece (or gets hold from queue)
    pub fn hold(&mut self) {
        let h = self.hold;
        self.hold = Some(self.active.r#type);
        self.active = Piece::new(h.unwrap_or_else(|| self.queue.next_piece_type()));
    }
}
