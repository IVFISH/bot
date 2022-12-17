#![allow(dead_code)]

use crate::board::Board;
use crate::constants::bot_constants::*;
use crate::constants::piece_constants::NUM_ROTATE_STATES;
use crate::constants::types::*;
use crate::game::{Game};
use crate::game::game_rules_and_data::*;
use crate::piece::Piece;
use crate::players::{do_command, Player};
use crate::weight::Weights;
use std::fmt::{Display, Formatter};
use std::iter::zip;
use crate::communications::Suggestion;
use crate::Point;

pub struct Bot {
    game: Game,
    weight: Weights,
    opener: bool,
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
            opener: true,
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

        if self.opener {
            match self.do_opener() {
                Ok(m) => {
                    action = m;
                    action.push(Command::HardDrop);
                    return action;
                },
                Err(_) => {
                    eprintln!("opener sequence terminated");
                    self.opener = false
                }
            }
        }

        let (deep_moves, _, deep_scores) = self.move_placement_score(2, &self.weight.clone());
        let deep_scores: Vec<f32> = deep_scores
            .iter()
            .map(|(board, versus)| board + versus)
            .collect();

        let mut min_score = f32::INFINITY;

        for (moves, score) in zip(deep_moves, deep_scores) {
            if score < min_score {
                min_score = score;
                action = moves;
            }
        }

        println!("{:?}", action);
        println!("{}", min_score);

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
        const TKI: [Piece; 7] = [
            Piece{piece_type: 0, rotation_state: 0, center: Point(1,4), last_kick: 0},
            Piece{piece_type: 1, rotation_state: 1, center: Point(1,0), last_kick: 0},
            Piece{piece_type: 2, rotation_state: 0, center: Point(0,8), last_kick: 0},
            Piece{piece_type: 3, rotation_state: 1, center: Point(1,6), last_kick: 0},
            Piece{piece_type: 4, rotation_state: 0, center: Point(0,4), last_kick: 0},
            Piece{piece_type: 5, rotation_state: 2, center: Point(3,4), last_kick: 0},
            Piece{piece_type: 8, rotation_state: 0, center: Point(0,4), last_kick: 0}, // always invalid
        ];
        return Bot::moves_to_placements(self.get_game_mut(), &TKI);
    }

    pub fn move_placement_score(
        &mut self,
        depth: usize,
        weights: &Weights,
        ) -> (MoveList, Vec<PlacementList>, ScoreList) {
            let mut dummy = self.game.clone();
            let (mut moves, mut placementss, mut scores) =
                Bot::move_placement_score_1d(&mut dummy, weights);

            let mut placements = vec![];
            for place in placementss{
                placements.push(vec!(place))
            }
            // let mut placements: Vec<PlacementList> = placements.iter().map(|x| vec![*x]).collect();

            if depth <= 1 {
                return (moves, placements, scores);
            }

            let mut outmoves = MoveList::new();
            let mut outplace : Vec<PlacementList> = Vec::new();
            let mut outscores : ScoreList = Vec::new();

            let mut index = 0;
            while placements[index].len() < depth {
                let game_save = dummy.clone();
                for p in placements[index].clone() {
                    dummy.active_piece = p;
                    // if dummy.hard_drop() {
                    //     continue;
                    // }
                    dummy.set_piece();
                }

                let (_, mut add_placements, mut add_scores) =
                    Bot::move_placement_score_1d(&mut dummy, weights);

                //TODO only keep the top x add_placements (pruning)

                for i in 0..add_placements.len() {
                    let mut place : Vec<Piece> = placements[index].clone();

                    let mut versus_score : Score = scores[index].1;
                    versus_score += add_scores[i].1;

                    moves.push(moves[index].clone());

                    place.push(add_placements[i].clone());
                    placements.push(place.clone());

                    scores.push((add_scores[i].0, versus_score));

                    if placements[index].len() >= depth - 1 {
                        outmoves.push(moves[index].clone());
                        outplace.push(place.clone());
                        outscores.push((add_scores[i].0, versus_score.clone()));

                        // println!("{}", versus_score);
                    }
                }

                // if placements[index].len() >= num_moves - 1 {
                //     outscores.extend(add_scores);
                // }

                index += 1;
                dummy = game_save;
            }
            (outmoves, outplace, outscores)
        }

    pub(crate) fn move_placement_score_1d(
        game: &mut Game,
        weight: &Weights,
    ) -> (MoveList, PlacementList, ScoreList) {
        let (mut moves, mut placements, mut scores) = Bot::trivial(game, false, weight);
        Bot::non_trivial(game, weight, &mut moves, &mut placements, &mut scores);

        let hold_piece = game.get_hold_piece_or_next();

        // return (moves, placements, scores);

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
            if !(action(game)
                && game.board.piece_grounded(&game.get_active_piece())
                && Bot::new_placement(&game.get_active_piece(), &placements))
            {
                continue;
            }
            new_move.push(command);
            Bot::clone_and_extend(moves, placements, scores, new_move.clone(), game, weight);
            new_move.push(Command::SoftDrop);
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
        let piece = game.ret_active_drop();
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
        (
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

        // println!("{}, {}", clear, attack);

        if game_data.last_cleared as usize == (2 as usize) && game_data.last_sent as u8 == (4 as u8){
            // println!("GOOD");
            extra -= 10000.0;
        }

        if pc {
            extra -= 100000.0;
        }

        if game_data.last_cleared as usize == (1 as usize) && game_data.last_sent as u8 == (2 as u8){
            // println!("GOOD");
            extra -= 100.0;
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
        weight.t_slot_weight.eval(board.t_slot(board.get_max_height()) as f32)
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
