use std::fmt::{Display, Formatter};
use crate::board::*;
use crate::placement::*;
use crate::queue::*;
use crate::board::*;
use crate::placement::piece_data::offset::{FIVE_OFFSETS, THREE_OFFSETS, FIVE_180_OFFSETS, THREE_180_OFFSETS, O_OFFSETS};

use crate::placement::piece_data::rotation::{RotationDirection, RotationState};


#[derive(Default)]
pub struct Game {
    board: Board,

    piece_queue: PieceQueue,
    garbage_queue: GarbageQueue,

    pub game_data: GameData,

    pub active_piece: Placement,
    pub hold_piece: Option<Placement>,
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.board.to_string(&self.active_piece))?;
        write!(f, "Queue: {}", self.piece_queue)?;

        Ok(())
    }
}

impl Game {
    #[allow(unused)]
    pub fn new(seed: Option<usize>) -> Self {

        let mut out = Self {
            piece_queue: PieceQueue::new(seed),
            ..Default::default()
        };

        out.active_piece = Placement::new(out.piece_queue.next());
        out
    }

    pub fn set_piece(&mut self) {
        self.board.set_piece(&self.active_piece, true);
        self.active_piece = Placement::new(self.piece_queue.next());
    }

    fn safe_move_active_piece_by_vector(&mut self, v: MoveVector) -> bool {
        //  checks if center is invalid
        if !self.active_piece.move_by_vector(v) {
            return false;
        }

        // checks if rest of piece is invalid
        self.board.check_valid_placement(&self.active_piece).is_ok()
    }

    pub fn piece_left(&mut self) -> bool {
        self.safe_move_active_piece_by_vector(MoveVector(0, -1))
    }

    pub fn piece_right(&mut self) -> bool {
        self.safe_move_active_piece_by_vector(MoveVector(0, 1))
    }

    pub fn piece_das_left(&mut self) {
        while self.piece_left() {
            // this is intended wheeee
        }
    }

    pub fn piece_das_right(&mut self) {
        while self.piece_right() {
            // this is intended wheeee
        }
    }

    pub fn piece_soft_drop(&mut self) -> bool {
        let out = self.piece_down();

        while self.piece_down() {
            // this is intended wheeee
        }

        out
    }

    fn piece_down(&mut self) -> bool {
        self.safe_move_active_piece_by_vector(MoveVector(-1, 0))
    }

    fn piece_up(&mut self) -> bool {
        self.safe_move_active_piece_by_vector(MoveVector(1, 0))
    }

    fn rotate_with_kick(&mut self, dir: RotationDirection, kicks: Vec<MoveVector>) -> bool {
        self.active_piece.rotate(dir);

        for index in 0..kicks.len() {
            if self.active_piece.move_by_vector(kicks[index]) {
                if self.board.check_valid_placement(&self.active_piece).is_ok() {
                    return true;
                }
            } else {
                self.active_piece.move_by_vector(kicks[index].negative());
            }
        }

        // undo if cannot kick
        self.active_piece.rotate(4 - dir);
        false
    }

    fn get_kicks(&self, dir: RotationDirection) -> Vec<MoveVector> {
        let before = self.active_piece.rotation_state;

        let kicks;
        if self.active_piece.piece_type == 4 {  // I piece is the special child
            if dir == 2 {
                kicks = FIVE_180_OFFSETS[before].to_vec()
            } else {
                kicks = FIVE_OFFSETS[before][dir / 2].to_vec();
            }
        } else if self.active_piece.piece_type == 2 { // O piece is the other special child
            kicks = O_OFFSETS[before].to_vec();
        } else {
            if dir == 2 {
                kicks = THREE_180_OFFSETS[before].to_vec()
            } else {
                kicks = THREE_OFFSETS[before][dir / 2].to_vec();
            }
        }

        kicks
    }

    pub fn piece_rotate_cw(&mut self) -> bool {
        self.rotate_with_kick(1, self.get_kicks(1))
    }

    pub fn piece_rotate_180(&mut self) -> bool {
        self.rotate_with_kick(2, self.get_kicks(2))
    }

    pub fn piece_rotate_ccw(&mut self) -> bool {
        self.rotate_with_kick(3, self.get_kicks(3))
    }

    fn add_garbage(&mut self, amt: usize, update_heights: bool) {
        self.board.add_garbage(GarbageItem::new(amt), update_heights);
    }

    fn reset(&mut self) {
        unimplemented!()
    }
}

pub struct GameData {
    pub all_clear: bool,
    pub combo: i8,
    pub b2b: i8,

    pub pieces_placed: u8,
    pub lines_cleared: u8,
    pub lines_sent: u8,

    pub game_over: bool,

    init_time: f32,
}

impl Default for GameData {
    fn default() -> Self {
        Self {
            all_clear: false,
            combo: -1,
            b2b: -1,
            pieces_placed: 0,
            lines_cleared: 0,
            lines_sent: 0,
            game_over: false,
            init_time: 0.0,
        }
    }
}

impl GameData {
    fn run_time() -> f32 {
        unimplemented!()
    }

    #[allow(unused)]
    fn new(init_time: f32) -> Self {
        Self {
            init_time,
            ..Default::default()
        }
    }
}