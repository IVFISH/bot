#![allow(dead_code)]

use crate::board::Board;
use crate::constants::piece_constants::PIECE_NAME;
use crate::piece::Piece;
use crate::piece_queue::PieceQueue;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Default, Debug, Eq, Hash, PartialEq)]
pub struct VersusStats {
    pub combo: u8,
    pub attack_chain: u8,
    pub b2b: u8,
    pub total_attack: u8,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Game {
    pub board: Board,
    pub active: Piece,
    pub hold: Option<u8>,
    pub queue: PieceQueue,
    pub versus: VersusStats,
}

impl Display for VersusStats {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "attack: {} || combo: {} | b2b: {}",
            self.total_attack, self.combo, self.b2b
        )?;
        Ok(())
    }
}

impl VersusStats {
    /// Updates versus stats using the amount of lines cleared and the type
    #[inline]
    fn clear_lines(&mut self, amt: u8, b2b: bool) {
        if amt == 0 {
            self.combo = 0;
            self.attack_chain = 0;
        } else {
            // TODO: IMPLEMENT ATTACK TABLE
            let attk = amt + self.combo - 1;
            // let attk = (self.combo as i8 - 2).clamp(0, 100) as u8;
            self.attack_chain += attk;
            self.total_attack += attk;
            self.combo += 1;

            if b2b {
                self.b2b += 1;
            } else {
                self.b2b = 0;
            }
        }
    }
}

impl Display for Game {
    /// returns a string representation of the board
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{} | [{}] {}",
            PIECE_NAME
                .get(self.hold.unwrap_or(255) as usize)
                .unwrap_or(&'-'),
            PIECE_NAME[self.active.r#type as usize],
            self.queue
        )?;
        write!(f, "{}", self.board)?;
        writeln!(f, "{}", self.versus)?;
        Ok(())
    }
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
            versus: VersusStats::default(),
        }
    }

    /// returns a game with a random seed
    pub fn random() -> Self {
        let random_seed = 4;
        Self::new(random_seed)
    }

    // mutators ---------------------------------
    /// places the current active piece onto the board
    /// this also updates the queue and the new active
    /// (this does not check for validity of placement)
    pub fn place_active(&mut self) -> &mut Self {
        // check if the piece is a spin
        let is_spin = false; // TODO: figure this out somehow. controller method??
                             // update the board
        self.board.set_piece(&self.active);
        let cleared = self.board.clear_lines();
        self.versus.clear_lines(cleared.count_ones() as u8, is_spin);
        // update the active
        self.active = self.queue.next();
        self
    }

    /// sets the active piece to the new piece
    pub fn set_active(&mut self, piece: Piece, held: bool) -> &mut Self {
        if held {
            self.hold();
        }
        self.active = piece;
        self
    }

    /// swaps the hold and active piece (or gets hold from queue)
    pub fn hold(&mut self) -> &mut Self {
        let h = self.hold;
        self.hold = Some(self.active.r#type);
        self.active = Piece::new(h.unwrap_or_else(|| self.queue.next_piece_type()));
        self
    }

    /// returns the piece that would be given from hold
    pub fn get_hold_piece(&self) -> Piece {
        Piece::new(self.hold.unwrap_or_else(|| self.queue.peek()))
    }
}
