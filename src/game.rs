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
    pub history: u128,
    pub line_clears: u32,
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
    #[inline]
    fn clear_lines(&mut self, amt: u8, b2b: bool) {
        if amt == 0 {
            self.combo = 0;
            self.attack_chain = 0;
        } else {
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
            history: 0,
            line_clears: 0,
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
    pub fn place_active(&mut self, held: bool) -> &mut Self {
        // push the piece into the history
        let t_spin = false; // fix (have a board method)
        self.history = self.history << 16 | (self.active.encode(held, t_spin) as u128);
        // update the board
        self.board.set_piece(&self.active);
        let cleared = self.board.clear_lines();
        self.versus.clear_lines(cleared.count_ones() as u8, false);
        // update line clear history
        self.line_clears <<= 4;
        self.line_clears |= (cleared >> self.active.bottom_row().unwrap()) & 0xF;
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

    /// recovers the past board states (up to 8)
    /// THIS DOES NOT REWIND ANYTHING BESIDES BOARD
    pub fn past_states(&self) -> Vec<Self> {
        let mut history = self.history;
        let mut line_clears = self.line_clears;
        let mut games = Vec::new();
        let mut prev = *self;
        while history != 0 {
            games.push(prev);
            let piece = Piece::decode((history & (u16::MAX as u128)) as u16);
            let rows = (line_clears & 0xF) << piece.bottom_row().unwrap();
            prev.board.insert_rows(rows as usize);
            prev.board.remove_piece(&piece);
            history >>= 16;
            line_clears >>= 4;
        }
        games.push(prev);
        games.reverse();
        games
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::piece_constants::*;
    use crate::test_api::functions::*;

    #[test]
    fn regenerate_past_boards() {
        let mut game = Game::new(1337);
        let mut past = vec![game];
        // queue = OISTLJZ
        assert_games_eq(&game.past_states(), &past);

        let o = Piece {
            r#type: PIECE_O,
            dir: 0,
            row: 0,
            col: 0,
        };
        game.set_active(o, false);
        game.place_active(false);
        past.push(game);
        let i = Piece {
            r#type: PIECE_I,
            dir: 0,
            row: 0,
            col: 3,
        };
        game.set_active(i, false);
        game.place_active(false);
        past.push(game);
        let s = Piece {
            r#type: PIECE_S,
            dir: 1,
            row: 1,
            col: 5,
        };
        game.set_active(s, false);
        game.place_active(false);
        past.push(game);
        let t = Piece {
            r#type: PIECE_T,
            dir: 0,
            row: 0,
            col: 8,
        };
        game.set_active(t, false);
        game.place_active(false);
        past.push(game);
        let l = Piece {
            r#type: PIECE_L,
            dir: 0,
            row: 0,
            col: 3,
        };
        game.set_active(l, false);
        game.place_active(false);
        past.push(game);

        for g in &past {
            println!("{}", g);
        }
        assert_eq!(game.past_states().len(), 6);
        assert_games_eq(&game.past_states(), &past);
    }
}
