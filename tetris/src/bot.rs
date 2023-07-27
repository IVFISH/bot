#![allow(dead_code)]
use crate::board::Board;
use crate::constants::bot_constants::*;
use crate::constants::piece_constants::NUM_ROTATE_STATES;
use crate::constants::types::*;
use crate::game::{Game};
use crate::game::game_rules_and_data::*;
use crate::piece::Piece;
use crate::players::{do_command, do_move_list, Player};
use crate::weight::Weights;
use std::fmt::{Display, Formatter};
use std::iter::zip;
use std::{mem, thread, time};
use std::cmp::Ordering::Equal;
use futures_util::stream::iter;
use itertools::{izip, Itertools};
use crate::communications::Suggestion;
use crate::{Dependency, Opener, OpenerStatus, Point};
use crate::book::openers;
use crate::constants::board_constants::BOARD_WIDTH;
use crate::point_vector::PointVector;


pub struct Bot {
    game: Game,
    weight: Weights,
    opener: Opener
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
            opener: Opener::default(),
        }
    }
}

impl Player for Bot {
    fn get_game(&self) -> &Game {
        &self.game
    }

    fn get_game_mut(&mut self) -> &mut Game {
        &mut self.game
    }

    fn get_next_move(&mut self) -> CommandList {
        // R, C
        let mut action = vec![];

        if self.opener.status == OpenerStatus::New {
            let mut sequence = vec![self.get_game().active_piece.piece_type];
            sequence.append(&mut self.get_game().piece_queue.get_vec());
            self.opener.init(&sequence);
        }
        if self.opener.status == OpenerStatus::Active {
            match self.do_opener() {
                Ok(m) => {
                    action = m;
                    action.push(Command::HardDrop);
                    return action;
                },
                Err(_) => {
                    eprintln!("opener sequence terminated");
                    self.opener.status = OpenerStatus::Invalid
                }
            }
        }

        // thread::sleep(time::Duration::from_millis(250));

        let (deep_moves, places, deep_scores) = self.move_placement_score(11, &self.weight.clone());
        let deep_scores: Vec<f32> = deep_scores
            .iter()
            .map(|(board, versus)| board + versus)
            .collect();

        let mut min_score = f32::INFINITY;
        let mut p = PlacementList::new();

        for (moves, place, score) in izip!(deep_moves, places, deep_scores) {
            if score < min_score {
                min_score = score;
                action = moves;
                p = place;
            }
        }



        println!("{:?}", action);
        println!("{}", min_score);
        println!("{}", p[0]);
        if min_score < -10000.0{
            println!("{:?}", p);
        }

        action.push(Command::HardDrop);
        action
    }
}

impl Bot {
    // initialization
    pub fn new(game: Game) -> Self {
        Self {
            game,
            ..Default::default()
        }
    }

    // move gen
    fn command_list_string(commands: &CommandList) -> Vec<String> {
        commands
        .iter()
        .filter(|&&command| command != Command::None)
        .map(|&command| command.to_string())
        .collect()
    }

    pub fn moves_to_placement (
        game: &mut Game,
        piece: &Piece,
    ) -> Result<CommandList, usize> {
        let (moves, placements, _) = Bot::move_placement_score_1d(game, &Weights::default());
        for (m, p) in zip(moves, placements) {
            if &p == piece {
                return Ok(m);
            }
        }
        Err(0) // shitty error, make this better
    }

    pub fn moves_to_placements (
        game: &mut Game,
        bag: &[Piece; 7],
    ) -> Result<CommandList, usize> {
        let (moves, placements, _) = Bot::move_placement_score_1d(game, &Weights::default());
        for (m, p) in zip(moves, placements) {
            for piece in bag {
                if &p == piece {
                    return Ok(m);
                }
            }
        }
        Err(0) // shitty error, make this better
    }

    pub fn do_opener(&mut self) -> Result<CommandList, usize> {
        let mut sequence = vec![self.get_game().active_piece.piece_type];
        sequence.append(&mut self.get_game().piece_queue.get_vec());
        let mut placement = self.opener.next_placement(&sequence);
        placement.moved(PointVector(0 - self.get_game().game_data.lines_cleared as i8, 0));
        Bot::moves_to_placement(&mut self.get_game().clone(), &placement)
    }

    pub fn move_placement_score(
        &mut self,
        depth: usize,
        weights: &Weights,
        ) -> (MoveList, Vec<PlacementList>, ScoreList) {
            let mut dummy = self.game.clone();
            let (mut curr_moves, temp_placements, mut curr_scores) =
                Bot::move_placement_score_1d(&mut dummy, weights);

            let mut curr_placements: Vec<PlacementList> = temp_placements.into_iter().map(|x| vec!(x)).collect();

            if depth <= 1 {
                return (curr_moves, curr_placements, curr_scores);
            }

            let mut next_moves = MoveList::new();
            let mut next_placements= Vec::new();
            let mut next_scores = ScoreList::new();

            //pruning parameters
            let n = 200;
            let prune_depth = 1;

            for curr_depth in 1..depth {
                //pruning curr_mps
                if (curr_depth) % prune_depth == 0 {
                    //TODO! use multiple scoring functions
                    let combined_scores: Vec<Score> = curr_scores.clone().into_iter().map(|(versus, board)| versus + board).collect();

                    (curr_moves, curr_placements) =
                        zip(curr_moves, curr_placements)
                            .enumerate()
                            .sorted_by(|&(i1, _), &(i2, _)| combined_scores[i1].partial_cmp(&combined_scores[i2]).unwrap())
                            .take(n)
                            .map(|(_, p)| p)
                            .unzip();

                    curr_scores.sort_by(|(v1, b1), (v2, b2)| (v1 + b1).partial_cmp(&(v2 + b2)).unwrap());
                    curr_scores.truncate(n);
                }

                //generating next_mps
                for (one_move, placements, (_, versus)) in izip!(curr_moves.clone(), curr_placements.clone(), curr_scores.clone()) {
                    let mut dummy = dummy.clone();

                    //set dummy/cloned game
                    for p in &placements {
                        if dummy.active_piece.piece_type == p.piece_type {
                            dummy.set_active_piece(p.clone());
                            dummy.set_piece();
                        }
                        // need to hold
                        else {
                            dummy.hold();
                            assert_eq!(dummy.active_piece.piece_type, p.piece_type);
                            dummy.set_active_piece(p.clone());
                            dummy.set_piece();
                        }
                    }

                    let (_, add_placements, add_scores) =
                        Bot::move_placement_score_1d(&mut dummy, weights);

                    for (add_place, (board, add_versus)) in zip(add_placements, add_scores) {
                        let mut placements = placements.clone();
                        placements.push(add_place);

                        next_moves.push(one_move.clone());
                        next_placements.push(placements.clone());
                        next_scores.push((board, versus + add_versus));
                    }
                }
                curr_moves = mem::take(&mut next_moves);
                curr_placements = mem::take(&mut next_placements);
                curr_scores = mem::take(&mut next_scores);
            }
            (curr_moves, curr_placements, curr_scores)
        }

    pub fn move_placement_score_1d(
        game: &mut Game,
        weight: &Weights,
    ) -> (MoveList, PlacementList, ScoreList) {
        let (mut moves, mut placements, mut scores) =
            Bot::trivial(game, false, weight);
        Bot::non_trivial(game, weight, &mut moves, &mut placements, &mut scores);

        let hold_piece = game.get_hold_piece_or_next();

        if hold_piece.get_type() == game.get_active_piece().get_type() {
            return (moves, placements, scores);
        }

        let active_piece = game.get_active_piece().get_type();
        game.set_active_piece(hold_piece);
        let (mut hold_moves, mut hold_placements, mut hold_scores) =
            Bot::trivial(game, true, weight);
        Bot::non_trivial(
            game,
            weight,
            &mut hold_moves,
            &mut hold_placements,
            &mut hold_scores,
        );

        moves.extend(hold_moves);
        placements.extend(hold_placements);
        scores.extend(hold_scores);
        game.set_active_piece(Piece::new(active_piece));
        (moves, placements, scores)
    }

    fn non_trivial(
        game: &mut Game,
        weight: &Weights,
        moves: &mut MoveList,
        placements: &mut PlacementList,
        scores: &mut ScoreList,
    ) {
        let max_index = moves.len();
        let mut index = 0;
        while index < max_index {
            Bot::non_trivial_recurse(
                game,
                weight,
                &mut moves[index].clone(),
                moves,
                placements,
                scores,
                &placements[index].clone(),
            );
            index += 1;
        }
    }

    fn non_trivial_recurse(
        game: &mut Game,
        weight: &Weights,
        new_move: &mut CommandList,
        moves: &mut MoveList,
        placements: &mut PlacementList,
        scores: &mut ScoreList,
        start: &Piece,
    ) {
        for (command, action) in zip(COMMANDS, ACTIONS) {
            game.set_active_piece(start.clone());

            if !action(game) {
                continue;
            }

            let sd = game.active_drop();
            if !Bot::new_placement(&game.get_active_piece(), &placements) {
                continue;
            }
            new_move.push(command);
            if sd { new_move.push(Command::SoftDrop); }
            moves.push(new_move.clone());
            placements.push(game.clone().active_piece);
            scores.push(Bot::score_game(game.clone(), weight, &game.active_piece));
            Bot::non_trivial_recurse(
                game,
                weight,
                new_move,
                moves,
                placements,
                scores,
                &game.get_active_piece().clone(),
            );
        }
    }

    fn trivial(
        game: &mut Game,
        hold: bool,
        weight: &Weights,
    ) -> (MoveList, PlacementList, ScoreList) {
        let mut moves = Vec::with_capacity(40);
        let mut placements = Vec::with_capacity(40);
        let mut scores = Vec::with_capacity(40);

        for direction in 0..NUM_ROTATE_STATES {
            if !game.active_piece_rotate_direction(direction) {
                continue;
            }

            let mut base_move = Vec::with_capacity(8);
            if hold {
                base_move.push(Command::Hold);
            }
            base_move.push(ROTATIONS[direction]);
            Bot::clone_and_extend(
                &mut moves,
                &mut placements,
                &mut scores,
                base_move.clone(),
                game,
                weight,
            );
            Bot::trivial_extend_direction(
                &mut moves,
                &mut placements,
                &mut scores,
                base_move.clone(),
                Command::MoveLeft,
                game,
                weight,
            );
            game.reset_active_piece();
            game.active_piece_rotate_direction(direction);
            Bot::trivial_extend_direction(
                &mut moves,
                &mut placements,
                &mut scores,
                base_move.clone(),
                Command::MoveRight,
                game,
                weight,
            );
            game.reset_active_piece();
        }

        (moves, placements, scores)
    }

    fn clone_and_extend(
        moves: &mut MoveList,
        placements: &mut PlacementList,
        scores: &mut ScoreList,
        mut new_move: CommandList,
        game: &mut Game,
        weight: &Weights,
    ) {
        let piece = game.clone().ret_active_drop();
        new_move.push(Command::SoftDrop);
        moves.push(new_move);
        scores.push(Bot::score_game(game.clone(), weight, &piece));
        placements.push(piece);
    }
    fn trivial_extend_direction(
        moves: &mut MoveList,
        placements: &mut PlacementList,
        scores: &mut ScoreList,
        mut base_move: CommandList,
        command: Command,
        game: &mut Game,
        weight: &Weights,
    ) {
        while do_command(game, command) {
            base_move.push(command);
            Bot::clone_and_extend(moves, placements, scores, base_move.clone(), game, weight);
        }
    }

    fn new_placement(placement: &Piece, used_placements: &Vec<Piece>) -> bool {
        !used_placements.contains(placement)
    }

    // scoring
    fn score_game(mut game: Game, weights: &Weights, piece: &Piece) -> (Score, Score) {
        game.board.set_piece(piece);
        game.active_piece = piece.clone();
        game.update();

        //TODO: put all the logic in nice places (scorer class?)

        // PC (pseudo) PRUNING
        if true && (game.board.get_mino_count() % 2 == 0) {
            // pseudo pruning, gives a large penalty for unsolvable/hard to solve boards
            let penalty = (100000.0, Bot::score_board(&game.board, weights));

            let target_height = 4 + (game.board.get_mino_count() % 4);

            // limits search to target height
            if game.board.get_max_height() > target_height{
                return penalty
            }

            let parity = game.board.get_parities();

            // the number of pieces required to fill the board to target height
            let pieces_to_pc = (10*target_height - game.board.get_mino_count())/4;

            // TODO: use a set or take piece order into account

            // a queue of all the pieces available before the bot reaches target height
            let mut usable_queue = game.piece_queue.get_queue().clone();
            usable_queue.truncate(pieces_to_pc);
            usable_queue.push_back(game.active_piece.piece_type);
            if game.hold_piece.is_some(){
                usable_queue.push_back(game.hold_piece.unwrap());
            }

            // Checkerboard parity, solvability depends on if there is a playable T
            // TODO: Take line clears into account
            if parity.0 == false && (!usable_queue.contains(&6) || game.board.get_min_height() >= 2){
                return penalty
            }

            // Col Parity, solvability depends on if there is a playable L, J, or T
            // Not affected by line clears
            if parity.1 == false && !(usable_queue.contains(&1) || usable_queue.contains(&5) || usable_queue.contains(&6) || !game.board.get_min_height() >= 2){
                return penalty
            }
        }

        return (
            Bot::score_board(&game.board, weights),
            Bot::score_versus(&game.game_data, weights),
        )
    }

    fn score_board(board: &Board, weights: &Weights) -> Score {
        Bot::get_holes_and_cell_covered_score(board, weights)
            + Bot::get_height_score(board, weights)
            + Bot::get_height_differences_score(board, weights)
            + Bot::get_t_slot_score(board, weights)
    }

    fn score_versus(game_data: &GameData, weight: &Weights) -> Score {
        // let spin = Game::get_t_spin_type(piece, board);
        let combo_score = weight.combo_weight.eval(game_data.combo as f32);
        let b2b = weight.b2b_weight.eval(game_data.b2b as f32);
        let attack = weight.damage_weight.eval(game_data.last_sent as f32);
        let clear = weight.clear_weight.eval(game_data.last_cleared as f32);
        let pc = game_data.all_clear;

        let mut extra = 0.0;

        if pc {
            extra -= 1000000.0;
        }

        combo_score + b2b + attack + clear + extra
    }

    fn get_height_differences_score(board: &Board, weight: &Weights) -> f32 {
        let adjacent_score: f32 = board
            .get_adjacent_height_differences()
            .iter()
            .map(|&x| weight.adjacent_height_differences_weight.eval(x as f32))
            .sum();

        let total_score = weight
            .total_height_difference_weight
            .eval(board.get_max_height_difference() as f32);

        adjacent_score + total_score
    }

    pub(crate) fn get_t_slot_score(board: &Board, weight: &Weights) -> f32 {
        weight.t_slot_weight.eval(board.t_slot() as f32)
    }

    fn get_height_score(board: &Board, weight: &Weights) -> f32 {
        let total_height = board.get_max_height();
        weight.height_weight.eval(total_height as f32)
    }

    fn get_holes_and_cell_covered_score(board: &Board, weight: &Weights) -> f32 {
        let mut out = 0.0;

        let (holes_t, holes_w, covered) = board.holes_cell_covered();

        out += weight.num_hole_total_weight.eval(holes_t as f32);
        out += weight.num_hole_weighted_weight.eval(holes_w as f32);
        out += weight.cell_covered_weight.eval(covered as f32);

        out
    }
}
