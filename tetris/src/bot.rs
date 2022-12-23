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
use std::{thread, time};
use std::cmp::Ordering::Equal;
use futures_util::stream::iter;
use crate::communications::Suggestion;
use crate::{Dependency, Opener, OpenerStatus, Point};
use crate::book::openers;
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

        let (deep_moves, _, deep_scores) = self.move_placement_score(7, &self.weight.clone());
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
        let mut sequence = vec![self.get_game().active_piece.piece_type];
        sequence.append(&mut self.get_game().piece_queue.get_vec());
        let mut placement = self.opener.next_placement(&sequence);
        placement.moved(PointVector(0 - self.get_game().game_data.lines_cleared as i8, 0));
        println!("{}", placement);
        Bot::moves_to_placement(&mut self.get_game().clone(), &placement)
    }

    pub fn move_placement_score(
        &mut self,
        depth: usize,
        weights: &Weights,
        ) -> (MoveList, Vec<PlacementList>, ScoreList) {
            let now = time::Instant::now();
            let mut dummy = self.game.clone();
            let (mvs, plcmnts, scrs) =
                Bot::move_placement_score_1d(&mut dummy, weights);

            //# placements kept after prune for versus and scores (2n placements are kept)
            let n = 5;

            let mut moves = vec![Vec::new(); 2];
            moves[0] = mvs;

            let mut placements = vec![Vec::new(); 2];
            placements[0] = plcmnts.into_iter().map(|x| vec!(x)).collect();

            let mut scores = vec![Vec::new(); 2];
            scores[0] = scrs;

            if depth <= 1 {
                return (moves[0].clone(), placements[0].clone(), scores[0].clone());
            }

            let mut curr_depth = 0;
            let mut index = 0;
            let prune_depth = 2;

            while curr_depth < depth-1 {
                // println!("LEEEROOYYY");
                // println!("{}, {}", placements[0].len(), curr_depth);
                while index < placements[0].len() {
                    // println!("JENKINS");
                    //set save
                    let game_save = dummy.clone();

                    //set dummy/cloned game
                    for p in placements[0][index].clone() {
                        dummy.active_piece = p;
                        dummy.set_piece();
                    }

                    let (_, mut add_placements, mut add_scores) =
                        Bot::move_placement_score_1d(&mut dummy, weights);

                    for i in 0..add_placements.len() {
                        let mut place = placements[0][index].clone();

                        let mut versus_score = scores[0][index].1;
                        versus_score += add_scores[i].1;

                        let m = moves[0][index].clone();
                        moves[1].push(m);

                        place.push(add_placements[i].clone());
                        placements[1].push(place.clone());

                        scores[1].push((add_scores[i].0, versus_score));
                    }
                    index += 1;
                    dummy = game_save;
                }
                //pruning
                if (curr_depth) % prune_depth == 0 {
                    let combined_scores: Vec<Score> = scores[1].clone().into_iter().map(|(versus, board)| versus + board).collect();

                    let mut enumerated_placements: Vec<(usize, PlacementList)> = placements[1].clone().into_iter().enumerate().collect();
                    enumerated_placements.sort_by(|(i1, _), (i2, _)| combined_scores[*i1].partial_cmp(&combined_scores[*i2]).unwrap());
                    placements[1] = enumerated_placements[..n].into_iter().map(|(_, placement_list)| placement_list.clone()).collect();

                    let mut enumerated_moves: Vec<(usize, CommandList)> = moves[1].clone().into_iter().enumerate().collect();
                    enumerated_moves.sort_by(|(i1, _), (i2, _)| combined_scores[*i1].partial_cmp(&combined_scores[*i2]).unwrap());
                    moves[1] = enumerated_moves[..n].into_iter().map(|(_, command_list)| command_list.clone()).collect();

                    scores[1].sort_by(|(v1, b1), (v2, b2)| (v1 + b1).partial_cmp(&(v2 + b2)).unwrap());
                    scores[1] = scores[1][..n].to_vec();
                }
                // println!("INNER TIME: {}", now.elapsed().as_millis());
                moves.remove(0);
                moves.push(Vec::new());
                placements.remove(0);
                placements.push(Vec::new());
                scores.remove(0);
                scores.push(Vec::new());
                index = 0;
                curr_depth += 1;
            }
            // println!("MPS: {:?}, {:?}, {:?}", moves[0].len(), placements[0].len(), scores[0].len());
            // println!("TIME: {}", now.elapsed().as_millis());
            (moves[0].clone(), placements[0].clone(), scores[0].clone())
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
        // let mut i = 0;
        // while i < placements.len(){
        //     if !game.board.piece_grounded(&placements[i]) {
        //         moves.remove(i);
        //         placements.remove(i);
        //         scores.remove(i);
        //     }
        //     else { i += 1; }
        // }
        // println!("MPS: {:?}, {:?}, {:?}", moves.len(), placements.len(), scores.len());
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
        // println!("{}", zip(COMMANDS, ACTIONS).len());
        for (command, action) in zip(COMMANDS, ACTIONS) {
            game.set_active_piece(start.clone());
            // let mut c = false;
            // if game.active_piece.piece_type == 6 && game.active_piece.center.0 < 7 && game.active_piece.center.1 < 3 {
            //     println!("bababa \n{}", game);
            //     println!("{}", command);
            //     println!("{:?}", game.active_piece);
            //     if command == Command::RotateCCW {
            //         c = true;
            //         println!("c IS TRUE");
            //     }
            // }
            if !action(game) {
                // if c {
                //     println!("SAD \n{}", game);
                //     println!("{}", command);
                //     println!("{:?}", game.active_piece);
                // }
                continue;
            }
            // if c {
            //     println!("ababa \n{}", game);
            //     println!("{}", command);
            //     println!("{:?}", game.active_piece);
            // }
            let sd = game.active_drop();
            if !Bot::new_placement(&game.get_active_piece(), &placements) {
                continue;
            }
            new_move.push(command);
            if sd { new_move.push(Command::SoftDrop); }
            moves.push(new_move.clone());
            placements.push(game.clone().active_piece);
            scores.push(Bot::score_game(game.clone(), weight, &game.active_piece));
            // Bot::clone_and_extend(moves, placements, scores, new_move.clone(), game, weight);
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
