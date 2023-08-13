#![allow(dead_code)]

use crate::board::Board;
use crate::constants::piece_constants::{NUM_ROTATE_STATES, RELATIVE_CORNERS};
use crate::constants::types::{PieceType, RotationDirection};
use crate::constants::versus_constants::*;
use crate::constants::queue_constants::*;
use crate::piece::Piece;
use crate::point_vector::PointVector;
use crate::queue::{piece_type_to_string, piece_type_to_string_colored, BagType, PieceQueue};
use crate::versus::*;
use game_rules_and_data::*;
use std::fmt::{Display, Formatter};
use crate::game::game_rules_and_data::SpinBonus::TSpin;
use colored::*;
use crate::constants::localbotgameplay::*;
use std::fs;
use crate::constants::board_constants::MAX_PLACE_HEIGHT;
use crate::constants::queue_constants::MIN_QUEUE_LENGTH;
use num::clamp;

#[derive(Default, Clone)]
pub struct Game {
    pub board: Board,
    pub piece_queue: PieceQueue,
    pub game_data: GameData,
    game_rules: GameRules,

    pub active_piece: Piece,
    pub hold_piece: Option<PieceType>,
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if CONSOLE_DISPLAY_QUEUE {
            write!(f, "Queue: {}", self.piece_queue)?;
            if let Some(hold) = self.hold_piece {
                write!(f, "Hold: {}\n", piece_type_to_string_colored(hold))?;
            } else {
                write!(f, "Hold: None\n")?;
            }
        }

        write!(f, "{}", self.board.display_with_active(&self.active_piece))?;
        // write!(f, "{}", self.game_data.to_string())?;
        Ok(())
    }
}

impl Game {
    // init
    pub fn new(seed: Option<usize>) -> Self {
        let mut out = Self {
            piece_queue: PieceQueue::new(seed),
            ..Default::default()
        };

        out.active_piece = out.piece_queue.next();
        out
    }

    pub fn from_rules(seed: Option<usize>, game_rules: GameRules) -> Self {
        let mut out = Self {
            piece_queue: PieceQueue::new(seed),
            game_rules,
            ..Default::default()
        };

        out.active_piece = out.piece_queue.next();
        out
    }
    // piece getters and setters
    pub fn get_active_piece(&self) -> &Piece {
        &self.active_piece
    }

    pub fn get_hold_piece(&self) -> Option<PieceType> {
        self.hold_piece
    }

    pub fn get_hold_piece_or_next(&self) -> Piece {
        return if let Some(piece) = self.hold_piece {
            Piece::new(piece)
        } else {
            Piece::new(self.piece_queue.peek())
        };
    }

    pub fn set_active_piece(&mut self, new_piece: Piece) {
        self.active_piece = new_piece;
    }

    // game over
    pub fn get_game_over(&self) -> bool {
        self.game_data.game_over
    }

    pub fn set_game_over(&mut self, game_over: bool) {
        self.game_data.game_over = game_over;
    }

    // safe piece movements
    pub fn active_left(&mut self) -> bool {
        self.active_piece.set_kick(999);
        Game::move_piece(&mut self.active_piece, &self.board, PointVector(0, -1))
    }

    pub fn ret_active_left(&mut self) -> Option<Piece> {
        Game::ret_move_piece(&self.active_piece.clone(), &self.board, PointVector(0, -1))
    }

    pub fn active_right(&mut self) -> bool {
        self.active_piece.set_kick(999);
        Game::move_piece(&mut self.active_piece, &self.board, PointVector(0, 1))
    }

    pub fn ret_active_right(&mut self) -> Option<Piece> {
        Game::ret_move_piece(&self.active_piece.clone(), &self.board, PointVector(0, 1))
    }

    fn active_down(&mut self) -> bool {
        Game::move_piece(&mut self.active_piece, &self.board, PointVector(-1, 0))
    }

    pub fn active_drop(&mut self) -> bool {
        let out = self.active_down();
        if out { self.active_piece.set_kick(999)}
        while out && self.active_down() {}
        out
    }

    pub fn ret_active_drop(&mut self) -> Piece {
        // TODO: make not mut
        let save = self.active_piece.clone();
        self.active_drop();
        let out = self.active_piece.clone();
        self.active_piece = save;
        out
    }

    fn move_piece(p: &mut Piece, b: &Board, v: PointVector) -> bool {
        if p.moved(v) {
            if b.piece_valid_location(p) {
                return true;
            }
            p.moved(v.negative());
        }
        false
    }

    fn ret_move_piece(p: &Piece, b: &Board, v: PointVector) -> Option<Piece> {
        if let Some(piece) = p.ret_moved(v) {
            if b.piece_valid_location(&piece) {
                return Some(piece);
            }
        };
        None
    }

    // safe piece ROTATIONS
    pub fn active_cw(&mut self) -> bool {
        self.active_piece_rotate_direction(1)
    }

    pub fn ret_active_cw(&mut self) -> Option<Piece> {
        Game::ret_rotate_piece(&self.active_piece, &self.board, 1)
    }

    pub fn active_180(&mut self) -> bool {
        self.active_piece_rotate_direction(2)
    }

    pub fn ret_active_180(&mut self) -> Option<Piece> {
        Game::ret_rotate_piece(&self.active_piece, &self.board, 2)
    }

    pub fn active_ccw(&mut self) -> bool {
        // println!("CCW REACHED");
        let out = self.active_piece_rotate_direction(3);
        // println!("ccw? {}", out);
        out
    }

    pub fn ret_active_ccw(&mut self) -> Option<Piece> {
        Game::ret_rotate_piece(&self.active_piece, &self.board, 3)
    }

    pub fn active_piece_rotate_direction(&mut self, direction: RotationDirection) -> bool {
        Game::rotate_piece(&mut self.active_piece, &self.board, direction)
    }

    fn rotate_piece(p: &mut Piece, b: &Board, dir: RotationDirection) -> bool {
        if dir == 0 {
            return true;
        }
        p.rotate(dir);
        for (index, kick) in p.get_kicks(dir).iter().enumerate() {
            if p.moved(*kick) {
                if b.piece_valid_location(&p) {
                    p.set_kick(index);
                    return true;
                } else {
                    p.unsafe_move(kick.negative())
                }
            }
        }
        p.rotate(NUM_ROTATE_STATES - dir);
        false
    }

    fn ret_rotate_piece(p: &Piece, b: &Board, dir: RotationDirection) -> Option<Piece> {
        let mut piece = p.clone();
        if Game::rotate_piece(&mut piece, b, dir) {
            return Some(piece);
        }
        None
    }

    // versus
    pub fn get_t_spin_type(piece: &Piece, board: &Board) -> TSpinType {
        if piece.get_type() != 6 {
            return TSpinType::None;
        } else if piece.get_last_kick() == 999 {
            // println!{"ian would think this is a tspin"};
            return TSpinType::None;
        }

        let (front, back) = RELATIVE_CORNERS[piece.get_rotation_state()];

        let front_count = front
            .iter()
            .map(|x| x.add_to_point(&piece.get_center()))
            .flatten()
            .filter(|x| board.get(x.0 as usize, x.1 as usize))
            .count();

        let mut back_count = back
            .iter()
            .map(|x| x.add_to_point(&piece.get_center()))
            .flatten()
            .filter(|x| board.get(x.0 as usize, x.1 as usize))
            .count();

        if (piece.get_col() == 9 && piece.get_rotation_state() == 3)
            || (piece.get_col() == 0 && piece.get_rotation_state() == 1)
            || (piece.get_rotation_state() + piece.get_row() == 0)
        {
            back_count += 2;
        }

        return if (front_count == 2 && back_count >= 1)
            || (front_count == 1 && back_count >= 2 && piece.get_last_kick() == 4)
        {
            TSpinType::Full
        } else if front_count == 1 && back_count >= 2 {
            TSpinType::Mini
        } else {
            TSpinType::None
        };
    }

    // other
    pub fn reset_active_piece(&mut self) {
        self.active_piece = Piece::new(self.active_piece.get_type())
    }
    pub fn hold(&mut self) {
        let active_type = self.active_piece.get_type();
        if let Some(hold) = self.hold_piece {
            self.active_piece = Piece::new(hold);
        } else {
            self.active_piece = self.piece_queue.next();
        }
        self.hold_piece = Some(active_type);
    }

    pub fn hard_drop(&mut self) -> bool {
        self.active_drop();
        return self.set_piece()
    }

    pub fn set_piece(&mut self) -> bool {
        if self
            .board
            .top_out(&self.active_piece, &Piece::new(self.piece_queue.peek()))
        {
            return false;
        }

        self.board.set_piece(&self.active_piece);
        self.update();
        self.game_data.last_placed_piece = self.active_piece;
        self.active_piece = self.piece_queue.next();

        true
    }

    pub fn nearest_tpiece(&self) -> usize {
        if self.hold_piece == Some(6) || self.active_piece.get_type() == 6 {
            return 0;
        }
        self.piece_queue.nearest_tpiece()
    }

    pub fn nearest_ipiece(&self) -> usize {
        if self.hold_piece == Some(4) || self.active_piece.get_type() == 4 {
            return 0;
        }
        self.piece_queue.nearest_ipiece()
    }

    pub fn should_panic(&self) -> bool {
        if
            self.board.get_max_height() + self.game_data.garbage_in_queue > MAX_PLACE_HEIGHT / 2 && !self.game_data.panic ||
            self.game_data.combo > 0 && self.game_data.panic ||
            self.board.get_max_height() > MAX_PLACE_HEIGHT / 2 && self.game_data.panic
        {
            return true;
        }
        false
    }

    pub fn get_paranoia(&self) -> f32 {
        if self.should_panic() {
            return 0.0;
        }

        let (holes_count_total, holes_count_weighted, _) = self.board.holes_cell_covered();

        (clamp(2.0 * self.board.get_max_height() as f32 / MAX_PLACE_HEIGHT as f32, 0.1, 1.0) * self.nearest_ipiece() as f32) +
        (holes_count_total as f32 / 20.0 + holes_count_weighted as f32 / 8.0) +
        (self.nearest_ipiece() as f32 / 4.0)
    }

    pub fn get_garbage_in_queue(&self) -> usize {
        let mut garbage = 0;
        if ALLOWLOCALGAMEPLAY {
            garbage = fs::read_to_string(LOCALGAMEPLAYFILEPATH).expect("e").chars().nth(BOTNUM).expect("e").to_string().parse::<usize>().unwrap();
        }
        garbage
    }

    pub fn update_garbage_amount(&mut self) {
        self.game_data.garbage_in_queue = self.get_garbage_in_queue();
        self.game_data.total_garbage_recieved += self.game_data.garbage_in_queue; // will die in edge cases with garbage being sent during cancellation combos but im too lazy to accurately track this
        self.game_data.panic = self.should_panic();
    }

    pub fn update(&mut self) {
        let t_spin_type = Game::get_t_spin_type(&self.active_piece, &self.board);
        let lines_cleared = self.board.clear_lines();
        let attack_type = attack_type(t_spin_type, lines_cleared);

        self.game_data.update(lines_cleared, attack_type, t_spin_type, self.board.all_clear());
    }
}

pub mod game_rules_and_data {
    use std::str::FromStr;
    use super::*;
    use crate::constants::board_constants::{MAX_PLACE_HEIGHT};
    use crate::constants::versus_constants::AttackType::TD;

    #[derive(Default, Clone)]
    pub struct GameData {
        pub all_clear: bool,
        pub combo: i8,
        pub b2b: i8,

        pub pieces_placed: usize,
        pub lines_cleared: usize,
        pub lines_sent: u16,
        pub last_sent: u16,
        pub last_cleared: usize,
        pub last_placed_piece: Piece,

        pub t_spin: bool,
        pub t_spin_type: u16,

        pub game_over: bool,

        pub panic: bool,
        pub garbage_in_queue: usize,
        pub total_garbage_recieved: usize,
    }

    impl GameData {
        pub fn update(&mut self, lines_cleared: usize, attack: AttackType, t_spin_type: TSpinType, all_clear: bool) {
            self.pieces_placed += 1;
            self.t_spin_type = t_spin_type as u16;

            if lines_cleared == 0 {
                self.combo = 0;
                self.all_clear = false;
                self.last_cleared = 0;
                self.last_sent = 0;
                self.t_spin = false;
                return;
            }

            self.lines_cleared += lines_cleared;
            self.last_cleared = lines_cleared;

            if self.t_spin_type > 0 {
                self.t_spin = true;
            }

            // update lines sent before adding b2b/combo
            let mut lines_sent = calc_damage(self, attack, lines_cleared);
            if all_clear { lines_sent += 10; }
            self.lines_sent += lines_sent as u16;
            self.last_sent = lines_sent as u16;
            let b2b = BACK_TO_BACK_TYPES.contains(&attack);
            if b2b {
                self.b2b += 1;
            } else {
                self.b2b = 0;
            }
            self.combo += 1;

            self.all_clear = all_clear;
        }
    }

    #[derive(Clone)]
    pub struct GameRules {
        pub bag_type: BagType,
        pub allow_180: bool,
        pub allow_hard_drop: bool,
        pub allow_b2b_chain: bool,
        pub max_board_height: usize,
        pub kick_set: KickSet,
        pub spin_bonus: SpinBonus,
    }

    impl Default for GameRules {
        fn default() -> Self {
            Self {
                bag_type: Default::default(),
                allow_180: true,
                allow_hard_drop: true,
                allow_b2b_chain: true,
                max_board_height: MAX_PLACE_HEIGHT,
                kick_set: Default::default(),
                spin_bonus: Default::default()
            }
        }
    }

    #[derive(Clone)]
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

    impl FromStr for KickSet {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(match s {
                "SRS+" => KickSet::SRSPlus,
                "SRS" => KickSet::SRS,
                "SRS-X" => KickSet::SRSX,
                "TETRA-X" => KickSet::TetraX,
                "NRS" => KickSet::NRS,
                "ARS" => KickSet::ARS,
                "ASC" => KickSet::ASC,
                "none" => KickSet::None,
                other => {
                    eprintln!("unknown kickset '{}'", other);
                    KickSet::SRSPlus
                }
            })
        }
    }

    impl Default for KickSet {
        fn default() -> Self {
            KickSet::SRSPlus
        }
    }

    #[derive(Clone)]
    pub enum SpinBonus {
        TSpin,
        All,
        Stupid,
        None,
    }

    impl FromStr for SpinBonus {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(match s {
                "T-spins" => SpinBonus::TSpin,
                "all" => SpinBonus::All,
                "stupid" => SpinBonus::Stupid,
                "none" => SpinBonus::None,
                other => (|| {
                    eprintln!("unknown spinbonus '{}'", other);
                    SpinBonus::TSpin
                })(),
            })
        }
    }

    impl Default for SpinBonus {
        fn default() -> Self {
            SpinBonus::TSpin
        }
    }
}

#[cfg(test)]
pub mod game_test {
    use super::*;

    #[test]
    pub fn general_tests() {
        let mut game = Game::new(None);
        println!("{}", game);
        game.active_drop();
        println!("{}", game);

        // assert!(false);
    }
}
