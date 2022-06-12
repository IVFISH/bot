use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use crate::board::*;
use crate::placement::*;
use crate::queue::*;
use crate::board::*;
use crate::errors::GameError;
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

    pub fn set_piece(&mut self) -> Result<(), GameError> {
        self.board.top_out(&self.active_piece, &Placement::new(self.piece_queue.peek()), 20)?;

        self.board.set_piece(&self.active_piece, true);
        self.active_piece = Placement::new(self.piece_queue.next());

        Ok(())
    }

    fn safe_move_active_piece_by_vector(&mut self, v: MoveVector) -> bool {
        //  checks if center is invalid
        if !self.active_piece.move_by_vector(v) {
            return false;
        }

        // checks if rest of piece is invalid
        let safe = self.board.check_valid_placement(&self.active_piece).is_ok();

        if !safe {
            self.active_piece.move_by_vector(v.negative());
        }

        safe
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
                } else {
                    self.active_piece.move_by_vector(kicks[index].negative());
                }
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

    pub fn add_garbage(&mut self, amt: usize, update_heights: bool) {
        self.board.add_garbage(GarbageItem::new(amt), update_heights);
    }

    fn reset(&mut self) {
        unimplemented!()
    }

    pub fn hold(&mut self) {}

    fn manual_set_queue(&mut self, new_queue: VecDeque<usize>) {
        self.piece_queue.manual_queue_set(new_queue)
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

#[cfg(test)]
mod game_tests {
    use super::*;

    #[test]
    fn test_queue_init() {
        let game = Game::new(None);

        // tests if queue is more than 5 elements long. find some more direct way
        format!("{}", game.piece_queue);
    }

    #[test]
    fn set_piece() {
        let mut game = Game::new(Some(1));

        game.active_piece.move_by_vector(MoveVector(-5, 0));
        game.set_piece().expect("crash and burn");

        // making sure calling next piece correctly
        assert_eq!(game.active_piece.piece_type, 5);

        assert!(!game.board.get(21, 4));
        assert!(game.board.get(16, 4));
        assert!(game.board.get(17, 5));
        assert!(!game.board.get(15, 5));
    }

    #[test]
    fn test_top_out() {
        let mut game = Game::new(Some(1));

        assert!(game.set_piece().is_err());

        let mut game = Game::new(None);
        game.piece_soft_drop();

        assert!(game.set_piece().is_ok());
    }

    #[test]
    fn test_move_piece() {
        let mut game = Game::new(Some(1337));
        // OISTLJZ

        game.piece_das_right();
        game.piece_down();

        assert_eq!(game.active_piece.abs_locations().unwrap(),
                   [Point { row: 21, col: 8 }, Point { row: 21, col: 9 }, Point { row: 20, col: 8 }, Point { row: 20, col: 9 }]);

        game.piece_down();
        game.set_piece().expect("crash and burn");
        game.piece_soft_drop();

        assert_eq!(game.active_piece.abs_locations().unwrap(),
                   [Point { row: 0, col: 3 }, Point { row: 0, col: 4 }, Point { row: 0, col: 5 }, Point { row: 0, col: 6 }]);
    }

    #[test]
    fn test_rotate_piece() {
        let mut game = Game::new(Some(1336));
        // TLJSO

        game.piece_rotate_cw();

        assert_eq!(game.active_piece.abs_locations().unwrap(),
                   [Point { row: 21, col: 5 }, Point { row: 22, col: 4 }, Point { row: 21, col: 4 }, Point { row: 20, col: 4 }]);

        game.piece_rotate_cw();

        assert_eq!(game.active_piece.abs_locations().unwrap(),
                   [Point { row: 20, col: 4 }, Point { row: 21, col: 5 }, Point { row: 21, col: 4 }, Point { row: 21, col: 3 }]);

        game.piece_rotate_ccw();

        assert_eq!(game.active_piece.abs_locations().unwrap(),
                   [Point { row: 21, col: 5 }, Point { row: 22, col: 4 }, Point { row: 21, col: 4 }, Point { row: 20, col: 4 }]);

        game.piece_soft_drop();
        game.set_piece().expect("crash and burn 2");

        game.piece_rotate_180();

        assert_eq!(game.active_piece.abs_locations().unwrap(),
                   [Point { row: 20, col: 3 }, Point { row: 21, col: 5 }, Point { row: 21, col: 4 }, Point { row: 21, col: 3 }]);
    }

    fn test_hold() {}

    #[test]
    fn test_wall_kick() {
        println!("aaa");
        let mut game = Game::new(Some(1336));
        // TLJSO

        game.piece_rotate_cw();
        game.piece_das_left();

        assert!(game.piece_rotate_ccw());

        assert_eq!(game.active_piece.abs_locations().unwrap(),
                   [Point { row: 22, col: 1 }, Point { row: 21, col: 0 }, Point { row: 21, col: 1 }, Point { row: 21, col: 2 }]);

        let mut game = Game::new(Some(1337));
        game.piece_soft_drop();
        game.set_piece().expect("die");

        // I piece
        game.piece_rotate_ccw();
        game.piece_das_right();

        assert!(game.piece_rotate_ccw());
        assert_eq!(game.active_piece.abs_locations().unwrap(),
                   [Point { row: 20, col: 9 }, Point { row: 20, col: 8 }, Point { row: 20, col: 7 }, Point { row: 20, col: 6 }]);
    }

    #[test]
    fn test_floor_kick() {
        let mut game = Game::new(Some(1336));

        game.piece_soft_drop();

        assert!(game.piece_rotate_cw());

        println!("{} {:?}", game, game.active_piece.abs_locations());

        assert_eq!(game.active_piece.abs_locations().unwrap(),
                   [Point { row: 1, col: 4 }, Point { row: 2, col: 3 }, Point { row: 1, col: 3 }, Point { row: 0, col: 3 }]);
    }

    #[test]
    fn test_srs_jank() {

        // z spin 1
        let mut game = Game::new(None);
        game.active_piece = Placement::new(0);

        game.board.add(0, 0, false);
        game.board.add(1, 0, false);
        game.board.add(0, 1, false);
        game.board.add(1, 1, false);
        game.board.add(0, 2, false);
        game.board.add(1, 2, false);
        game.board.add(0, 3, false);
        game.board.add(1, 5, false);
        game.board.add(0, 6, false);
        game.board.add(1, 6, false);
        game.board.add(0, 7, false);
        game.board.add(1, 7, false);
        game.board.add(0, 8, false);
        game.board.add(1, 8, false);
        game.board.add(0, 9, false);
        game.board.add(1, 9, false);

        println!("{}", game);
        game.piece_rotate_ccw();
        game.piece_right();
        game.piece_soft_drop();
        game.piece_rotate_ccw();
        println!("{}", game);

        assert_eq!(game.active_piece.abs_locations().unwrap(),
                   [Point { row: 0, col: 5 }, Point { row: 0, col: 4 }, Point { row: 1, col: 4 }, Point { row: 1, col: 3 }]);

        // z spin 2
        let mut game = Game::new(None);
        game.active_piece = Placement::new(0);

        game.board.add(0, 0, false);
        game.board.add(1, 0, false);
        game.board.add(2, 0, false);
        game.board.add(0, 1, false);
        game.board.add(1, 1, false);
        game.board.add(2, 1, false);
        game.board.add(0, 2, false);
        game.board.add(1, 2, false);
        game.board.add(2, 2, false);
        game.board.add(0, 3, false);
        game.board.add(1, 3, false);
        game.board.add(2, 3, false);
        game.board.add(0, 4, false);
        game.board.add(1, 6, false);
        game.board.add(2, 6, false);
        game.board.add(0, 7, false);
        game.board.add(1, 7, false);
        game.board.add(2, 7, false);
        game.board.add(0, 8, false);
        game.board.add(1, 8, false);
        game.board.add(2, 8, false);
        game.board.add(0, 9, false);
        game.board.add(1, 9, false);
        game.board.add(2, 9, false);

        println!("{}", game);
        game.piece_soft_drop();
        game.piece_rotate_cw();
        game.piece_rotate_cw();
        println!("{}", game);

        assert_eq!([Point { row: 0, col: 6 }, Point { row: 0, col: 5 }, Point { row: 1, col: 5 }, Point { row: 1, col: 4 }],
                   game.active_piece.abs_locations().unwrap());

        // s spin 1
        let mut game = Game::new(None);
        game.active_piece = Placement::new(3);

        game.board.add(0, 0, false);
        game.board.add(1, 0, false);
        game.board.add(0, 1, false);
        game.board.add(1, 1, false);
        game.board.add(0, 2, false);
        game.board.add(1, 2, false);
        game.board.add(0, 3, false);
        game.board.add(1, 3, false);
        game.board.add(1, 4, false);
        game.board.add(0, 6, false);
        game.board.add(0, 7, false);
        game.board.add(1, 7, false);
        game.board.add(0, 8, false);
        game.board.add(1, 8, false);
        game.board.add(0, 9, false);
        game.board.add(1, 9, false);

        println!("{}", game);
        game.piece_rotate_cw();
        game.piece_soft_drop();
        game.piece_rotate_cw();
        println!("{}", game);

        assert_eq!([Point { row: 0, col: 5 }, Point { row: 0, col: 4 }, Point { row: 1, col: 6 }, Point { row: 1, col: 5 }],
                   game.active_piece.abs_locations().unwrap());

        // s spin 2
        let mut game = Game::new(None);
        game.active_piece = Placement::new(3);

        game.board.add(0, 0, false);
        game.board.add(1, 0, false);
        game.board.add(2, 0, false);
        game.board.add(0, 1, false);
        game.board.add(1, 1, false);
        game.board.add(2, 1, false);
        game.board.add(0, 2, false);
        game.board.add(1, 2, false);
        game.board.add(2, 2, false);
        game.board.add(1, 3, false);
        game.board.add(2, 3, false);
        game.board.add(0, 5, false);
        game.board.add(0, 6, false);
        game.board.add(1, 6, false);
        game.board.add(2, 6, false);
        game.board.add(0, 7, false);
        game.board.add(1, 7, false);
        game.board.add(2, 7, false);
        game.board.add(0, 8, false);
        game.board.add(1, 8, false);
        game.board.add(2, 8, false);
        game.board.add(0, 9, false);
        game.board.add(1, 9, false);
        game.board.add(2, 9, false);

        println!("{}", game);
        game.piece_rotate_ccw();
        game.piece_right();
        game.piece_soft_drop();
        game.piece_rotate_ccw();
        println!("{}", game);

        assert_eq!([Point { row: 0, col: 4 }, Point { row: 0, col: 3 }, Point { row: 1, col: 5 }, Point { row: 1, col: 4 }],
                   game.active_piece.abs_locations().unwrap());

        // l spin 1
        let mut game = Game::new(None);
        game.active_piece = Placement::new(1);

        game.board.add(0, 0, false);
        game.board.add(1, 0, false);
        game.board.add(2, 0, false);
        game.board.add(0, 1, false);
        game.board.add(1, 1, false);
        game.board.add(2, 1, false);
        game.board.add(0, 2, false);
        game.board.add(1, 2, false);
        game.board.add(2, 2, false);
        game.board.add(2, 3, false);
        game.board.add(0, 4, false);
        game.board.add(0, 5, false);
        game.board.add(0, 6, false);
        game.board.add(1, 6, false);
        game.board.add(2, 6, false);
        game.board.add(0, 7, false);
        game.board.add(1, 7, false);
        game.board.add(2, 7, false);
        game.board.add(0, 8, false);
        game.board.add(1, 8, false);
        game.board.add(2, 8, false);
        game.board.add(0, 9, false);
        game.board.add(1, 9, false);
        game.board.add(2, 9, false);

        println!("{}", game);
        game.piece_rotate_ccw();
        game.piece_right();
        game.piece_soft_drop();
        game.piece_rotate_ccw();
        println!("{}", game);

        assert_eq!([Point { row: 0, col: 3 }, Point { row: 1, col: 5 }, Point { row: 1, col: 4 }, Point { row: 1, col: 3 }],
                   game.active_piece.abs_locations().unwrap());

        // l spin 2
        let mut game = Game::new(None);
        game.active_piece = Placement::new(1);

        game.board.add(0, 0, false);
        game.board.add(0, 1, false);
        game.board.add(0, 2, false);
        game.board.add(0, 3, false);
        game.board.add(0, 5, false);
        game.board.add(2, 5, false);
        game.board.add(0, 6, false);
        game.board.add(2, 6, false);
        game.board.add(0, 7, false);
        game.board.add(1, 7, false);
        game.board.add(2, 7, false);
        game.board.add(0, 8, false);
        game.board.add(1, 8, false);
        game.board.add(2, 8, false);
        game.board.add(0, 9, false);
        game.board.add(1, 9, false);
        game.board.add(2, 9, false);

        println!("{}", game);
        game.piece_rotate_cw();
        game.piece_left();
        game.piece_soft_drop();
        game.piece_right();
        game.piece_rotate_cw();
        println!("{}", game);

        assert_eq!([Point { row: 0, col: 4 }, Point { row: 1, col: 6 }, Point { row: 1, col: 5 }, Point { row: 1, col: 4 }],
                   game.active_piece.abs_locations().unwrap());

        // l spin 3
        let mut game = Game::new(None);
        game.active_piece = Placement::new(1);

        game.board.add(0, 0, false);
        game.board.add(1, 0, false);
        game.board.add(2, 0, false);
        game.board.add(0, 1, false);
        game.board.add(1, 1, false);
        game.board.add(2, 1, false);
        game.board.add(0, 2, false);
        game.board.add(1, 2, false);
        game.board.add(2, 2, false);
        game.board.add(0, 3, false);
        game.board.add(1, 3, false);
        game.board.add(2, 3, false);
        game.board.add(2, 6, false);
        game.board.add(0, 7, false);
        game.board.add(1, 7, false);
        game.board.add(2, 7, false);
        game.board.add(0, 8, false);
        game.board.add(1, 8, false);
        game.board.add(2, 8, false);
        game.board.add(0, 9, false);
        println!("{}", game);

        println!("{}", game);
        game.piece_rotate_cw();
        game.piece_soft_drop();
        game.piece_rotate_ccw();
        println!("{}", game);

        assert_eq!([Point { row: 1, col: 6 }, Point { row: 0, col: 4 }, Point { row: 0, col: 5 }, Point { row: 0, col: 6 }],
                   game.active_piece.abs_locations().unwrap());

        // j spin 1
        let mut game = Game::new(None);
        game.active_piece = Placement::new(5);

        game.board.add(0, 0, false);
        game.board.add(1, 0, false);
        game.board.add(0, 1, false);
        game.board.add(1, 1, false);
        game.board.add(0, 2, false);
        game.board.add(1, 2, false);
        game.board.add(0, 3, false);
        game.board.add(1, 3, false);
        game.board.add(1, 5, false);
        game.board.add(1, 6, false);
        game.board.add(0, 7, false);
        game.board.add(1, 7, false);
        game.board.add(0, 8, false);
        game.board.add(1, 8, false);
        game.board.add(0, 9, false);
        game.board.add(1, 9, false);

        println!("{}", game);
        game.piece_rotate_cw();
        game.piece_soft_drop();
        game.piece_rotate_ccw();
        println!("{}", game);

        assert_eq!([Point { row: 1, col: 4 }, Point { row: 0, col: 4 }, Point { row: 0, col: 5 }, Point { row: 0, col: 6 }],
                   game.active_piece.abs_locations().unwrap());

        // j spin 2
        let mut game = Game::new(None);
        game.active_piece = Placement::new(5);

        game.board.add(0, 0, false);
        game.board.add(1, 0, false);
        game.board.add(2, 0, false);
        game.board.add(0, 1, false);
        game.board.add(1, 1, false);
        game.board.add(2, 1, false);
        game.board.add(0, 2, false);
        game.board.add(1, 2, false);
        game.board.add(2, 2, false);
        game.board.add(0, 3, false);
        game.board.add(1, 3, false);
        game.board.add(2, 3, false);
        game.board.add(0, 4, false);
        game.board.add(0, 5, false);
        game.board.add(2, 5, false);
        game.board.add(2, 6, false);
        game.board.add(0, 7, false);
        game.board.add(1, 7, false);
        game.board.add(2, 7, false);
        game.board.add(0, 8, false);
        game.board.add(1, 8, false);
        game.board.add(2, 8, false);
        game.board.add(0, 9, false);
        game.board.add(1, 9, false);

        println!("{}", game);
        game.piece_rotate_cw();
        game.piece_soft_drop();
        game.piece_rotate_cw();
        println!("{}", game);

        assert_eq!([Point { row: 0, col: 6 }, Point { row: 1, col: 6 }, Point { row: 1, col: 5 }, Point { row: 1, col: 4 }],
                   game.active_piece.abs_locations().unwrap());

        // j spin 3
        let mut game = Game::new(None);
        game.active_piece = Placement::new(5);

        game.board.add(0, 0, false);
        game.board.add(1, 0, false);
        game.board.add(2, 0, false);
        game.board.add(0, 1, false);
        game.board.add(1, 1, false);
        game.board.add(2, 1, false);
        game.board.add(0, 2, false);
        game.board.add(1, 2, false);
        game.board.add(2, 2, false);
        game.board.add(0, 3, false);
        game.board.add(2, 3, false);
        game.board.add(0, 4, false);
        game.board.add(0, 6, false);
        game.board.add(1, 6, false);
        game.board.add(2, 6, false);
        game.board.add(0, 7, false);
        game.board.add(1, 7, false);
        game.board.add(2, 7, false);
        game.board.add(0, 8, false);
        game.board.add(1, 8, false);
        game.board.add(2, 8, false);
        game.board.add(0, 9, false);
        game.board.add(1, 9, false);
        game.board.add(2, 9, false);

        println!("{}", game);
        game.piece_rotate_ccw();
        game.piece_right();
        game.piece_soft_drop();
        game.piece_rotate_ccw();
        println!("{}", game);

        assert_eq!([Point { row: 0, col: 5 }, Point { row: 1, col: 5 }, Point { row: 1, col: 4 }, Point { row: 1, col: 3 }],
                   game.active_piece.abs_locations().unwrap());

        // s spin triple 1
        let mut game = Game::new(None);
        game.active_piece = Placement::new(3);

        game.board.add(0, 0, false);
        game.board.add(1, 0, false);
        game.board.add(2, 0, false);
        game.board.add(0, 1, false);
        game.board.add(1, 1, false);
        game.board.add(2, 1, false);
        game.board.add(0, 2, false);
        game.board.add(1, 2, false);
        game.board.add(2, 2, false);
        game.board.add(0, 3, false);
        game.board.add(2, 4, false);
        game.board.add(0, 5, false);
        game.board.add(1, 5, false);
        game.board.add(2, 5, false);
        game.board.add(3, 5, false);
        game.board.add(0, 6, false);
        game.board.add(1, 6, false);
        game.board.add(2, 6, false);
        game.board.add(3, 6, false);
        game.board.add(0, 7, false);
        game.board.add(1, 7, false);
        game.board.add(2, 7, false);
        game.board.add(3, 7, false);
        game.board.add(0, 8, false);
        game.board.add(1, 8, false);
        game.board.add(2, 8, false);
        game.board.add(3, 8, false);
        game.board.add(0, 9, false);
        game.board.add(1, 9, false);
        game.board.add(2, 9, false);
        game.board.add(3, 9, false);

        println!("{}", game);
        game.piece_soft_drop();
        game.piece_rotate_ccw();
        println!("{}", game);

        assert_eq!([Point { row: 1, col: 3 }, Point { row: 2, col: 3 }, Point { row: 0, col: 4 }, Point { row: 1, col: 4 }],
                   game.active_piece.abs_locations().unwrap());

        // s spin triple 2
        let mut game = Game::new(None);
        game.active_piece = Placement::new(3);

        game.board.add(0, 0, false);
        game.board.add(1, 0, false);
        game.board.add(2, 0, false);
        game.board.add(3, 0, false);
        game.board.add(0, 1, false);
        game.board.add(1, 1, false);
        game.board.add(2, 1, false);
        game.board.add(3, 1, false);
        game.board.add(0, 2, false);
        game.board.add(1, 2, false);
        game.board.add(2, 2, false);
        game.board.add(3, 2, false);
        game.board.add(4, 2, false);
        game.board.add(0, 3, false);
        game.board.add(1, 3, false);
        game.board.add(2, 3, false);
        game.board.add(4, 3, false);
        game.board.add(0, 4, false);
        game.board.add(2, 5, false);
        game.board.add(0, 6, false);
        game.board.add(1, 6, false);
        game.board.add(2, 6, false);
        game.board.add(0, 7, false);
        game.board.add(1, 7, false);
        game.board.add(2, 7, false);
        game.board.add(0, 8, false);
        game.board.add(1, 8, false);
        game.board.add(2, 8, false);
        game.board.add(0, 9, false);
        game.board.add(1, 9, false);
        game.board.add(2, 9, false);

        println!("{}", game);
        game.piece_right();
        game.piece_soft_drop();
        game.piece_left();
        game.piece_rotate_cw();
        println!("{}", game);

        assert_eq!([Point { row: 1, col: 5 }, Point { row: 0, col: 5 }, Point { row: 2, col: 4 }, Point { row: 1, col: 4 }],
                   game.active_piece.abs_locations().unwrap());

        // s spin triple 3
        let mut game = Game::new(None);
        game.active_piece = Placement::new(3);

        game.board.add(0, 0, false);
        game.board.add(1, 0, false);
        game.board.add(2, 0, false);
        game.board.add(3, 0, false);
        game.board.add(4, 0, false);
        game.board.add(0, 1, false);
        game.board.add(1, 1, false);
        game.board.add(2, 1, false);
        game.board.add(3, 1, false);
        game.board.add(4, 1, false);
        game.board.add(0, 2, false);
        game.board.add(1, 2, false);
        game.board.add(2, 2, false);
        game.board.add(3, 2, false);
        game.board.add(4, 2, false);
        game.board.add(0, 3, false);
        game.board.add(4, 3, false);
        game.board.add(2, 4, false);
        game.board.add(0, 5, false);
        game.board.add(1, 5, false);
        game.board.add(2, 5, false);
        game.board.add(0, 6, false);
        game.board.add(1, 6, false);
        game.board.add(2, 6, false);
        game.board.add(3, 6, false);
        game.board.add(4, 6, false);
        game.board.add(5, 6, false);
        game.board.add(0, 7, false);
        game.board.add(1, 7, false);
        game.board.add(2, 7, false);
        game.board.add(3, 7, false);
        game.board.add(4, 7, false);
        game.board.add(5, 7, false);
        game.board.add(0, 8, false);
        game.board.add(1, 8, false);
        game.board.add(2, 8, false);
        game.board.add(3, 8, false);
        game.board.add(4, 8, false);
        game.board.add(5, 8, false);
        game.board.add(0, 9, false);
        game.board.add(1, 9, false);
        game.board.add(2, 9, false);
        game.board.add(3, 9, false);
        game.board.add(4, 9, false);
        game.board.add(5, 9, false);

        println!("{}", game);
        game.piece_right();
        game.piece_rotate_ccw();
        game.piece_soft_drop();
        game.piece_rotate_cw();
        game.piece_rotate_cw();
        println!("{}", game);

        assert_eq!([Point { row: 1, col: 4 }, Point { row: 0, col: 4 }, Point { row: 2, col: 3 }, Point { row: 1, col: 3 }],
                   game.active_piece.abs_locations().unwrap());

        // z spin triple 1
        let mut game = Game::new(None);
        game.active_piece = Placement::new(0);

        game.board.add(0, 0, false);
        game.board.add(1, 0, false);
        game.board.add(2, 0, false);
        game.board.add(0, 1, false);
        game.board.add(1, 1, false);
        game.board.add(2, 1, false);
        game.board.add(0, 2, false);
        game.board.add(1, 2, false);
        game.board.add(2, 2, false);
        game.board.add(0, 3, false);
        game.board.add(1, 3, false);
        game.board.add(2, 3, false);
        game.board.add(2, 4, false);
        game.board.add(0, 5, false);
        game.board.add(5, 5, false);
        game.board.add(0, 6, false);
        game.board.add(1, 6, false);
        game.board.add(2, 6, false);
        game.board.add(3, 6, false);
        game.board.add(4, 6, false);
        game.board.add(0, 7, false);
        game.board.add(1, 7, false);
        game.board.add(2, 7, false);
        game.board.add(3, 7, false);
        game.board.add(4, 7, false);
        game.board.add(0, 8, false);
        game.board.add(1, 8, false);
        game.board.add(2, 8, false);
        game.board.add(3, 8, false);
        game.board.add(4, 8, false);
        game.board.add(0, 9, false);
        game.board.add(1, 9, false);
        game.board.add(2, 9, false);
        game.board.add(3, 9, false);
        game.board.add(4, 9, false);

        println!("{}", game);
        game.piece_left();
        game.piece_soft_drop();
        game.piece_right();
        game.piece_rotate_ccw();
        println!("{}", game);

        assert_eq!([Point { row: 0, col: 4 }, Point { row: 1, col: 4 }, Point { row: 1, col: 5 }, Point { row: 2, col: 5 }],
                   game.active_piece.abs_locations().unwrap());

        // j spin 1
        let mut game = Game::new(None);
        game.active_piece = Placement::new(5);

        game.board.add(0, 0, false);
        game.board.add(1, 0, false);
        game.board.add(2, 0, false);
        game.board.add(0, 1, false);
        game.board.add(1, 1, false);
        game.board.add(2, 1, false);
        game.board.add(0, 2, false);
        game.board.add(1, 2, false);
        game.board.add(2, 2, false);
        game.board.add(2, 3, false);
        game.board.add(1, 4, false);
        game.board.add(2, 4, false);

        println!("{}", game);
        game.piece_right();
        game.piece_right();
        game.piece_rotate_ccw();
        game.piece_soft_drop();
        game.piece_left();
        game.piece_rotate_cw();
        println!("{}", game);

        assert_eq!([Point { row: 1, col: 3 }, Point { row: 0, col: 3 }, Point { row: 0, col: 4 }, Point { row: 0, col: 5 }],
                   game.active_piece.abs_locations().unwrap());

        // j spin triple
        let mut game = Game::new(None);
        game.active_piece = Placement::new(5);

        game.board.add(0, 0, false);
        game.board.add(1, 0, false);
        game.board.add(2, 0, false);
        game.board.add(3, 0, false);
        game.board.add(4, 0, false);
        game.board.add(5, 0, false);
        game.board.add(0, 1, false);
        game.board.add(1, 1, false);
        game.board.add(2, 1, false);
        game.board.add(3, 1, false);
        game.board.add(4, 1, false);
        game.board.add(5, 1, false);
        game.board.add(0, 2, false);
        game.board.add(1, 2, false);
        game.board.add(2, 2, false);
        game.board.add(3, 2, false);
        game.board.add(4, 2, false);
        game.board.add(5, 2, false);
        game.board.add(0, 3, false);
        game.board.add(1, 3, false);
        game.board.add(2, 3, false);
        game.board.add(1, 4, false);
        game.board.add(2, 4, false);
        game.board.add(4, 5, false);
        game.board.add(0, 6, false);
        game.board.add(1, 6, false);
        game.board.add(2, 6, false);
        game.board.add(3, 6, false);
        game.board.add(4, 6, false);
        game.board.add(0, 7, false);
        game.board.add(1, 7, false);
        game.board.add(2, 7, false);
        game.board.add(3, 7, false);
        game.board.add(4, 7, false);
        game.board.add(0, 8, false);
        game.board.add(1, 8, false);
        game.board.add(2, 8, false);
        game.board.add(3, 8, false);
        game.board.add(4, 8, false);
        game.board.add(0, 9, false);
        game.board.add(1, 9, false);
        game.board.add(2, 9, false);
        game.board.add(3, 9, false);
        game.board.add(4, 9, false);

        println!("{}", game);
        game.piece_left();
        game.piece_rotate_cw();
        game.piece_soft_drop();
        game.piece_rotate_ccw();
        game.piece_rotate_ccw();
        println!("{}", game);

        assert_eq!([Point { row: 0, col: 4 }, Point { row: 0, col: 5 }, Point { row: 1, col: 5 }, Point { row: 2, col: 5 }],
                   game.active_piece.abs_locations().unwrap());

        // l spin 180 (?)
        let mut game = Game::new(None);
        game.active_piece = Placement::new(1);

        game.board.add(0, 0, false);
        game.board.add(1, 0, false);
        game.board.add(2, 0, false);
        game.board.add(3, 0, false);
        game.board.add(4, 0, false);
        game.board.add(5, 0, false);
        game.board.add(6, 0, false);
        game.board.add(0, 1, false);
        game.board.add(1, 1, false);
        game.board.add(2, 1, false);
        game.board.add(3, 1, false);
        game.board.add(4, 1, false);
        game.board.add(5, 1, false);
        game.board.add(6, 1, false);
        game.board.add(0, 2, false);
        game.board.add(1, 2, false);
        game.board.add(2, 2, false);
        game.board.add(3, 2, false);
        game.board.add(4, 2, false);
        game.board.add(5, 2, false);
        game.board.add(6, 2, false);
        game.board.add(0, 3, false);
        game.board.add(1, 3, false);
        game.board.add(2, 3, false);
        game.board.add(3, 3, false);
        game.board.add(4, 3, false);
        game.board.add(5, 3, false);
        game.board.add(6, 3, false);
        game.board.add(4, 4, false);
        game.board.add(1, 5, false);
        game.board.add(4, 5, false);
        game.board.add(0, 6, false);
        game.board.add(1, 6, false);
        game.board.add(2, 6, false);
        game.board.add(4, 6, false);
        game.board.add(0, 7, false);
        game.board.add(1, 7, false);
        game.board.add(2, 7, false);
        game.board.add(0, 8, false);
        game.board.add(1, 8, false);
        game.board.add(2, 8, false);
        game.board.add(3, 8, false);
        game.board.add(4, 8, false);
        game.board.add(5, 8, false);
        game.board.add(0, 9, false);
        game.board.add(1, 9, false);
        game.board.add(2, 9, false);
        game.board.add(3, 9, false);
        game.board.add(4, 9, false);
        game.board.add(5, 9, false);

        println!("{}", game);
        game.piece_rotate_ccw();
        game.piece_right();
        game.piece_right();
        game.piece_right();
        game.piece_soft_drop();
        game.piece_rotate_cw();
        game.piece_rotate_180();
        game.piece_left();
        game.piece_rotate_ccw();
        println!("{}", game);

        assert_eq!([Point { row: 0, col: 5 }, Point { row: 2, col: 4 }, Point { row: 1, col: 4 }, Point { row: 0, col: 4 }],
                   game.active_piece.abs_locations().unwrap());

        // l spin fuckery
        let mut game = Game::new(None);
        game.active_piece = Placement::new(1);

        game.board.clear();
        game.board.add(0, 0, false);
        game.board.add(1, 0, false);
        game.board.add(2, 0, false);
        game.board.add(3, 0, false);
        game.board.add(4, 0, false);
        game.board.add(5, 0, false);
        game.board.add(6, 0, false);
        game.board.add(7, 0, false);
        game.board.add(8, 0, false);
        game.board.add(9, 0, false);
        game.board.add(10, 0, false);
        game.board.add(11, 0, false);
        game.board.add(12, 0, false);
        game.board.add(13, 0, false);
        game.board.add(14, 0, false);
        game.board.add(4, 1, false);
        game.board.add(5, 1, false);
        game.board.add(6, 1, false);
        game.board.add(7, 1, false);
        game.board.add(8, 1, false);
        game.board.add(9, 1, false);
        game.board.add(10, 1, false);
        game.board.add(11, 1, false);
        game.board.add(12, 1, false);
        game.board.add(14, 1, false);
        game.board.add(1, 2, false);
        game.board.add(2, 2, false);
        game.board.add(5, 2, false);
        game.board.add(6, 2, false);
        game.board.add(7, 2, false);
        game.board.add(8, 2, false);
        game.board.add(9, 2, false);
        game.board.add(0, 3, false);
        game.board.add(1, 3, false);
        game.board.add(6, 3, false);
        game.board.add(7, 3, false);
        game.board.add(8, 3, false);
        game.board.add(9, 3, false);
        game.board.add(11, 3, false);
        game.board.add(12, 3, false);
        game.board.add(0, 4, false);
        game.board.add(1, 4, false);
        game.board.add(3, 4, false);
        game.board.add(4, 4, false);
        game.board.add(6, 4, false);
        game.board.add(9, 4, false);
        game.board.add(12, 4, false);
        game.board.add(0, 5, false);
        game.board.add(1, 5, false);
        game.board.add(2, 5, false);
        game.board.add(3, 5, false);
        game.board.add(4, 5, false);
        game.board.add(12, 5, false);
        game.board.add(0, 6, false);
        game.board.add(1, 6, false);
        game.board.add(2, 6, false);
        game.board.add(3, 6, false);
        game.board.add(4, 6, false);
        game.board.add(5, 6, false);
        game.board.add(6, 6, false);
        game.board.add(7, 6, false);
        game.board.add(9, 6, false);
        game.board.add(10, 6, false);
        game.board.add(11, 6, false);
        game.board.add(12, 6, false);
        game.board.add(0, 7, false);
        game.board.add(1, 7, false);
        game.board.add(2, 7, false);
        game.board.add(3, 7, false);
        game.board.add(4, 7, false);
        game.board.add(5, 7, false);
        game.board.add(6, 7, false);
        game.board.add(7, 7, false);
        game.board.add(9, 7, false);
        game.board.add(10, 7, false);
        game.board.add(11, 7, false);
        game.board.add(12, 7, false);
        game.board.add(0, 8, false);
        game.board.add(1, 8, false);
        game.board.add(2, 8, false);
        game.board.add(3, 8, false);
        game.board.add(4, 8, false);
        game.board.add(5, 8, false);
        game.board.add(6, 8, false);
        game.board.add(7, 8, false);
        game.board.add(8, 8, false);
        game.board.add(9, 8, false);
        game.board.add(10, 8, false);
        game.board.add(11, 8, false);
        game.board.add(12, 8, false);
        game.board.add(0, 9, false);
        game.board.add(1, 9, false);
        game.board.add(2, 9, false);
        game.board.add(3, 9, false);
        game.board.add(4, 9, false);
        game.board.add(5, 9, false);
        game.board.add(6, 9, false);
        game.board.add(7, 9, false);
        game.board.add(8, 9, false);
        game.board.add(9, 9, false);
        game.board.add(10, 9, false);
        game.board.add(11, 9, false);
        game.board.add(12, 9, false);

        println!("{}", game);
        game.piece_soft_drop();
        game.piece_left();
        game.piece_left();
        game.piece_rotate_cw();
        game.piece_rotate_ccw();
        game.piece_right();
        game.piece_rotate_ccw();
        game.piece_soft_drop();
        game.piece_rotate_180();
        game.piece_rotate_cw();
        game.piece_left();
        game.piece_rotate_cw();
        game.piece_rotate_cw();
        game.piece_rotate_cw();
        game.piece_rotate_180();
        game.piece_rotate_ccw();
        game.piece_rotate_ccw();
        println!("{}", game);

        assert_eq!([Point { row: 0, col: 2 }, Point { row: 2, col: 1 }, Point { row: 1, col: 1 }, Point { row: 0, col: 1 }],
                   game.active_piece.abs_locations().unwrap());

    }

    fn test_add_garbage() {}
}