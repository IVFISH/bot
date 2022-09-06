#![allow(dead_code)]

use crate::board::Board;
use crate::constants::bot_constants::*;
use crate::constants::piece_constants::NUM_ROTATE_STATES;
use crate::constants::types::*;
use crate::game::{Game, GameData};
use crate::piece::Piece;
use crate::players::{do_command, Player};
use crate::weight::Weights;
use std::fmt::{Display, Formatter};
use std::iter::zip;

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
    fn get_game(&self) -> &Game {
        &self.game
    }

    fn get_game_mut(&mut self) -> &mut Game {
        &mut self.game
    }

    fn get_next_move(&mut self) -> CommandList {
        let (deep_moves, _, deep_scores) = self.move_placement_score(3, &self.weight.clone());
        let deep_scores: Vec<f32> = deep_scores
            .iter()
            .map(|(board, versus)| board + versus)
            .collect();
        // for (m, s) in zip(&deep_moves, &deep_scores) {
        //     println!("{:?} {}", m, s);
        // }
        let mut min_score = f32::INFINITY;
        let mut action = vec![];

        for (moves, score) in zip(deep_moves, deep_scores) {
            if score < min_score {
                min_score = score;
                action = moves;
            }
        }

        action.push(Command::HardDrop);
        action
    }
}

impl Bot {
    // move gen
    fn move_placement_score(
        &mut self,
        depth: usize,
        weight: &Weights,
    ) -> (MoveList, PlacementList, ScoreList) {
        let mut dummy = self.game.clone();
        Bot::move_placement_score_1d(&mut dummy, weight)
    }

    fn move_placement_score_1d(
        game: &mut Game,
        weight: &Weights,
    ) -> (MoveList, PlacementList, ScoreList) {
        let (mut moves, mut placements, mut scores) = Bot::trivial(game, false, weight);
        Bot::non_trivial(game, weight, &mut moves, &mut placements, &mut scores);
        (moves, placements, scores)
    }

    fn non_trivial(game: &mut Game, weight: &Weights, moves: &mut MoveList, placements: &mut PlacementList, scores: &mut ScoreList) {
        let max_index = moves.len();
        let mut index = 0;
        while index < max_index {
            Bot::non_trivial_recurse(game, weight, &mut moves[index].clone(), moves, placements, scores, &placements[index].clone());
            index += 1;
        }
    }

    fn non_trivial_recurse(game: &mut Game, weight: &Weights, new_move: &mut CommandList,
                           moves: &mut MoveList, placements: &mut PlacementList, scores: &mut ScoreList, start: &Piece) {

        for (command, action) in zip(COMMANDS, ACTIONS) {
            game.set_active_piece(start.clone());
            if !action(game) || !Bot::new_placement(&game.get_active_piece(), &placements){
                continue;
            }

            // println!("{}", game.board.display_with_active(&game.get_active_piece()));

            new_move.push(command);
            new_move.push(Command::SoftDrop);
            Bot::clone_and_extend(moves, placements, scores, new_move.clone(), game, weight);
            Bot::non_trivial_recurse(game, weight, new_move, moves, placements, scores, &start);
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
            Bot::clone_and_extend(&mut moves, &mut placements, &mut scores, base_move.clone(), game, weight);
            Bot::trivial_extend_direction(&mut moves, &mut placements, &mut scores, base_move.clone(), Command::MoveLeft, game, weight);
            game.reset_active_piece();
            game.active_piece_rotate_direction(direction);
            Bot::trivial_extend_direction(&mut moves, &mut placements, &mut scores, base_move.clone(), Command::MoveRight, game, weight);
            game.reset_active_piece();
        }

        (moves, placements, scores)
    }

    fn clone_and_extend(moves: &mut MoveList,
                        placements: &mut PlacementList,
                        scores: &mut ScoreList,
                        mut new_move: CommandList,
                        game: &mut Game,
                        weight: &Weights) {
        let piece = game.ret_active_drop();
        new_move.push(Command::SoftDrop);
        // println!("{:?}\n{}", new_move, game.board.display_with_active(&piece));
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
    fn score_game(game: Game, weights: &Weights, piece: &Piece) -> (Score, Score) {
        let versus_score = 0.0;
        let mut game = game.clone();
        game.board.set_piece(piece, true);
        (Bot::score_board(&game.board, weights), versus_score)
    }

    fn score_board(board: &Board, weights: &Weights) -> Score {
        Bot::get_holes_and_cell_covered_score(board, weights)
            + Bot::get_height_score(board, weights)
            + Bot::get_height_differences_score(board, weights)
    }

    fn score_versus(game_data: &GameData, weight: &Weights) -> Score {
        let combo_score = weight.combo_weight.eval(game_data.combo as f32);
        let b2b = weight.b2b_weight.eval(game_data.b2b as f32);
        let attack = weight.damage_weight.eval(game_data.last_sent as f32);
        let clear = weight.clear_weight.eval(game_data.last_cleared as f32);

        combo_score + b2b + attack + clear
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
