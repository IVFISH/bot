#![allow(dead_code)]

use crate::board::Board;
use crate::piece::Piece;
use crate::piece_queue::PieceQueue;

#[derive(Default, Copy, Clone, Debug)]
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
            ..Default::default()
        }
    }

    /// returns a game with a random seed
    pub fn random() -> Self {
        let random_seed = 0;
        Self::new(random_seed)
    }

    // mutators ---------------------------------
    /// places the current active piece onto the board
    /// this also updates the queue and the new active
    /// (this does not check for validity of placement)
    pub fn place_active(&mut self) {
        self.board.set_piece(&self.active);
        self.active = self.queue.next();
    }

    /// updates the active piece to a new piece if possible
    /// this swaps the hold and active if the piece is the hold piece
    pub fn update_active(&mut self, piece: Piece) {
        if piece.r#type == self.active.r#type  {
            self.active = piece;
        } else if self.hold.is_some() && self.hold.unwrap() == piece.r#type {
            self.hold = Some(self.active.r#type);
            self.active = piece;
        }
    }
}
