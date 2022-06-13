use std::collections::VecDeque;
use std::iter::zip;

use crate::game::*;
use crate::players::*;
use crate::placement::*;
use crate::placement::piece_data::*;

use rand::Rng;

pub struct Bot {
    pub game: Game,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Command {
    None,
    MoveLeft,
    MoveRight,
    SoftDrop,
    RotateCW,
    RotateCCW,
    Rotate180,
    HardDrop,
    Hold,

}

type MoveList = Vec<Command>;

impl Default for Bot {
    fn default() -> Self {
        Self {
            game: Game::new(None)
        }
    }
}

impl Player for Bot {
    fn make_move(&mut self) {
        let mut moves = self.all_moves();
        let num = rand::thread_rng().gen_range(0..moves.len());

        let mut action = moves.remove(num);
        action.push(Command::HardDrop);

        self.do_move_list(action);
    }

    fn get_next_move(&self) {
        todo!()
    }

    fn score_board(&self) -> i32 {
        todo!()
    }

    fn score(&self) -> i32 {
        todo!()
    }
}

impl Bot {
    pub fn new(optional_seed: Option<usize>) -> Self {
        Self {
            game: Game::new(optional_seed)
        }
    }

    pub fn all_moves(&mut self) -> Vec<MoveList> {
        let start_piece = self.game.active_piece.piece_type;
        let hold_piece;

        if let Some(piece) = self.game.hold_piece {
            hold_piece = piece
        } else {
            hold_piece = self.game.piece_queue_peek();
        }

        let (moves, used) = self.find_trivial(false);
        let (mut moves, _) = self.add_non_trivial(moves, used);

        self.game.active_piece = Placement::new(hold_piece);

        let (hold_moves, used) = self.find_trivial(true);
        let (hold_moves, _) = self.add_non_trivial(hold_moves, used);

        moves.extend(hold_moves);

        self.game.active_piece = Placement::new(start_piece);

        moves
    }

    fn do_undo_action(&mut self, action: fn(&mut Game) -> bool, command: Command, current_move: &Vec<Command>, mut used_placements: &Vec<Placement>)
                      -> (Vec<MoveList>, Vec<Placement>) {
        // saves the start state

        // while it can apply the action on the piece
        // soft drop and check for new spot
        // if new, add to moves and used
        // else break

        let save = self.game.active_piece.clone();

        let mut added_moves = vec![];
        let mut added_used = vec![];

        let mut add_list = current_move.clone();
        add_list.push(Command::SoftDrop);

        while action(&mut self.game) {
            add_list.push(command);

            self.game.piece_soft_drop();

            if Bot::new_placement(&self.game.active_piece, &used_placements) &&
                Bot::new_placement(&self.game.active_piece, &added_used) {

                added_moves.push(add_list.clone());
                added_used.push(self.game.active_piece.clone());
                continue;
            }

            break;
        }

        self.game.active_piece = save;

        (added_moves, added_used)
    }

    fn find_trivial(&mut self, hold: bool) -> (Vec<MoveList>, Vec<Placement>) {
        let mut trivial_moves = Vec::new();
        let mut trivial_placements = Vec::new();

        let rotations = [Command::None, Command::RotateCW, Command::Rotate180, Command::RotateCCW];

        let row = self.game.active_piece.center.row;

        for rotation in rotations {
            self.game.active_piece.move_center_to_column(0);
            for col in 0..10 {
                if self.game.valid_location_for_active() {

                    let mut inputs: MoveList;
                    if hold {
                        inputs = vec![Command::Hold, rotation];
                    } else {
                        inputs = vec![rotation];
                    }

                    inputs.append(Bot::column_to_move_list(col).as_mut());
                    trivial_moves.push(inputs);

                    self.game.piece_soft_drop();
                    trivial_placements.push(self.game.active_piece.clone());
                    self.game.active_piece.center.row = row;
                }

                self.game.active_piece.move_by_vector(MoveVector(0, 1));
            }
            self.game.active_piece.rotate(1);
        }

        self.game.active_piece = Placement::new(self.game.active_piece.piece_type);

        (trivial_moves, trivial_placements)
    }

    fn add_non_trivial(&mut self, mut trivial: Vec<MoveList>, mut used_placements: Vec<Placement>)
                       -> (Vec<MoveList>, Vec<Placement>) {
        let mut unchecked_moves = VecDeque::from(trivial.clone());
        let mut unchecked_placements = VecDeque::from(used_placements.clone());

        let commands = [Command::MoveRight, Command::MoveLeft, Command::RotateCW, Command::RotateCCW, Command::Rotate180];
        let actions = [Game::piece_right, Game::piece_left, Game::piece_rotate_cw, Game::piece_rotate_ccw, Game::piece_rotate_180];

        while !unchecked_moves.is_empty() {
            let current_move = unchecked_moves.pop_front().unwrap();
            self.game.active_piece = unchecked_placements.pop_front().unwrap();

            for (command, action) in zip(commands, actions) {
                let (new_trivial, new_used_placements) =
                    self.do_undo_action(action, command, &current_move, &used_placements);

                unchecked_moves.append(&mut VecDeque::from(new_trivial.clone()));
                unchecked_placements.append(&mut VecDeque::from(new_used_placements.clone()));
                trivial.extend(new_trivial);
                used_placements.extend(new_used_placements);
            }
        }

        self.game.active_piece = Placement::new(self.game.active_piece.piece_type);

        (trivial, used_placements)
    }


    fn do_command(&mut self, command: Command) {
        match command {
            Command::MoveLeft => self.game.piece_left(),
            Command::MoveRight => self.game.piece_right(),
            Command::SoftDrop => self.game.piece_soft_drop(),
            Command::RotateCW => self.game.piece_rotate_cw(),
            Command::RotateCCW => self.game.piece_rotate_ccw(),
            Command::Rotate180 => self.game.piece_rotate_180(),
            Command::HardDrop => self.game.piece_hard_drop(true).is_ok(),
            Command::Hold => {self.game.hold(); true},
            Command::None => false,
        };
    }

    fn new_placement(placement: &Placement, used_placements: &Vec<Placement>) -> bool {
        !used_placements.contains(placement)
    }

    fn do_move_list(&mut self, commands: MoveList) {
        for command in commands {
            self.do_command(command);
        }
    }

    fn column_to_move_list(col: usize) -> MoveList {
        if col == SPAWN_COL {
            return vec![Command::None];
        }
        if col < SPAWN_COL {
            return vec![Command::MoveLeft; SPAWN_COL - col];
        }
        return vec![Command::MoveRight; col - SPAWN_COL];
    }
}

#[cfg(test)]
mod move_gen_tests {
    use super::*;

    #[test]
    fn test_tucks() {
        let mut bot = Bot::new(Some(1337));

        bot.game.add(1, 7, false);
        bot.game.add(1, 8, false);
        bot.game.add(1, 9, false);

        bot.game.add(1, 0, false);
        bot.game.add(1, 1, false);
        bot.game.add(1, 2, false);

        bot.game.piece_das_right();
        bot.game.piece_hard_drop(true).expect("die");

        let (trivial, used) = bot.find_trivial(false);
        let trivial_only = trivial.clone();
        let (all_moves, _) = bot.add_non_trivial(trivial, used);

        let non_trivial: Vec<&MoveList> = all_moves
            .iter()
            .filter(
                |x| !trivial_only.contains(*x)
            ).collect();

        assert_eq!(non_trivial.len(), 12);

        let expected_out = vec![
            vec![Command::None, Command::None, Command::SoftDrop, Command::MoveRight],
            vec![Command::None, Command::None, Command::SoftDrop, Command::MoveRight, Command::MoveRight],
            vec![Command::None, Command::None, Command::SoftDrop, Command::MoveRight, Command::MoveRight, Command::MoveRight],
            vec![Command::None, Command::None, Command::SoftDrop, Command::MoveLeft],
            vec![Command::None, Command::None, Command::SoftDrop, Command::MoveLeft, Command::MoveLeft],
            vec![Command::None, Command::None, Command::SoftDrop, Command::MoveLeft, Command::MoveLeft, Command::MoveLeft],
            vec![Command::Rotate180, Command::MoveRight, Command::SoftDrop, Command::MoveRight],
            vec![Command::Rotate180, Command::MoveRight, Command::SoftDrop, Command::MoveRight, Command::MoveRight],
            vec![Command::Rotate180, Command::MoveRight, Command::SoftDrop, Command::MoveRight, Command::MoveRight, Command::MoveRight],
            vec![Command::Rotate180, Command::MoveRight, Command::SoftDrop, Command::MoveLeft],
            vec![Command::Rotate180, Command::MoveRight, Command::SoftDrop, Command::MoveLeft, Command::MoveLeft],
            vec![Command::Rotate180, Command::MoveRight, Command::SoftDrop, Command::MoveLeft, Command::MoveLeft, Command::MoveLeft]];

        for out in expected_out {
            assert!(non_trivial.contains(&&out));
        }
    }

    #[test]
    fn test_z_spin() {
        let mut bot = make_z_spin_1();

        let (moves, used) = bot.find_trivial(false);
        let (_, placements) = bot.add_non_trivial(moves, used);

        assert!(placements.iter().
            any(|x|
                x.abs_locations().unwrap() == [Point { row: 0, col: 5 }, Point { row: 0, col: 4 }, Point { row: 1, col: 4 }, Point { row: 1, col: 3 }]));
    }

    #[test]
    fn test_tst() {
        let mut bot = make_tst();

        let (moves, used) = bot.find_trivial(false);
        let (_, placements) = bot.add_non_trivial(moves, used);

        assert!(placements.iter().
            any(|x|
                x.abs_locations().unwrap() == [Point { row: 1, col: 2 }, Point { row: 0, col: 3 }, Point { row: 1, col: 3 }, Point { row: 2, col: 3 }]));
    }

    #[test]
    fn test_l_spin_jank() {
        let mut bot = make_fuckery();

        let (moves, used) = bot.find_trivial(false);
        let (_, placements) = bot.add_non_trivial(moves, used);

        assert!(placements.iter().
            any(|x|
                x.abs_locations().unwrap() == [Point { row: 0, col: 2 }, Point { row: 2, col: 1 }, Point { row: 1, col: 1 }, Point { row: 0, col: 1 }]));
    }

    fn make_z_spin_1() -> Bot {
        let mut bot = Bot::new(None);
        bot.game.active_piece = Placement::new(0);

        bot.game.add(0, 0, false);
        bot.game.add(1, 0, false);
        bot.game.add(0, 1, false);
        bot.game.add(1, 1, false);
        bot.game.add(0, 2, false);
        bot.game.add(1, 2, false);
        bot.game.add(0, 3, false);
        bot.game.add(1, 5, false);
        bot.game.add(0, 6, false);
        bot.game.add(1, 6, false);
        bot.game.add(0, 7, false);
        bot.game.add(1, 7, false);
        bot.game.add(0, 8, false);
        bot.game.add(1, 8, false);
        bot.game.add(0, 9, false);
        bot.game.add(1, 9, false);

        bot
    }

    fn make_tst() -> Bot {
        let mut bot = Bot::new(None);
        bot.game.active_piece = Placement::new(6);

        bot.game.add(1, 0, false);
        bot.game.add(0, 0, false);
        bot.game.add(0, 1, false);
        bot.game.add(0, 2, false);
        bot.game.add(2, 1, false);
        bot.game.add(2, 0, false);
        bot.game.add(1, 1, false);
        bot.game.add(2, 2, false);
        bot.game.add(0, 4, false);
        bot.game.add(2, 4, false);
        bot.game.add(1, 4, false);
        bot.game.add(3, 4, false);
        bot.game.add(4, 4, false);
        bot.game.add(4, 3, false);
        bot.game.add(4, 5, false);
        bot.game.add(3, 5, false);
        bot.game.add(1, 5, false);
        bot.game.add(1, 5, false);
        bot.game.add(2, 5, false);
        bot.game.add(0, 5, false);
        bot.game.add(2, 6, false);
        bot.game.add(1, 6, false);
        bot.game.add(0, 6, false);
        bot.game.add(2, 7, false);
        bot.game.add(1, 7, false);
        bot.game.add(0, 7, false);
        bot.game.add(2, 8, false);
        bot.game.add(1, 8, false);
        bot.game.add(0, 8, false);
        bot.game.add(2, 9, false);
        bot.game.add(1, 9, false);
        bot.game.add(0, 9, false);

        bot
    }

    fn make_fuckery() -> Bot {
        let mut bot = Bot::new(None);
        bot.game.active_piece = Placement::new(1);

        bot.game.add(0, 0, false);
        bot.game.add(1, 0, false);
        bot.game.add(2, 0, false);
        bot.game.add(3, 0, false);
        bot.game.add(4, 0, false);
        bot.game.add(5, 0, false);
        bot.game.add(6, 0, false);
        bot.game.add(7, 0, false);
        bot.game.add(8, 0, false);
        bot.game.add(9, 0, false);
        bot.game.add(10, 0, false);
        bot.game.add(11, 0, false);
        bot.game.add(12, 0, false);
        bot.game.add(13, 0, false);
        bot.game.add(14, 0, false);
        bot.game.add(4, 1, false);
        bot.game.add(5, 1, false);
        bot.game.add(6, 1, false);
        bot.game.add(7, 1, false);
        bot.game.add(8, 1, false);
        bot.game.add(9, 1, false);
        bot.game.add(10, 1, false);
        bot.game.add(11, 1, false);
        bot.game.add(12, 1, false);
        bot.game.add(14, 1, false);
        bot.game.add(1, 2, false);
        bot.game.add(2, 2, false);
        bot.game.add(5, 2, false);
        bot.game.add(6, 2, false);
        bot.game.add(7, 2, false);
        bot.game.add(8, 2, false);
        bot.game.add(9, 2, false);
        bot.game.add(0, 3, false);
        bot.game.add(1, 3, false);
        bot.game.add(6, 3, false);
        bot.game.add(7, 3, false);
        bot.game.add(8, 3, false);
        bot.game.add(9, 3, false);
        bot.game.add(11, 3, false);
        bot.game.add(12, 3, false);
        bot.game.add(0, 4, false);
        bot.game.add(1, 4, false);
        bot.game.add(3, 4, false);
        bot.game.add(4, 4, false);
        bot.game.add(6, 4, false);
        bot.game.add(9, 4, false);
        bot.game.add(12, 4, false);
        bot.game.add(0, 5, false);
        bot.game.add(1, 5, false);
        bot.game.add(2, 5, false);
        bot.game.add(3, 5, false);
        bot.game.add(4, 5, false);
        bot.game.add(12, 5, false);
        bot.game.add(0, 6, false);
        bot.game.add(1, 6, false);
        bot.game.add(2, 6, false);
        bot.game.add(3, 6, false);
        bot.game.add(4, 6, false);
        bot.game.add(5, 6, false);
        bot.game.add(6, 6, false);
        bot.game.add(7, 6, false);
        bot.game.add(9, 6, false);
        bot.game.add(10, 6, false);
        bot.game.add(11, 6, false);
        bot.game.add(12, 6, false);
        bot.game.add(0, 7, false);
        bot.game.add(1, 7, false);
        bot.game.add(2, 7, false);
        bot.game.add(3, 7, false);
        bot.game.add(4, 7, false);
        bot.game.add(5, 7, false);
        bot.game.add(6, 7, false);
        bot.game.add(7, 7, false);
        bot.game.add(9, 7, false);
        bot.game.add(10, 7, false);
        bot.game.add(11, 7, false);
        bot.game.add(12, 7, false);
        bot.game.add(0, 8, false);
        bot.game.add(1, 8, false);
        bot.game.add(2, 8, false);
        bot.game.add(3, 8, false);
        bot.game.add(4, 8, false);
        bot.game.add(5, 8, false);
        bot.game.add(6, 8, false);
        bot.game.add(7, 8, false);
        bot.game.add(8, 8, false);
        bot.game.add(9, 8, false);
        bot.game.add(10, 8, false);
        bot.game.add(11, 8, false);
        bot.game.add(12, 8, false);
        bot.game.add(0, 9, false);
        bot.game.add(1, 9, false);
        bot.game.add(2, 9, false);
        bot.game.add(3, 9, false);
        bot.game.add(4, 9, false);
        bot.game.add(5, 9, false);
        bot.game.add(6, 9, false);
        bot.game.add(7, 9, false);
        bot.game.add(8, 9, false);
        bot.game.add(9, 9, false);
        bot.game.add(10, 9, false);
        bot.game.add(11, 9, false);
        bot.game.add(12, 9, false);

        bot
    }
}

