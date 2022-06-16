use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};

use crate::board::*;
use crate::errors::GameError;
use crate::placement::*;
use crate::queue::*;

use crate::placement::piece_data::offset::*;
use crate::placement::piece_data::rotation::*;
use crate::placement::piece_data::*;

#[derive(Serialize, Deserialize)]
struct JsonBoard {
    board: String,
    kind: String,
}

#[derive(Serialize, Deserialize)]
struct JsonPieceQueue {
    queue: String,
    kind: String,
}

#[derive(Default)]
pub struct Game {
    pub board: Board,

    piece_queue: PieceQueue,
    garbage_queue: GarbageQueue,

    last_piece_cleared: bool,

    pub game_data: GameData,
    pub game_rules: GameRules,

    pub active_piece: Placement,
    pub hold_piece: Option<Piece>,

    pub game_over: bool,
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Queue: {}", self.piece_queue)?;
        if let Some(hold) = self.hold_piece {
            write!(f, "Hold: {}\n", PieceQueue::int_to_piece(hold))?;
        } else {
            write!(f, "Hold: None\n")?;
        }

        write!(f, "{}", self.board.to_string(&self.active_piece))?;
        write!(f, "{}", self.game_data.to_string())?;

        Ok(())
    }
}

impl Game {
    pub fn get_board_string(&self) -> String {
        self.board.to_string(&self.active_piece)
    }

    pub fn get_board_json(&self) -> Value {
        json!(JsonBoard {
            kind: String::from("board"),
            board: self.get_board_string()
        })
    }

    pub fn get_piece_queue(&self) -> &PieceQueue {
        &self.piece_queue
    }

    pub fn get_piece_queue_json(&self) -> Value {
        json!(JsonPieceQueue {
            kind: String::from("piecequeue"),
            queue: self.get_piece_queue().to_string()
        })
    }

    pub fn get_active_piece_type(&self) -> Piece {
        self.active_piece.piece_type
    }

    pub fn reset_active_piece(&mut self) {
        self.active_piece = Placement::new(self.get_active_piece_type())
    }

    pub fn new(seed: Option<usize>) -> Self {
        let mut out = Self {
            piece_queue: PieceQueue::new(seed),
            ..Default::default()
        };

        out.active_piece = Placement::new(out.piece_queue.next());
        out
    }

    pub fn create(
        seed: usize,
        bagtype: &str,
        allow_180: bool,
        allow_hard_drop: bool,
        allow_b2b_chain: bool,
        max_board_height: usize,
        kickset: &str,
        spinbonus: &str,
    ) -> Self {
        let mut out = Self {
            piece_queue: PieceQueue::new(Some(seed)),
            game_rules: GameRules {
                seed,
                bag_type: match bagtype {
                    "7-bag" => BagType::SevenBag,
                    "14-bag" => BagType::FourteenBag,
                    "classic" => BagType::Classic,
                    "pairs" => BagType::Pairs,
                    "total mayhem" => BagType::Mayhem,
                    other => (|| {
                        eprintln!("unknown bagtype '{}'", other);
                        BagType::SevenBag
                    })(),
                },
                allow_180,
                allow_hard_drop,
                allow_b2b_chain,
                max_board_height,
                kick_set: match kickset {
                    "SRS+" => KickSet::SRSPlus,
                    "SRS" => KickSet::SRS,
                    "SRS-X" => KickSet::SRSX,
                    "TETRA-X" => KickSet::TetraX,
                    "NRS" => KickSet::NRS,
                    "ARS" => KickSet::ARS,
                    "ASC" => KickSet::ASC,
                    "none" => KickSet::None,
                    other => (|| {
                        eprintln!("unknown kickset '{}'", other);
                        KickSet::SRSPlus
                    })(),
                },
                spin_bonus: match spinbonus {
                    "T-spins" => SpinBonus::TSpin,
                    "all" => SpinBonus::All,
                    "stupid" => SpinBonus::Stupid,
                    "none" => SpinBonus::None,
                    other => (|| {
                        eprintln!("unknown spinbonus '{}'", other);
                        SpinBonus::TSpin
                    })(),
                },
            },
            ..Default::default()
        };

        out.active_piece = Placement::new(out.piece_queue.next());
        out
    }

    pub fn valid_location_for_active(&self) -> bool {
        self.board.check_valid_location(&self.active_piece).is_ok()
    }

    pub fn valid_placement_for_active(&mut self) -> bool {
        self.board
            .check_valid_placement(&mut self.active_piece)
            .is_ok()
    }

    pub fn set_piece(&mut self, update_heights: bool) -> Result<(), GameError> {
        self.board.top_out(
            &self.active_piece,
            &Placement::new(self.piece_queue.peek()),
            self.game_rules.max_board_height,
        )?;

        self.board.set_piece(&self.active_piece, update_heights);
        self.active_piece = Placement::new(self.piece_queue.next());

        Ok(())
    }

    fn safe_move_active_piece_by_vector(&mut self, v: MoveVector) -> bool {
        //  checks if center is invalid
        if !self.active_piece.move_by_vector(v) {
            return false;
        }

        // checks if rest of piece is invalid
        let safe = self.board.check_valid_location(&self.active_piece).is_ok();

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

    pub fn piece_das_left(&mut self) -> bool {
        while self.piece_left() {
            // this is intended wheeee
        }
        true
    }

    pub fn piece_das_right(&mut self) -> bool {
        while self.piece_right() {
            // this is intended wheeee
        }
        true
    }

    pub fn piece_soft_drop(&mut self) -> bool {
        let out = self.piece_down();

        while self.piece_down() {
            // this is intended wheee
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
                if self.board.check_valid_location(&self.active_piece).is_ok() {
                    self.active_piece.last_kick = index;
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
        if self.active_piece.piece_type == 4 {
            // I piece is the special child
            if dir == 2 {
                kicks = FIVE_180_OFFSETS[before].to_vec()
            } else {
                kicks = FIVE_OFFSETS[before][dir / 2].to_vec();
            }
        } else if self.active_piece.piece_type == 2 {
            // O piece is the other special child
            kicks = vec![O_OFFSETS[before][dir - 1]];
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

    pub fn piece_hard_drop(&mut self, update_heights: bool) -> Result<(), GameError> {
        self.piece_soft_drop();
        self.set_piece(update_heights)?;

        let lines_cleared = self.board.clear_lines(update_heights);

        let t_spin_type = self.board.get_t_spin_type(self.active_piece);
        self.game_data
            .update(lines_cleared, t_spin_type, self.board.all_clear());

        Ok(())
    }

    pub fn add_garbage_to_queue(&mut self, amt: usize) {
        todo!()
    }

    pub fn add_garbage_to_board(&mut self, amt: usize, update_heights: bool) {
        self.board
            .add_garbage(GarbageItem::new(amt), update_heights);
    }

    fn reset(&mut self) {
        unimplemented!()
    }

    pub fn hold(&mut self) -> bool {
        // TODO: refactor maybe

        let first_hold = self.hold_piece.is_none();
        let active_type = self.active_piece.piece_type;

        if first_hold {
            self.active_piece = Placement::new(self.piece_queue.next());
        } else {
            self.active_piece = Placement::new(self.hold_piece.unwrap());
        }

        self.hold_piece = Some(active_type);
        true
    }

    fn manual_set_queue(&mut self, new_queue: VecDeque<usize>) {
        self.piece_queue.manual_queue_set(new_queue)
    }

    pub fn add(&mut self, row: usize, col: usize, update_heights: bool) {
        self.board.add(row, col, update_heights);
    }

    pub fn piece_queue_peek(&self) -> Piece {
        self.piece_queue.peek()
    }
}

pub enum KickSet {
    None,
    SRSPlus,
    SRS,
    SRSX,
    TetraX,
    NRS,
    ARS,
    ASC,
}

impl Default for KickSet {
    fn default() -> Self {
        KickSet::SRSPlus
    }
}

pub enum SpinBonus {
    TSpin,
    All,
    Stupid,
    None,
}

impl Default for SpinBonus {
    fn default() -> Self {
        SpinBonus::TSpin
    }
}

pub struct GameRules {
    pub seed: usize,
    bag_type: BagType,
    allow_180: bool,
    allow_hard_drop: bool,
    allow_b2b_chain: bool,
    max_board_height: usize,
    kick_set: KickSet,
    spin_bonus: SpinBonus,
}

impl Default for GameRules {
    fn default() -> Self {
        Self {
            seed: 0,
            bag_type: BagType::SevenBag,
            allow_180: true,
            allow_hard_drop: true,
            allow_b2b_chain: true,
            max_board_height: 20,
            kick_set: Default::default(),
            spin_bonus: Default::default(),
        }
    }
}

impl GameRules {
    fn new(
        seed: usize,
        bag_type: BagType,
        allow_180: bool,
        allow_hard_drop: bool,
        allow_b2b_chain: bool,
        max_board_height: usize,
        kick_set: KickSet,
        spin_bonus: SpinBonus,
    ) -> Self {
        Self {
            seed,
            bag_type,
            allow_180,
            allow_hard_drop,
            allow_b2b_chain,
            max_board_height,
            kick_set,
            spin_bonus,
        }
    }

    fn new_seed(seed: usize) -> Self {
        Self {
            seed,
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct GameData {
    pub all_clear: bool,
    pub combo: i8,
    pub b2b: i8,

    pub pieces_placed: u8,
    pub lines_cleared: usize,
    pub lines_sent: u8,

    pub game_over: bool,
    init_time: f32,
}

impl Display for GameData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "All Clear: {}\n", self.all_clear)?;
        write!(f, "Combo: {}\n", self.combo)?;
        write!(f, "Back to Back: {}\n", self.b2b)?;

        write!(f, "Pieces Placed: {}\n", self.pieces_placed)?;

        write!(f, "Lines Cleared: {}\n", self.lines_cleared)?;
        write!(f, "Lines Sent: {}", self.lines_sent)?;

        Ok(())
    }
}

impl GameData {
    fn run_time() -> f32 {
        unimplemented!()
    }

    fn new(init_time: f32) -> Self {
        Self {
            init_time,
            ..Default::default()
        }
    }

    fn update(&mut self, lines_cleared: usize, t_spin_type: TSpinType, all_clear: bool) {
        self.pieces_placed += 1;

        if lines_cleared == 0 {
            self.combo = 0;
            self.all_clear = false;
            return;
        }

        self.lines_cleared += lines_cleared;

        // update lines sent before adding b2b/combo
        self.lines_sent += 1;

        let b2b = (t_spin_type != TSpinType::None) || (lines_cleared == 4);
        if b2b {
            self.b2b += 1;
        } else {
            self.b2b = 0;
        }
        self.combo += 1;

        self.all_clear = all_clear;
    }
}

mod damage_calculations {
    use crate::board::TSpinType;

    const D_T_Q_MULTIPLIER: [f32; 3] = [0.25, 0.5, 1.0];

    const S_ATTACKS: [u8; 20] = [0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3];
    const TM_ATTACKS: [[u8; 2]; 20] = [
        [0, 1],
        [0, 1],
        [1, 1],
        [1, 1],
        [1, 2],
        [1, 2],
        [2, 2],
        [2, 2],
        [2, 3],
        [2, 3],
        [2, 3],
        [2, 3],
        [2, 4],
        [2, 4],
        [2, 4],
        [2, 4],
        [3, 5],
        [3, 5],
        [3, 5],
        [3, 5],
    ];

    pub fn calculate_damage(
        lines_cleared: usize,
        t_spin: TSpinType,
        b2b: u8,
        combo: u8,
        all_clear: bool,
    ) -> u8 {
        let multiplier = 1.0;
        let all_clear_damage = all_clear as u8 * 10;

        ((combo as f32 + 4.0) * b2b as f32 * multiplier) as u8 + all_clear_damage
    }

    fn single(combo: u8) -> u8 {
        S_ATTACKS[combo as usize]
    }

    fn mini(combo: u8, lines_cleared: usize) -> u8 {
        TM_ATTACKS[combo as usize][lines_cleared - 2]
    }

    fn double_triple(combo: u8, lines_cleared: usize) -> u8 {
        (D_T_Q_MULTIPLIER[lines_cleared - 2] * (combo as f32 + 4.0)) as u8
    }

    fn other(combo: u8, lines_cleared: usize, t_spin_full: bool, b2b: usize) {
        let mut b2b_multiplier = 1.0;
        match b2b {
            0 => b2b_multiplier = 1.0,
            1..=3 => b2b_multiplier = 1.25,
            4..=8 => b2b_multiplier = 1.5,
            9..=24 => b2b_multiplier = 1.75,
            25..=67 => b2b_multiplier = 2.0,
            68..=185 => b2b_multiplier = 2.25,
            186..=504 => b2b_multiplier = 2.5,
            505..=1370 => b2b_multiplier = 2.75,
            _ => b2b_multiplier = 3.0,
        }

        (D_T_Q_MULTIPLIER[lines_cleared - 2] * b2b_multiplier) as u8 * (combo + 4);
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
        game.set_piece(true).expect("crash and burn");

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

        assert!(game.set_piece(true).is_err());

        let mut game = Game::new(None);
        game.piece_soft_drop();

        assert!(game.set_piece(true).is_ok());
    }

    #[test]
    fn test_move_piece() {
        let mut game = Game::new(Some(1337));
        // OISTLJZ

        game.piece_das_right();
        game.piece_down();

        assert_eq!(
            game.active_piece.abs_locations().unwrap(),
            [
                Point { row: 21, col: 8 },
                Point { row: 21, col: 9 },
                Point { row: 20, col: 8 },
                Point { row: 20, col: 9 }
            ]
        );

        game.piece_down();
        game.set_piece(false).expect("crash and burn");
        game.piece_soft_drop();

        assert_eq!(
            game.active_piece.abs_locations().unwrap(),
            [
                Point { row: 0, col: 3 },
                Point { row: 0, col: 4 },
                Point { row: 0, col: 5 },
                Point { row: 0, col: 6 }
            ]
        );
    }

    #[test]
    fn test_rotate_piece() {
        let mut game = Game::new(Some(1336));
        // TLJSO

        game.piece_rotate_cw();

        assert_eq!(
            game.active_piece.abs_locations().unwrap(),
            [
                Point { row: 21, col: 5 },
                Point { row: 22, col: 4 },
                Point { row: 21, col: 4 },
                Point { row: 20, col: 4 }
            ]
        );

        game.piece_rotate_cw();

        assert_eq!(
            game.active_piece.abs_locations().unwrap(),
            [
                Point { row: 20, col: 4 },
                Point { row: 21, col: 5 },
                Point { row: 21, col: 4 },
                Point { row: 21, col: 3 }
            ]
        );

        game.piece_rotate_ccw();

        assert_eq!(
            game.active_piece.abs_locations().unwrap(),
            [
                Point { row: 21, col: 5 },
                Point { row: 22, col: 4 },
                Point { row: 21, col: 4 },
                Point { row: 20, col: 4 }
            ]
        );

        game.piece_soft_drop();
        game.set_piece(false).expect("crash and burn 2");

        game.piece_rotate_180();

        assert_eq!(
            game.active_piece.abs_locations().unwrap(),
            [
                Point { row: 20, col: 3 },
                Point { row: 21, col: 5 },
                Point { row: 21, col: 4 },
                Point { row: 21, col: 3 }
            ]
        );
    }

    #[test]
    fn test_hold() {
        let mut game = Game::new(Some(1337));
        // OISTLJZ

        println!("{}", game);
        assert_eq!(game.active_piece.piece_type, 2);
        assert!(game.hold_piece.is_none());
        game.hold();
        println!("{}", game);
        assert_eq!(game.active_piece.piece_type, 4);
        assert_eq!(game.hold_piece.unwrap(), 2);

        game.piece_soft_drop();
        game.set_piece(false).expect("crash and burn");

        println!("{}", game);

        game.hold();
        assert_eq!(game.hold_piece.unwrap(), 3);
        assert_eq!(game.active_piece.piece_type, 2);
    }

    #[test]
    fn test_wall_kick() {
        let mut game = Game::new(Some(1336));
        // TLJSO

        game.piece_rotate_cw();
        game.piece_das_left();

        assert!(game.piece_rotate_ccw());

        assert_eq!(
            game.active_piece.abs_locations().unwrap(),
            [
                Point { row: 22, col: 1 },
                Point { row: 21, col: 0 },
                Point { row: 21, col: 1 },
                Point { row: 21, col: 2 }
            ]
        );

        let mut game = Game::new(Some(1337));
        game.piece_soft_drop();
        game.set_piece(false).expect("die");

        // I piece
        game.piece_rotate_ccw();
        game.piece_das_right();

        assert!(game.piece_rotate_ccw());
        assert_eq!(
            game.active_piece.abs_locations().unwrap(),
            [
                Point { row: 20, col: 9 },
                Point { row: 20, col: 8 },
                Point { row: 20, col: 7 },
                Point { row: 20, col: 6 }
            ]
        );
    }

    #[test]
    fn test_floor_kick() {
        let mut game = Game::new(Some(1336));

        game.piece_soft_drop();

        assert!(game.piece_rotate_cw());

        println!("{} {:?}", game, game.active_piece.abs_locations());

        assert_eq!(
            game.active_piece.abs_locations().unwrap(),
            [
                Point { row: 1, col: 4 },
                Point { row: 2, col: 3 },
                Point { row: 1, col: 3 },
                Point { row: 0, col: 3 }
            ]
        );
    }

    #[test]
    fn test_hard_drop() {
        let mut game = Game::new(Some(1337));

        game.piece_hard_drop(true).expect("die");
        game.piece_hard_drop(true).expect("die");

        assert_eq!(game.active_piece.piece_type, 3);
        game.piece_soft_drop();

        assert_eq!(
            game.active_piece.abs_locations().unwrap(),
            [
                Point { row: 4, col: 4 },
                Point { row: 4, col: 5 },
                Point { row: 3, col: 3 },
                Point { row: 3, col: 4 }
            ]
        );
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

        assert_eq!(
            game.active_piece.abs_locations().unwrap(),
            [
                Point { row: 0, col: 5 },
                Point { row: 0, col: 4 },
                Point { row: 1, col: 4 },
                Point { row: 1, col: 3 }
            ]
        );

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

        assert_eq!(
            [
                Point { row: 0, col: 6 },
                Point { row: 0, col: 5 },
                Point { row: 1, col: 5 },
                Point { row: 1, col: 4 }
            ],
            game.active_piece.abs_locations().unwrap()
        );

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

        assert_eq!(
            [
                Point { row: 0, col: 5 },
                Point { row: 0, col: 4 },
                Point { row: 1, col: 6 },
                Point { row: 1, col: 5 }
            ],
            game.active_piece.abs_locations().unwrap()
        );

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

        assert_eq!(
            [
                Point { row: 0, col: 4 },
                Point { row: 0, col: 3 },
                Point { row: 1, col: 5 },
                Point { row: 1, col: 4 }
            ],
            game.active_piece.abs_locations().unwrap()
        );

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

        assert_eq!(
            [
                Point { row: 0, col: 3 },
                Point { row: 1, col: 5 },
                Point { row: 1, col: 4 },
                Point { row: 1, col: 3 }
            ],
            game.active_piece.abs_locations().unwrap()
        );

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

        assert_eq!(
            [
                Point { row: 0, col: 4 },
                Point { row: 1, col: 6 },
                Point { row: 1, col: 5 },
                Point { row: 1, col: 4 }
            ],
            game.active_piece.abs_locations().unwrap()
        );

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

        assert_eq!(
            [
                Point { row: 1, col: 6 },
                Point { row: 0, col: 4 },
                Point { row: 0, col: 5 },
                Point { row: 0, col: 6 }
            ],
            game.active_piece.abs_locations().unwrap()
        );

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

        assert_eq!(
            [
                Point { row: 1, col: 4 },
                Point { row: 0, col: 4 },
                Point { row: 0, col: 5 },
                Point { row: 0, col: 6 }
            ],
            game.active_piece.abs_locations().unwrap()
        );

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

        assert_eq!(
            [
                Point { row: 0, col: 6 },
                Point { row: 1, col: 6 },
                Point { row: 1, col: 5 },
                Point { row: 1, col: 4 }
            ],
            game.active_piece.abs_locations().unwrap()
        );

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

        assert_eq!(
            [
                Point { row: 0, col: 5 },
                Point { row: 1, col: 5 },
                Point { row: 1, col: 4 },
                Point { row: 1, col: 3 }
            ],
            game.active_piece.abs_locations().unwrap()
        );

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

        assert_eq!(
            [
                Point { row: 1, col: 3 },
                Point { row: 2, col: 3 },
                Point { row: 0, col: 4 },
                Point { row: 1, col: 4 }
            ],
            game.active_piece.abs_locations().unwrap()
        );

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

        assert_eq!(
            [
                Point { row: 1, col: 5 },
                Point { row: 0, col: 5 },
                Point { row: 2, col: 4 },
                Point { row: 1, col: 4 }
            ],
            game.active_piece.abs_locations().unwrap()
        );

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

        assert_eq!(
            [
                Point { row: 1, col: 4 },
                Point { row: 0, col: 4 },
                Point { row: 2, col: 3 },
                Point { row: 1, col: 3 }
            ],
            game.active_piece.abs_locations().unwrap()
        );

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

        assert_eq!(
            [
                Point { row: 0, col: 4 },
                Point { row: 1, col: 4 },
                Point { row: 1, col: 5 },
                Point { row: 2, col: 5 }
            ],
            game.active_piece.abs_locations().unwrap()
        );

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

        assert_eq!(
            [
                Point { row: 1, col: 3 },
                Point { row: 0, col: 3 },
                Point { row: 0, col: 4 },
                Point { row: 0, col: 5 }
            ],
            game.active_piece.abs_locations().unwrap()
        );

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

        assert_eq!(
            [
                Point { row: 0, col: 4 },
                Point { row: 0, col: 5 },
                Point { row: 1, col: 5 },
                Point { row: 2, col: 5 }
            ],
            game.active_piece.abs_locations().unwrap()
        );

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

        assert_eq!(
            [
                Point { row: 0, col: 5 },
                Point { row: 2, col: 4 },
                Point { row: 1, col: 4 },
                Point { row: 0, col: 4 }
            ],
            game.active_piece.abs_locations().unwrap()
        );

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

        assert_eq!(
            [
                Point { row: 0, col: 2 },
                Point { row: 2, col: 1 },
                Point { row: 1, col: 1 },
                Point { row: 0, col: 1 }
            ],
            game.active_piece.abs_locations().unwrap()
        );
    }

    #[test]
    fn test_add_garbage() {
        let mut game = Game::new(Some(1337));

        game.piece_soft_drop();
        game.set_piece(true).expect("die");

        game.add_garbage_to_board(3, false);

        // testing if stuff moves up properly
        for location in [
            Point { row: 4, col: 4 },
            Point { row: 4, col: 5 },
            Point { row: 3, col: 4 },
            Point { row: 3, col: 5 },
        ] {
            assert!(game.board.get(location.row, location.col));
        }

        let mut counter = 0;

        for row in 0..3 {
            for col in 0..10 {
                if game.board.get(row, col) {
                    counter += 1;
                }
            }
        }

        assert_eq!(counter, 27);

        // assumes garbage is in a random column. this is tested in garbage item
    }
}
