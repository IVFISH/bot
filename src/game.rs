#![allow(dead_code)]

use crate::board::Board;
use crate::constants::piece_constants::SpinType;
use crate::constants::piece_constants::PIECE_NAME;
use crate::constants::versus_constants::*;
use crate::piece::Piece;
use crate::piece_queue::PieceQueue;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Default, Debug, Eq, Hash, PartialEq)]
pub struct VersusStats {
    pub combo: u8,
    pub attack_chain: u8,
    pub b2b: u8,
    pub last_attack: u8,
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
            self.last_attack, self.combo, self.b2b
        )?;
        Ok(())
    }
}

impl VersusStats {
    /// Updates versus stats using the amount of lines cleared and the type
    #[inline]
    fn clear_lines(&mut self, amt: u8, spin: SpinType) {
        if amt == 0 {
            self.combo = 0;
            self.attack_chain = 0;
        } else {
            // 0: ????  |  S, TMS
            // 1: 0.25  |  D, TMD, B2B-TMS
            // 2: 0.50  |  T, TS, TMT, B2B-TMD,
            // 3: 0.75  |  B2B-TMT, B2B-TS
            // 4: 1.00  |  Q, TD
            // 5: 1.25  |  B2B-Q, B2B-TD
            // 6: 1.50  |  TT
            // 7: 1.75  |  B2B-TT
            let attack_table = if self.b2b == 0 {
                match (spin, amt) {
                    (SpinType::Full, 1) => ATTACK2, // TSS
                    (SpinType::Full, 2) => ATTACK4, // TSD
                    (SpinType::Full, 3) => ATTACK6, // TST
                    (_, 1) => ATTACK0,
                    (_, 2) => ATTACK1,
                    (_, 3) => ATTACK2,
                    (_, 4) => ATTACK4,
                    _ => panic!("Can't calculate garbage: {}", amt),
                }
            } else {
                match (spin, amt) {
                    (_, 4) => ATTACK5,              // Quad
                    (SpinType::Full, 1) => ATTACK3, // TSS
                    (SpinType::Full, 2) => ATTACK5, // TSD
                    (SpinType::Full, 3) => ATTACK7, // TST
                    (SpinType::Mini, 1) => ATTACK1,
                    (SpinType::Mini, 2) => ATTACK2,
                    (SpinType::Mini, 3) => ATTACK3,
                    (SpinType::None, 1) => ATTACK0,
                    (SpinType::None, 2) => ATTACK1,
                    (SpinType::None, 3) => ATTACK2,
                    _ => panic!("Can't calculate b2b garbage: {}", amt),
                }
            };
            let attk = attack_table[self.combo as usize];
            self.attack_chain = attk;
            self.last_attack += attk;
            self.combo += 1;

            // Check if b2b clear
            if spin != SpinType::None || amt == 4 {
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
        self.board.set_piece(&self.active);
        let cleared = self.board.clear_lines().count_ones();
        self.versus.clear_lines(cleared as u8, self.active.spin);
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
