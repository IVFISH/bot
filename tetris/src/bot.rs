use std::fmt::{Display, Formatter};
use std::iter::zip;
use serde_json::to_writer;
use serde::{Deserialize, Serialize};
use std::io::Write;

use polynomial::Polynomial;

use crate::communications::*;
use crate::game::*;
use crate::placement::piece_data::*;
use crate::placement::*;
use crate::players::*;

use rand::Rng;

pub struct Bot {
    game: Game,
    pub weight: Weights,
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
    fn get_game_mut(&mut self) -> &mut Game {
        &mut self.game
    }

    fn get_game(&self) -> &Game {
        &self.game
    }

    fn get_next_move(&mut self) -> MoveList {
        // use std::time::Instant;
        // let now = Instant::now();

        let (mut deep_moves, deep_placements) = self.all_moves_and_placements(3);

        // let elapsed = now.elapsed();
        // println!("Elapsed: {:.2?}", elapsed);
        let mut min_score = f32::INFINITY;
        let mut action = vec![];
        let mut best_place=vec![];

        for (moves, placement) in zip(deep_moves, deep_placements) {
            let score = self.score(placement.clone());
            // println!("{}", score);
            if score < min_score {
                best_place = placement;
                min_score = score;
                action = moves;
            }
        }

        let mut best_board = self.game.board.clone();
        for placement in &best_place {
            best_board.set_piece(&placement, false);
            best_board.clear_lines(true);
        }
        println!("LKAJSDLJd \n{}", best_board);
        println!("BEST MOVE: {:?}", action);
        println!("BEST PLACEMENT SEQ: \n{} \n{}", best_place[0], best_place[1]);

        self.game.reset_active_piece();
        action.push(HardDrop);
        action
    }
}

pub type Score = f32;

impl Bot {
    pub fn give_birth(&self) -> Self {
        Self {
            game: Game::new(None),
            weight: self.weight.mutate(),
        }
    }

    pub fn suggest_next_move(&mut self) -> Suggestion {
        let action = self.get_next_move();
        let action = move_list_to_string(&action);

        Suggestion {
            input_list: action,
            info: "".to_string(),
        }
    }

    pub fn suggest_and_move(&mut self) -> Suggestion {
        let action = self.get_next_move();
        let action = move_list_to_string(&action);

        let out = Suggestion {
            input_list: action,
            info: "".to_string(),
        };
        self.make_move();
        // println!("{}", self);
        // thread::sleep(time::Duration::from_millis(250));
        out
    }

    pub fn game_over(&mut self) {
        self.game.game_data.game_over = true;
    }

    pub fn score(&mut self, placements: Vec<Placement>) -> Score {

        // TODO: only clone board
        let mut gameclone = self.game.clone();
        let mut clone = gameclone.board;
        let mut versus_score: f32 = 0.0;
        let mut cleared:usize = 0;

        for placement in placements {
            clone.set_piece(&placement, false);
            cleared = clone.clear_lines(true);
            // versus_score -= damage_calculations::calc_damage(
            //     &mut gameclone.game_data,
            //     attack_type(clone.get_t_spin_type(placement), cleared),
            //     cleared) as f32;
        }
        self.score_board(&clone)// + versus_score*10.0 + (cleared*6) as f32//+ self.score_versus()
    }

    fn score_board(&self, board: &Board) -> Score {
        let out = self.get_holes_and_cell_covered_score(board)
            + self.get_height_score(board)
            + self.get_height_differences_score(board);

        out
    }

    fn score_versus(&mut self) -> Score {
        let combo_score = self.weight.combo_weight.eval(self.game.game_data.combo as f32);
        let b2b = self.weight.b2b_weight.eval(self.game.game_data.b2b as f32);
        let attack = self.weight.damage_weight.eval(self.game.game_data.last_sent as f32);
        let clear = self.weight.clear_weight.eval(self.game.game_data.last_cleared as f32);

        return combo_score + b2b + attack + clear;
    }

    fn get_height_differences_score(&self, board: &Board) -> f32 {
        let mut out = board
            .get_adjacent_height_differences()
            .iter()
            .map(|x| {
                self.weight
                    .adjacent_height_differences_weight
                    .eval(*x as f32)
            })
            .sum();

        out += self
            .weight
            .total_height_difference_weight
            .eval(board.get_total_height_differences() as f32);
        out
    }

    pub fn get_height_score(&self, board: &Board) -> f32 {
        let total_height = board.max_filled_height();
        self.weight.height_weight.eval(total_height as f32)
    }

    pub fn get_holes_and_cell_covered_score(&self, board: &Board) -> f32 {
        let mut out = 0.0;

        let (holes_t, holes_w, covered) = board.holes_and_cell_covered();

        out += self.weight.num_hole_total_weight.eval(holes_t as f32);
        out += self.weight.num_hole_weighted_weight.eval(holes_w as f32);
        out += self.weight.cell_covered_weight.eval(covered as f32);

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

    pub fn all_moves_and_placements(&mut self, num_moves: usize) -> (Vec<MoveList>, Vec<Vec<Placement>>) {
        let mut dummy = self.game.clone();
        let (mut moves, mut placements) = Bot::move_placement_1d(&mut dummy);
        let mut placements: Vec<Vec<Placement>> = placements.iter().map(
            |x| vec![*x])
            .collect();

        if num_moves <= 1 {
            return (moves, placements)
        }
        let mut outmoves = vec![];
        let mut outplace = vec![];

        let mut index = 0;
        while placements[index].len() < num_moves {
            let game_save = dummy.clone();
            for p in placements[index].clone() {
                dummy.active_piece = p.clone();
                dummy.piece_hard_drop(true);
                // dummy.board.clear_lines(true);
            };

            // dummy.active_piece.piece_type = dummy.piece_queue.peek();

            let (_, mut add_placements) = Bot::move_placement_1d(&mut dummy);

            //TODO only keep the top x add_placements (pruning)

            for i in 0..add_placements.len() {
                moves.push(moves[index].clone());
                let mut place = placements[index].clone();
                place.push(add_placements[i]);
                if placements[index].len() >= num_moves - 1 {
                    placements.push(place.clone());
                    outmoves.push(moves[index].clone());
                    outplace.push(place.clone());
                }
                else {
                    placements.push(place);
                }
            }

            index += 1;
            dummy = game_save;
        }
        (outmoves, outplace)
    }

    pub fn move_placement_1d(game: &mut Game) -> (Vec<MoveList>, Vec<Placement>) {
        let start_piece = game.get_active_piece_type();
        let hold_piece;

        if let Some(piece) = game.hold_piece {
            hold_piece = piece
        } else {
            hold_piece = game.piece_queue_peek();
        }

        let (mut moves, mut used) = Bot::find_trivial(game, false);
        let (mut moves, mut used) = Bot::add_non_trivial(game, moves, used);

        game.active_piece = Placement::new(hold_piece);

        let (hold_moves, hold_used) = Bot::find_trivial(game,true);
        let (hold_moves, hold_used) = Bot::add_non_trivial(game, hold_moves, hold_used);

        moves.extend(hold_moves);
        used.extend(hold_used);

        game.active_piece = Placement::new(start_piece);

        (moves, used)
    }

    pub fn show_all_placements_on_input(&mut self, clear: bool) {
        use std::io;

        let (mut all_moves, mut all_placements) = self.all_moves_and_placements(100);
        println!("{}", all_placements.len());
        let all_placements = all_placements;

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

                let placement = &all_placements.get(index).unwrap().clone().pop().unwrap();
                self.show_placement(&placement, clear, &start);
            } else if input == String::from(".") {
                index += 1;
                index %= num_moves;
                println!("DEPTH IS: {}", all_placements.get(index).unwrap().len());
                let placement = &all_placements.get(index).unwrap().clone().pop().unwrap();
                self.show_placement(&placement, clear, &start);
            } else if input == String::from("exit") {
                break;
            } else if input == String::from("make move") {
                self.game.reset_active_piece();
                self.make_move();
                println!("{}", self.game);
                break;
            } else if input == String::from("do current move") {
                println!("{:?}", &all_moves[index]);
                println!("{:?}", all_placements.get(index));
                self.game.reset_active_piece();
                let mut muvs = &mut all_moves[index].clone();
                muvs.push(Command::HardDrop);
                self.do_moves(muvs);
                println!("{}", self.game);
                break;
            } else if input == String::from("eval") {
                println!("height: {}, \
                          holes 1: {}, holes 2 {}: cells covered: {}\n\
                          adjacent height differences: {:?}, total height difference: {}, ",
                         self.game.board.max_filled_height(),
                         self.game.board.holes_and_cell_covered().0, self.game.board.holes_and_cell_covered().1, self.game.board.holes_and_cell_covered().2,
                         self.game.board.get_adjacent_height_differences(), self.game.board.get_total_height_differences());
            }
        }
    }

    fn show_placement(&mut self, mut target_placement: &Placement, clear: bool, start: &Placement) {
        if clear {
            print!("{}[2J", 27 as char);
        }

        self.game.active_piece = target_placement.clone();
        println!("SCORE IS: {}", self.score_board(&self.game.board));
        println!("{}", self);
        self.game.active_piece = start.clone();
    }

    fn do_undo_action(
        &mut self,
        action: fn(&mut Game) -> bool,
        command: Command,
        current_move: &Vec<Command>,
        used_placements: &Vec<Placement>,
    ) -> (Vec<MoveList>, Vec<Placement>) {
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

            self.game.active_piece_soft_drop();

            if Bot::new_placement(&self.game.active_piece, &used_placements)
                && Bot::new_placement(&self.game.active_piece, &added_used)
            {
                added_moves.push(add_list.clone());
                added_used.push(self.game.active_piece.clone());
                continue;
            }

            break;
        }

        self.game.active_piece = save;

        (added_moves, added_used)
    }

    fn add_to_moves(
        mut moves: Vec<MoveList>,
        base_move: &MoveList,
        add_move: &mut MoveList,
    ) -> Vec<MoveList> {
        let mut to_add = base_move.clone();
        to_add.append(add_move);
        moves.push(to_add);
        moves
    }

    fn add_to_placements(
        mut placements: Vec<Placement>,
        add_placement: &Placement,
    ) -> Vec<Placement> {
        placements.push(add_placement.clone());
        return placements;
    }

    fn find_trivial(game: &mut Game, hold: bool) -> (Vec<MoveList>, Vec<Placement>) {
        let mut trivial_moves = Vec::new();
        let mut trivial_placements = Vec::new();

        let rotations = [
            Command::None,
            Command::RotateCW,
            Command::Rotate180,
            Command::RotateCCW,
        ];

        let row = game.active_piece.center.row;

        for direction in 0..NUM_ROTATE_STATES {
            // TODO: PROBABLY SIMPLIFY
            game.reset_active_piece();

            if !game.active_piece_rotate_direction(direction) {
                continue;
            }

            let mut base_move;
            if hold {
                base_move = vec![Command::Hold, rotations[direction]];
            } else {
                base_move = vec![rotations[direction]];
            }

            game.active_piece_soft_drop();
            trivial_placements =
                Bot::add_to_placements(trivial_placements, &game.active_piece);
            trivial_moves =
                Bot::add_to_moves(trivial_moves, &base_move, &mut vec![Command::SoftDrop]);
            game.active_piece.center.row = row;

            let mut add_moves = vec![];
            let mut counter = 0;
            while game.active_piece_left() {
                counter += 1;
                for _ in 0..counter {
                    add_moves.push(Command::MoveLeft);
                }

                game.active_piece_soft_drop();
                trivial_placements =
                    Bot::add_to_placements(trivial_placements, &game.active_piece);
                game.active_piece.center.row = row;

                add_moves.push(Command::SoftDrop);
                trivial_moves = Bot::add_to_moves(trivial_moves, &base_move, &mut add_moves);
                // add_moves.pop();
            }

            game.reset_active_piece();
            game.active_piece_rotate_direction(direction);

            let mut add_moves = vec![];
            let mut counter = 0;
            while game.active_piece_right() {
                counter += 1;
                for _ in 0..counter {
                    add_moves.push(Command::MoveRight);
                }

                game.active_piece_soft_drop();
                trivial_placements =
                    Bot::add_to_placements(trivial_placements, &game.active_piece);
                game.active_piece.center.row = row;

                add_moves.push(Command::SoftDrop);
                trivial_moves = Bot::add_to_moves(trivial_moves, &base_move, &mut add_moves);
                add_moves.pop();
            }
        }

        game.reset_active_piece();
        (trivial_moves, trivial_placements)
    }

    fn find_non_trivial(
        game: &mut Game,
        mut moove: MoveList,
        move_list: &mut Vec<MoveList>,
        placement_list: &mut Vec<Placement>,
        placement: Placement) {


        // TODO: move these somewhere
        let commands = [
            Command::MoveRight,
            Command::MoveLeft,
            Command::RotateCW,
            Command::RotateCCW,
            Command::Rotate180,
        ];

        // "SHADOW" IMPLEMENTATION
        let actions = [
            Game::return_piece_right,
            Game::return_piece_left,
            Game::ret_piece_rotate_cw,
            Game::ret_piece_rotate_ccw,
            Game::ret_piece_rotate_180,
        ];

        for (command, action) in zip(commands, actions) {
            let mut copy = action(&game, &placement);
            game.piece_soft_drop(&mut copy);

            // base case
            if !Bot::new_placement(&copy, placement_list) {
                continue;
            }
            // this is a new placement

            // finish movelist, add SD
            moove.push(command);
            moove.push(SoftDrop);

            // added to list of moves and placements
            move_list.push(moove.clone());
            placement_list.push(copy);

            // recurse
            Bot::find_non_trivial(game, moove.clone(), move_list, placement_list, copy);
        }

        // MUTATING IMPLEMENTATION
        // let actions = [
        //     Game::active_piece_rotate_cw,
        //     Game::active_piece_right,
        //     Game::active_piece_left,
        //     Game::active_piece_rotate_ccw,
        //     Game::active_piece_rotate_180,
        // ];
        //
        // for (command, action) in zip(commands, actions) {
        //     action(&mut self.game);
        //     Game::active_piece_soft_drop(&mut self.game); // TODO: use check_grounded instead of SDing every time
        //     if Bot::new_placement(&self.game.active_piece, placement_list) {
        //         moove.push(command);
        //         moove.push(SoftDrop);
        //
        //         move_list.push(moove.clone());
        //         placement_list.push(self.game.active_piece); // NEED TO CLONE?
        //
        //         self.find_non_trivial(
        //             moove.clone(),
        //             move_list,
        //             placement_list,
        //             self.game.active_piece,
        //         ); // NEED TO CLONE ACTIVE PIECE?
        //     }
        //     self.game.active_piece = placement;
        // }
    }

    fn add_non_trivial(
        game: &mut Game,
        mut move_list: Vec<MoveList>,
        mut placement_list: Vec<Placement>,
    ) -> (Vec<MoveList>, Vec<Placement>) {
        // OLD IMPLEMENTATION
        // while !unchecked_moves.is_empty() {
        //     let current_move = unchecked_moves.pop_front().unwrap();
        //     self.game.active_piece = unchecked_placements.pop_front().unwrap();
        //
        //     for (command, action) in zip(commands, actions) {
        //         let (new_trivial, new_used_placements) =
        //             self.do_undo_action(action, command, &current_move, &used_placements);
        //
        //         unchecked_moves.append(&mut VecDeque::from(new_trivial.clone()));
        //         unchecked_placements.append(&mut VecDeque::from(new_used_placements.clone()));
        //         trivial.extend(new_trivial);
        //         used_placements.extend(new_used_placements);
        //     }
        // }
        //
        // self.game.reset_active_piece();

        // println!("moves: {:?} \n len moves: {} \n placements: {:?} \n len placements: {}", move_list, move_list.len(), placement_list, placement_list.len());

        let trivialmoves = move_list.clone();
        let trivialplaces = placement_list.clone();

        for (amove, aplace) in zip(trivialmoves, trivialplaces) {
            Bot::find_non_trivial(game, amove, &mut move_list, &mut placement_list, aplace);
        }

        // println!("moves: {:?} \n len moves: {} \n placements: {:?} \n len placements: {}", move_list, move_list.len(), placement_list, placement_list.len());

        (move_list, placement_list)
    }

    fn new_placement(placement: &Placement, used_placements: &Vec<Placement>) -> bool {
        !used_placements.contains(placement)
    }

    fn column_to_move_list(col: usize, start_col: usize) -> MoveList {
        if col == start_col {
            return vec![Command::None];
        }
        if col < start_col {
            return vec![Command::MoveLeft; start_col - col];
        }
        return vec![Command::MoveRight; col - start_col];
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerializableWeights {
    pub weights: Vec<Vec<f32>>,
}

impl SerializableWeights {
    pub fn from_weight(weight: &Weights) -> Self {
        let weights = vec![
            weight.height_weight.data().try_into().unwrap(), weight.adjacent_height_differences_weight.data().try_into().unwrap(), weight.total_height_difference_weight.data().try_into().unwrap(),
            weight.num_hole_weighted_weight.data().try_into().unwrap(), weight.num_hole_total_weight.data().try_into().unwrap(), weight.cell_covered_weight.data().try_into().unwrap(),
            weight.b2b_weight.data().try_into().unwrap(), weight.combo_weight.data().try_into().unwrap(), weight.clear_weight.data().try_into().unwrap(),
            weight.damage_weight.data().try_into().unwrap(),
        ];
        Self {
            weights
        }
    }

    pub fn to_weight(&self) -> Weights {
        Weights {
            height_weight: Polynomial::new(self.weights[0].clone()),

            adjacent_height_differences_weight: Polynomial::new(Vec::from(self.weights[1].clone())),
            total_height_difference_weight: Polynomial::new(Vec::from(self.weights[2].clone())),
            num_hole_total_weight: Polynomial::new(Vec::from(self.weights[3].clone())),
            num_hole_weighted_weight: Polynomial::new(Vec::from(self.weights[4].clone())),
            cell_covered_weight: Polynomial::new(Vec::from(self.weights[5].clone())),

            b2b_weight: Polynomial::new(Vec::from(self.weights[6].clone())),
            combo_weight: Polynomial::new(Vec::from(self.weights[7].clone())),
            clear_weight: Polynomial::new(Vec::from(self.weights[8].clone())),
            damage_weight: Polynomial::new(Vec::from(self.weights[9].clone())),
        }
    }
}

pub struct Weights {
    pub height_weight: Polynomial<f32>,

    pub adjacent_height_differences_weight: Polynomial<f32>,
    pub total_height_difference_weight: Polynomial<f32>,
    pub num_hole_total_weight: Polynomial<f32>,
    pub num_hole_weighted_weight: Polynomial<f32>,
    pub cell_covered_weight: Polynomial<f32>,

    pub b2b_weight: Polynomial<f32>,
    pub combo_weight: Polynomial<f32>,
    pub damage_weight: Polynomial<f32>,
    pub clear_weight: Polynomial<f32>,
}

impl Default for Weights {
    fn default() -> Self {
        Self {
            height_weight: Polynomial::new(vec![0.0, -20.0, 1.0]),
            adjacent_height_differences_weight: Polynomial::new(vec![0.0, 2.0, 1.0]),
            total_height_difference_weight: Polynomial::new(vec![0.0, 1.0, 0.0]),
            num_hole_total_weight: Polynomial::new(vec![0.0, 10.0, 1.0]),
            num_hole_weighted_weight: Polynomial::new(vec![0.0, 0.0, 0.0]),
            cell_covered_weight: Polynomial::new(vec![0.0, 10.0, 0.0]),

            b2b_weight: Polynomial::new(vec![0.0, -1.0, -5.0]),
            combo_weight: Polynomial::new(vec![0.0, -2.0, -2.0]),
            damage_weight: Polynomial::new(vec![0.0, 0.0, -5.0]),
            clear_weight: Polynomial::new(vec![0.0, -5.0, 0.0])
        }
    }
}

impl Weights {
    pub const MAX_MUTATION: f32 = 0.1;

    pub fn to_json(&self, filename: String) {
        let weight = SerializableWeights::from_weight(self);
        let mut buffer = File::create(filename).unwrap();
        to_writer(buffer, &weight).unwrap();
    }

    pub fn from_json(filename: &str) -> Self {
        todo!()
    }

    pub fn mutate(&self) -> Self {
        Self {
            height_weight: Self::mutate_polynomial(&self.height_weight),

            adjacent_height_differences_weight: Self::mutate_polynomial(
                &self.adjacent_height_differences_weight,
            ),
            total_height_difference_weight: Self::mutate_polynomial(
                &self.total_height_difference_weight,
            ),
            num_hole_total_weight: Self::mutate_polynomial(&self.num_hole_total_weight),
            num_hole_weighted_weight: Self::mutate_polynomial(&self.num_hole_weighted_weight),
            cell_covered_weight: Self::mutate_polynomial(&self.cell_covered_weight),

            b2b_weight: Self::mutate_polynomial(&self.b2b_weight),
            combo_weight: Self::mutate_polynomial(&self.combo_weight),
            damage_weight: Self::mutate_polynomial(&self.damage_weight),
            clear_weight: Self::mutate_polynomial(&self.clear_weight),
        }
    }

    fn mutate_polynomial(poly: &Polynomial<f32>) -> Polynomial<f32> {
        Polynomial::new(
            poly.data()
                .iter()
                .map(|x| Weights::mutate_numb(*x))
                .collect(),
        )
    }

    fn mutate_numb(x: f32) -> f32 {
        let mut rng = rand::thread_rng();
        let n: f32 = rng.gen();
        let y: f32 = (n - 0.5) * Weights::MAX_MUTATION + 1.0;
        x * y
    }
}

use crate::bot::Command::{HardDrop, SoftDrop};
use std::{thread, time};
use std::fs::File;
use itertools::Itertools;
use crate::board::Board;
use crate::game::damage_calculations::attack_type;

pub fn bot_play() {
    let mut bot = Bot::default();

    let now = time::Instant::now();
    while !bot.game.game_over && bot.game.game_data.pieces_placed < 10000 {
        // clears the console
        // print!("{}[2J", 27 as char);

        // use std::time::Instant;
        // let now = Instant::now();

        bot.make_move();

        // let elapsed = now.elapsed();
        // println!("Elapsed: {:.2?}", elapsed);
        println!("{}", bot.game);
        // println!("height: {}", bot.game.board.max_filled_height());

        // thread::sleep(time::Duration::from_millis(0));
    }
    let elapsed = now.elapsed();
    println!("{:?}", elapsed);
    println!("{}", bot.game);
}

pub fn bot_debug() {
    let mut bot = bot_debug_board();
    bot.game.active_piece = Placement::new(0);
    bot.game.active_piece.piece_type = 0;
    // bot.game.add_garbage_to_board(8, true);

    // bot.show_all_placements_on_timer(true);
    // for _ in 0..30 {
    //     bot.make_move()
    // }
    // println!("{}", bot.game);
    // println!("{}", bot.score_board(false));

    println!("{}", bot.game);

    loop {
        bot.show_all_placements_on_input(true);
    }
}

fn bot_debug_board() -> Bot {
    let mut bot = Bot::default();
    bot.game.board.add(0, 0, true);
    bot.game.board.add(1, 2, true);
    // bot.game.board.add(1, 2, true);

    bot
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

        assert_eq!(bot.score_board(false), 344.0)
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

        assert_eq!(bot.score_board(false), 13762.0);
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

        bot.game.active_piece_das_right();
        bot.game.piece_hard_drop(true).expect("die");

        let (trivial, used) = bot.find_trivial(false);
        let trivial_only = trivial.clone();
        let (all_moves, _) = bot.add_non_trivial(trivial, used);

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

        let (moves, used) = bot.find_trivial(false);
        let (_, placements) = bot.add_non_trivial(moves, used);

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

        let (moves, used) = bot.find_trivial(false);
        let (_, placements) = bot.add_non_trivial(moves, used);

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

        let (moves, used) = bot.find_trivial(false);
        let (_, placements) = bot.add_non_trivial(moves, used);

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
            height_weight: Polynomial::new(vec![0.0, 2.0, 0.0]),
            adjacent_height_differences_weight: Polynomial::new(vec![0.0, 2.0, 1.0]),
            total_height_difference_weight: Polynomial::new(vec![0.0, 0.0, 0.0]),
            num_hole_total_weight: Polynomial::new(vec![0.0, 12.0, 0.0]),
            num_hole_weighted_weight: Polynomial::new(vec![0.0, 0.0, 0.0]),
            cell_covered_weight: Polynomial::new(vec![0.0, 10.0, 1.0]),

            b2b_weight: Polynomial::new(vec![0.0, -1.0, -5.0]),
            combo_weight: Polynomial::new(vec![0.0, -2.0, -2.0]),
            damage_weight: Polynomial::new(vec![0.0, 0.0]),
            clear_weight: Polynomial::new(vec![0.0, -5.0, -1.0]),
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
