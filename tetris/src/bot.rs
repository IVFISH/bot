use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use std::iter::zip;

use polynomial::Polynomial;

use crate::communications::*;
use crate::game::*;
use crate::placement::piece_data::*;
use crate::placement::*;
use crate::players::*;

use rand::Rng;

pub struct Bot {
    game: Game,
    weight: Weights,
}

impl Display for Bot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.game)?;
        Ok(())
    }
}

impl Default for Bot {
    fn default() -> Self {
        Self {
            game: Game::new(None),
            weight: Weights::default(),
        }
    }
}

impl Player for Bot {
    fn get_game(&mut self) -> &mut Game {
        &mut self.game
    }

    fn get_next_move(&mut self) -> MoveList {
        let (mut moves, scores) = self.all_moves();

        // let num = rand::thread_rng().gen_range(0..moves.len());
        let min_score = scores.iter().min().unwrap();
        let num = scores.iter().position(|x| x == min_score).unwrap();

        let mut action = moves.remove(num);
        action.push(Command::HardDrop);
        action
    }
}

pub type Score = usize;

impl Bot {
    pub fn suggest_next_move(&mut self) -> Suggestion {
        let action = self.get_next_move();
        let action = move_list_to_string(&action);

        Suggestion {
            input_list: action,
            info: "".to_string()}

    }

    pub fn suggest_and_move(&mut self) -> Suggestion {
        let action = self.get_next_move();
        let action = move_list_to_string(&action);

        let out = Suggestion {
            input_list: action,
            info: "".to_string()};
        self.make_move();
        out
    }

    pub fn game_over(&mut self) {
        self.game.game_data.game_over = true;
    }

    pub fn score(&mut self, set_piece: bool) -> Score {
        // todo: add versus weights, such as combo/b2b/attack

        self.score_board(set_piece) + self.score_game()
    }

    fn score_board(&mut self, set_piece: bool) -> Score {
        if set_piece {
            self.game.board.set_piece(&self.game.active_piece, true);
            // println!("{} {:?}", self.game, self.game.board.heights_for_each_column);
        }

        let out = self.get_holes_and_cell_covered_score()
            + self.get_height_score()
            + self.get_height_differences_score();

        if set_piece {
            self.game.board.remove_piece(&self.game.active_piece, true);
            // println!("{} {:?}", self.game, self.game.board.heights_for_each_column);
        }

        out
    }

    fn score_game(&mut self) -> Score {
        0
    }

    fn get_height_differences_score(&self) -> usize {
        self.game
            .board
            .get_height_differences()
            .iter()
            .map(|x| self.weight.adjacent_height_differences_weight.eval(*x))
            .sum()
    }

    pub fn get_height_score(&self) -> usize {
        let total_height = self.game.board.max_filled_height();
        self.weight.height_weight.eval(total_height)
    }

    pub fn get_holes_and_cell_covered_score(&self) -> usize {
        let mut out = 0;

        let (holes, covered) = self.game.board.holes_and_cell_covered();

        out += self.weight.num_hole_weight.eval(holes);
        out += self.weight.cell_covered_weight.eval(covered);

        out
    }

    pub fn create(game: Game) -> Self {
        Self {
            game,
            weight: Default::default(),
        }
    }

    pub fn new(optional_seed: Option<usize>, weight: Weights) -> Self {
        Self {
            game: Game::new(optional_seed),
            weight,
        }
    }

    pub fn all_moves(&mut self) -> (Vec<MoveList>, Vec<Score>) {
        let start_piece = self.game.get_active_piece_type();
        let hold_piece;

        if let Some(piece) = self.game.hold_piece {
            hold_piece = piece
        } else {
            hold_piece = self.game.piece_queue_peek();
        }

        let (moves, used, scores) = self.find_trivial(false);
        let (mut moves, _, mut scores) = self.add_non_trivial(moves, used, scores);

        self.game.active_piece = Placement::new(hold_piece);

        let (hold_moves, used, hold_scores) = self.find_trivial(true);
        let (hold_moves, _, hold_scores) = self.add_non_trivial(hold_moves, used, hold_scores);

        moves.extend(hold_moves);
        scores.extend(hold_scores);

        self.game.active_piece = Placement::new(start_piece);

        (moves, scores)
    }

    fn all_placements(&mut self) -> Vec<Placement> {
        // only used for debugging purposes

        let start_piece = self.game.get_active_piece_type();
        let hold_piece;

        if let Some(piece) = self.game.hold_piece {
            hold_piece = piece
        } else {
            hold_piece = self.game.piece_queue_peek();
        }

        let (moves, used, score) = self.find_trivial(false);
        let (_, mut placements, _) = self.add_non_trivial(moves, used, score);

        self.game.active_piece = Placement::new(hold_piece);

        let (hold_moves, used, hold_score) = self.find_trivial(true);
        let (_, hold_placements, _) = self.add_non_trivial(hold_moves, used, hold_score);

        placements.extend(hold_placements);

        self.game.active_piece = Placement::new(start_piece);

        placements
    }

    pub fn show_all_placements_on_timer(&mut self, clear: bool) {
        let all_placements = self.all_placements();
        let start = self.game.active_piece.clone();

        for placement in all_placements {
            self.show_placement(&placement, clear, &start);
            thread::sleep(time::Duration::from_millis(1000));
        }
    }

    pub fn show_all_placements_on_input(&mut self, clear: bool) {
        use std::io;

        let all_placements = self.all_placements();
        let start = self.game.active_piece.clone();
        let num_moves = all_placements.len();
        let mut index = num_moves - 1;

        loop {
            if index == 0 {
                index = num_moves;
                // prevent back flow error (-usize)
            }

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Cannot read line.");
            let mut input = input.trim();

            if input == String::from(",") {
                index -= 1;
                index %= num_moves;

                let placement = &all_placements.get(index).unwrap().clone();
                self.show_placement(placement, clear, &start);
            } else if input == String::from(".") {
                index += 1;
                index %= num_moves;

                let placement = &all_placements.get(index).unwrap().clone();
                self.show_placement(placement, clear, &start);
            } else if input == String::from("exit") {
                break;
            }
        }
    }

    fn show_placement(&mut self, mut target_placement: &Placement, clear: bool, start: &Placement) {
        if clear {
            print!("{}[2J", 27 as char);
        }

        self.game.active_piece = target_placement.clone();
        println!("{}", self);
        self.game.active_piece = start.clone();
    }

    fn do_undo_action(
        &mut self,
        action: fn(&mut Game) -> bool,
        command: Command,
        current_move: &Vec<Command>,
        used_placements: &Vec<Placement>,
    ) -> (Vec<MoveList>, Vec<Placement>, Vec<Score>) {
        // saves the start state

        // while it can apply the action on the piece
        // soft drop and check for new spot
        // if new, add to moves and used
        // else break

        let save = self.game.active_piece.clone();

        let mut added_moves = vec![];
        let mut added_used = vec![];
        let mut added_scores = vec![];

        let mut add_list = current_move.clone();
        add_list.push(Command::SoftDrop);

        while action(&mut self.game) {
            add_list.push(command);

            self.game.piece_soft_drop();

            if Bot::new_placement(&self.game.active_piece, &used_placements)
                && Bot::new_placement(&self.game.active_piece, &added_used)
            {
                added_moves.push(add_list.clone());
                added_used.push(self.game.active_piece.clone());
                added_scores.push(self.score(true));
                continue;
            }

            break;
        }

        self.game.active_piece = save;

        (added_moves, added_used, added_scores)
    }

    fn find_trivial(&mut self, hold: bool) -> (Vec<MoveList>, Vec<Placement>, Vec<Score>) {
        let mut trivial_moves = Vec::new();
        let mut trivial_placements = Vec::new();
        let mut trivial_scores = Vec::new();

        let rotations = [
            Command::None,
            Command::RotateCW,
            Command::Rotate180,
            Command::RotateCCW,
        ];

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
                    trivial_scores.push(self.score(true));
                    trivial_placements.push(self.game.active_piece.clone());
                    self.game.active_piece.center.row = row;
                }

                self.game.active_piece.move_by_vector(MoveVector(0, 1));
            }
            self.game.active_piece.rotate(1);
        }

        self.game.reset_active_piece();

        (trivial_moves, trivial_placements, trivial_scores)
    }

    fn add_non_trivial(
        &mut self,
        mut trivial: Vec<MoveList>,
        mut used_placements: Vec<Placement>,
        mut trivial_scores: Vec<Score>,
    ) -> (Vec<MoveList>, Vec<Placement>, Vec<Score>) {
        let mut unchecked_moves = VecDeque::from(trivial.clone());
        let mut unchecked_placements = VecDeque::from(used_placements.clone());

        let commands = [
            Command::MoveRight,
            Command::MoveLeft,
            Command::RotateCW,
            Command::RotateCCW,
            Command::Rotate180,
        ];
        let actions = [
            Game::piece_right,
            Game::piece_left,
            Game::piece_rotate_cw,
            Game::piece_rotate_ccw,
            Game::piece_rotate_180,
        ];

        while !unchecked_moves.is_empty() {
            let current_move = unchecked_moves.pop_front().unwrap();
            self.game.active_piece = unchecked_placements.pop_front().unwrap();

            for (command, action) in zip(commands, actions) {
                let (new_trivial, new_used_placements, new_scores) =
                    self.do_undo_action(action, command, &current_move, &used_placements);

                unchecked_moves.append(&mut VecDeque::from(new_trivial.clone()));
                unchecked_placements.append(&mut VecDeque::from(new_used_placements.clone()));
                trivial.extend(new_trivial);
                used_placements.extend(new_used_placements);
                trivial_scores.extend(new_scores)
            }
        }

        self.game.reset_active_piece();

        (trivial, used_placements, trivial_scores)
    }

    fn new_placement(placement: &Placement, used_placements: &Vec<Placement>) -> bool {
        !used_placements.contains(placement)
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

pub struct Weights {
    pub height_weight: Polynomial<usize>,

    pub adjacent_height_differences_weight: Polynomial<usize>,
    pub num_hole_weight: Polynomial<usize>,
    pub cell_covered_weight: Polynomial<usize>,

    pub b2b_weight: Polynomial<i8>,
    pub combo_weight: Polynomial<i8>,
}

impl Default for Weights {
    fn default() -> Self {
        Self {
            height_weight: Polynomial::new(vec![0, 0, 1]),

            adjacent_height_differences_weight: Polynomial::new(vec![0, 3, 0]),
            num_hole_weight: Polynomial::new(vec![0, 5, 0]),
            cell_covered_weight: Polynomial::new(vec![0, 0, 1]),

            b2b_weight: Polynomial::new(vec![0, -1, -5]),
            combo_weight: Polynomial::new(vec![0, -2, -2]),
        }
    }
}

use std::{thread, time};

pub fn bot_play() {
    let mut bot = Bot::default();

    while !bot.game.game_over {
        // clears the console
        print!("{}[2J", 27 as char);

        bot.make_move();
        println!("{}", bot.game);

        thread::sleep(time::Duration::from_millis(100));
    }
}

pub fn bot_debug() {
    let mut bot = Bot::default();

    // bot.show_all_placements_on_timer(true);
    bot.show_all_placements_on_input(true);
}

#[cfg(test)]
mod move_gen_tests {
    use super::*;

    #[test]
    fn test_score_board() {
        let mut bot = Bot::new(None, test_weights());

        bot.game.board.add(1, 1, true);
        bot.game.board.add(1, 2, true);
        bot.game.board.add(5, 1, true);
        bot.game.board.add(3, 7, true);

        assert_eq!(bot.score_board(false), 344)
    }

    #[test]
    fn test_score_fuckery_board() {
        let mut bot = make_fuckery();

        // 15^2 + 5(15) = 300
        // println!("{}", bot.get_height_score());

        // [5, 3] = 35 + 15 = 50
        // println!("{}", bot.get_height_differences_score());

        // 13 * 6 = 78
        // 113^2 + 5(113) = 13334

        assert_eq!(bot.score_board(false), 13762);
    }

    #[test]
    fn test_tucks() {
        let mut bot = Bot::new(Some(1337), Weights::default());

        bot.game.add(1, 7, false);
        bot.game.add(1, 8, false);
        bot.game.add(1, 9, false);

        bot.game.add(1, 0, false);
        bot.game.add(1, 1, false);
        bot.game.add(1, 2, false);

        bot.game.piece_das_right();
        bot.game.piece_hard_drop(true).expect("die");

        let (trivial, used, score) = bot.find_trivial(false);
        let trivial_only = trivial.clone();
        let (all_moves, _, _) = bot.add_non_trivial(trivial, used, score);

        let non_trivial: Vec<&MoveList> = all_moves
            .iter()
            .filter(|x| !trivial_only.contains(*x))
            .collect();

        assert_eq!(non_trivial.len(), 12);

        let expected_out = vec![
            vec![
                Command::None,
                Command::None,
                Command::SoftDrop,
                Command::MoveRight,
            ],
            vec![
                Command::None,
                Command::None,
                Command::SoftDrop,
                Command::MoveRight,
                Command::MoveRight,
            ],
            vec![
                Command::None,
                Command::None,
                Command::SoftDrop,
                Command::MoveRight,
                Command::MoveRight,
                Command::MoveRight,
            ],
            vec![
                Command::None,
                Command::None,
                Command::SoftDrop,
                Command::MoveLeft,
            ],
            vec![
                Command::None,
                Command::None,
                Command::SoftDrop,
                Command::MoveLeft,
                Command::MoveLeft,
            ],
            vec![
                Command::None,
                Command::None,
                Command::SoftDrop,
                Command::MoveLeft,
                Command::MoveLeft,
                Command::MoveLeft,
            ],
            vec![
                Command::Rotate180,
                Command::MoveRight,
                Command::SoftDrop,
                Command::MoveRight,
            ],
            vec![
                Command::Rotate180,
                Command::MoveRight,
                Command::SoftDrop,
                Command::MoveRight,
                Command::MoveRight,
            ],
            vec![
                Command::Rotate180,
                Command::MoveRight,
                Command::SoftDrop,
                Command::MoveRight,
                Command::MoveRight,
                Command::MoveRight,
            ],
            vec![
                Command::Rotate180,
                Command::MoveRight,
                Command::SoftDrop,
                Command::MoveLeft,
            ],
            vec![
                Command::Rotate180,
                Command::MoveRight,
                Command::SoftDrop,
                Command::MoveLeft,
                Command::MoveLeft,
            ],
            vec![
                Command::Rotate180,
                Command::MoveRight,
                Command::SoftDrop,
                Command::MoveLeft,
                Command::MoveLeft,
                Command::MoveLeft,
            ],
        ];

        for out in expected_out {
            assert!(non_trivial.contains(&&out));
        }
    }

    #[test]
    fn test_z_spin() {
        let mut bot = make_z_spin_1();

        let (moves, used, score) = bot.find_trivial(false);
        let (_, placements, _) = bot.add_non_trivial(moves, used, score);

        assert!(placements.iter().any(|x| x.abs_locations().unwrap()
            == [
                Point { row: 0, col: 5 },
                Point { row: 0, col: 4 },
                Point { row: 1, col: 4 },
                Point { row: 1, col: 3 }
            ]));
    }

    #[test]
    fn test_tst() {
        let mut bot = make_tst();

        let (moves, used, score) = bot.find_trivial(false);
        let (_, placements, _) = bot.add_non_trivial(moves, used, score);

        assert!(placements.iter().any(|x| x.abs_locations().unwrap()
            == [
                Point { row: 1, col: 2 },
                Point { row: 0, col: 3 },
                Point { row: 1, col: 3 },
                Point { row: 2, col: 3 }
            ]));
    }

    #[test]
    fn test_l_spin_jank() {
        let mut bot = make_fuckery();

        let (moves, used, score) = bot.find_trivial(false);
        let (_, placements, _) = bot.add_non_trivial(moves, used, score);

        assert!(placements.iter().any(|x| x.abs_locations().unwrap()
            == [
                Point { row: 0, col: 2 },
                Point { row: 2, col: 1 },
                Point { row: 1, col: 1 },
                Point { row: 0, col: 1 }
            ]));
    }

    fn test_weights() -> Weights {
        Weights {
            height_weight: Polynomial::new(vec![0, 5, 1]),

            adjacent_height_differences_weight: Polynomial::new(vec![0, 2, 1]),
            num_hole_weight: Polynomial::new(vec![0, 6, 0]),
            cell_covered_weight: Polynomial::new(vec![0, 5, 1]),

            b2b_weight: Polynomial::new(vec![0, -1, -5]),
            combo_weight: Polynomial::new(vec![0, -2, -2]),
        }
    }

    fn make_z_spin_1() -> Bot {
        let mut bot = Bot::new(None, Weights::default());
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
        let mut bot = Bot::new(None, Weights::default());
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
        let mut bot = Bot::new(None, test_weights());
        bot.game.active_piece = Placement::new(1);

        bot.game.add(0, 0, true);
        bot.game.add(1, 0, true);
        bot.game.add(2, 0, true);
        bot.game.add(3, 0, true);
        bot.game.add(4, 0, true);
        bot.game.add(5, 0, true);
        bot.game.add(6, 0, true);
        bot.game.add(7, 0, true);
        bot.game.add(8, 0, true);
        bot.game.add(9, 0, true);
        bot.game.add(10, 0, true);
        bot.game.add(11, 0, true);
        bot.game.add(12, 0, true);
        bot.game.add(13, 0, true);
        bot.game.add(14, 0, true);
        bot.game.add(4, 1, true);
        bot.game.add(5, 1, true);
        bot.game.add(6, 1, true);
        bot.game.add(7, 1, true);
        bot.game.add(8, 1, true);
        bot.game.add(9, 1, true);
        bot.game.add(10, 1, true);
        bot.game.add(11, 1, true);
        bot.game.add(12, 1, true);
        bot.game.add(14, 1, true);
        bot.game.add(1, 2, true);
        bot.game.add(2, 2, true);
        bot.game.add(5, 2, true);
        bot.game.add(6, 2, true);
        bot.game.add(7, 2, true);
        bot.game.add(8, 2, true);
        bot.game.add(9, 2, true);
        bot.game.add(0, 3, true);
        bot.game.add(1, 3, true);
        bot.game.add(6, 3, true);
        bot.game.add(7, 3, true);
        bot.game.add(8, 3, true);
        bot.game.add(9, 3, true);
        bot.game.add(11, 3, true);
        bot.game.add(12, 3, true);
        bot.game.add(0, 4, true);
        bot.game.add(1, 4, true);
        bot.game.add(3, 4, true);
        bot.game.add(4, 4, true);
        bot.game.add(6, 4, true);
        bot.game.add(9, 4, true);
        bot.game.add(12, 4, true);
        bot.game.add(0, 5, true);
        bot.game.add(1, 5, true);
        bot.game.add(2, 5, true);
        bot.game.add(3, 5, true);
        bot.game.add(4, 5, true);
        bot.game.add(12, 5, true);
        bot.game.add(0, 6, true);
        bot.game.add(1, 6, true);
        bot.game.add(2, 6, true);
        bot.game.add(3, 6, true);
        bot.game.add(4, 6, true);
        bot.game.add(5, 6, true);
        bot.game.add(6, 6, true);
        bot.game.add(7, 6, true);
        bot.game.add(9, 6, true);
        bot.game.add(10, 6, true);
        bot.game.add(11, 6, true);
        bot.game.add(12, 6, true);
        bot.game.add(0, 7, true);
        bot.game.add(1, 7, true);
        bot.game.add(2, 7, true);
        bot.game.add(3, 7, true);
        bot.game.add(4, 7, true);
        bot.game.add(5, 7, true);
        bot.game.add(6, 7, true);
        bot.game.add(7, 7, true);
        bot.game.add(9, 7, true);
        bot.game.add(10, 7, true);
        bot.game.add(11, 7, true);
        bot.game.add(12, 7, true);
        bot.game.add(0, 8, true);
        bot.game.add(1, 8, true);
        bot.game.add(2, 8, true);
        bot.game.add(3, 8, true);
        bot.game.add(4, 8, true);
        bot.game.add(5, 8, true);
        bot.game.add(6, 8, true);
        bot.game.add(7, 8, true);
        bot.game.add(8, 8, true);
        bot.game.add(9, 8, true);
        bot.game.add(10, 8, true);
        bot.game.add(11, 8, true);
        bot.game.add(12, 8, true);
        bot.game.add(0, 9, true);
        bot.game.add(1, 9, true);
        bot.game.add(2, 9, true);
        bot.game.add(3, 9, true);
        bot.game.add(4, 9, true);
        bot.game.add(5, 9, true);
        bot.game.add(6, 9, true);
        bot.game.add(7, 9, true);
        bot.game.add(8, 9, true);
        bot.game.add(9, 9, true);
        bot.game.add(10, 9, true);
        bot.game.add(11, 9, true);
        bot.game.add(12, 9, true);

        bot
    }
}
