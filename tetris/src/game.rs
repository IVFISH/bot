use std::fmt::{Display, Formatter};
use crate::board::*;
use crate::placement::*;
use crate::queue::*;
use crate::board::*;
use crate::placement::piece_data::offset::{FIVE_OFFSETS, THREE_OFFSETS, FIVE_180_OFFSETS, THREE_180_OFFSETS, O_OFFSETS};

use crate::placement::piece_data::rotation::{RotationDirection, RotationState};


pub struct Game {
    board: Board,

    piece_queue: PieceQueue,
    garbage_queue: GarbageQueue,

    pub game_data: GameData,

    pub active_piece: Placement,
    pub hold_piece: Option<Placement>,
}

impl Default for Game {
    fn default() -> Self {
        let mut out = Self {
            board: Default::default(),
            piece_queue: Default::default(),
            garbage_queue: Default::default(),
            game_data: Default::default(),

            active_piece: Default::default(),
            hold_piece: None,
        };

        out.active_piece = Placement::new(out.piece_queue.next());
        out
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.board)?;
        write!(f, "Queue: {}", self.piece_queue)?;

        Ok(())
    }
}

impl Game {
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

    pub fn piece_down(&mut self) -> bool {
        self.safe_move_active_piece_by_vector(MoveVector(-1, 0))
    }

    pub fn piece_up(&mut self) -> bool {
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

    fn add_garbage(&self) {
        unimplemented!()
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