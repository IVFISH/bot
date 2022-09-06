#![allow(dead_code)]

use crate::board::Board;
use crate::constants::piece_constants::{NUM_ROTATE_STATES, RELATIVE_CORNERS};
use crate::constants::types::{PieceType, RotationDirection};
use crate::constants::versus_constants::*;
use crate::piece::Piece;
use crate::point_vector::PointVector;
use crate::queue::{piece_type_to_string, PieceQueue};
use crate::versus::*;
use std::fmt::{Display, Formatter};

#[derive(Default, Clone)]
pub struct Game {
    pub board: Board,
    pub piece_queue: PieceQueue,
    game_data: GameData,
    game_rules: GameRules,

    active_piece: Piece,
    hold_piece: Option<PieceType>,
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Queue: {}", self.piece_queue)?;
        if let Some(hold) = self.hold_piece {
            write!(f, "Hold: {}\n", piece_type_to_string(hold))?;
        } else {
            write!(f, "Hold: None\n")?;
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

    // active piece
    pub fn get_active_piece(&self) -> &Piece {
        &self.active_piece
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
        Game::move_piece(&mut self.active_piece, &self.board, PointVector(0, -1))
    }

    pub fn ret_active_left(&mut self) -> Option<Piece> {
        Game::ret_move_piece(&self.active_piece.clone(), &self.board, PointVector(0, -1))
    }

    pub fn active_right(&mut self) -> bool {
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
        self.active_piece_rotate_direction(3)
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
                    p.moved(kick.negative());
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
        let out = self.set_piece();
        self.update();
        out
    }

    pub fn set_piece(&mut self) -> bool {
        if self
            .board
            .top_out(&self.active_piece, &Piece::new(self.piece_queue.peek()))
        {
            return false;
        }

        self.board.set_piece(&self.active_piece, true);
        self.active_piece = self.piece_queue.next();

        true
    }

    fn update(&mut self) {
        let lines_cleared = self.board.clear_lines(true);
        let t_spin_type = Game::get_t_spin_type(&self.active_piece, &self.board);
        let attack_type = attack_type(t_spin_type, lines_cleared);

        self.game_data
            .update(lines_cleared, attack_type, self.board.all_clear());
    }
}

#[derive(Default, Clone)]
pub struct GameData {
    pub all_clear: bool,
    pub combo: i8,
    pub b2b: i8,

    pub pieces_placed: usize,
    pub lines_cleared: usize,
    pub lines_sent: u16,
    pub last_sent: u8,
    pub last_cleared: usize,

    pub game_over: bool,
}

impl GameData {
    pub fn update(&mut self, lines_cleared: usize, attack: AttackType, all_clear: bool) {
        self.pieces_placed += 1;

        if lines_cleared == 0 {
            self.combo = 0;
            self.all_clear = false;
            self.last_cleared = 0;
            return;
        }

        self.lines_cleared += lines_cleared;
        self.last_cleared = lines_cleared;

        // update lines sent before adding b2b/combo
        let lines_sent = calc_damage(self, attack, lines_cleared);
        self.lines_sent += lines_sent as u16;
        self.last_sent = lines_sent as u8;

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

#[derive(Default, Clone)]
pub struct GameRules {}

#[cfg(test)]
pub mod game_test {
    use super::*;

    #[test]
    pub fn general_tests() {
        let mut game = Game::new(None);
        println!("{}", game);
        game.active_piece_drop();
        println!("{}", game);

        assert!(false);
    }
}
