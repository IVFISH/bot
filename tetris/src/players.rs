#![allow(dead_code)]

use crate::communications::Suggestion;
use crate::constants::bot_constants::*;
use crate::constants::types::*;
use crate::game::Game;
use num::clamp;
use std::fs;
use std::string::*;
use crate::constants::localbotgameplay::*;
use std::{thread, time};

pub trait Player {
    fn get_game(&self) -> &Game;
    fn get_game_mut(&mut self) -> &mut Game;
    fn get_next_move(&mut self) -> CommandList;

    fn make_move(&mut self) -> bool {
        if self.get_game().get_game_over() {
            return false;
        }
        let action = self.get_next_move();
        // println!("{:?}", action);
        do_move_list(self.get_game_mut(), action);

        // for local gameplay in cmd
        if ALLOWLOCALGAMEPLAY {
            let mut commsfile = fs::read_to_string(LOCALGAMEPLAYFILEPATH).expect("e");
            let mut garbage = commsfile.chars().nth(BOTNUM).expect("e").to_string().parse::<i16>().unwrap();
            let clamped_lines_sent = clamp(clamp(self.get_game().game_data.last_sent as i16 - garbage, 0, 9) + commsfile.chars().nth(BOTNUM2).expect("e").to_string().parse::<i16>().unwrap(), 0, 9);
            garbage = clamp(garbage - self.get_game().game_data.last_sent as i16, 0, 9);
            commsfile.replace_range(BOTNUM..BOTNUM+1, &garbage.to_string());
            commsfile.replace_range(BOTNUM2..BOTNUM2+1, &clamped_lines_sent.to_string());
            let _ = fs::write(LOCALGAMEPLAYFILEPATH, commsfile);

            // println!("{}", &garbage.to_string());
            // println!("{}", &clamped_lines_sent.to_string());
        }

        self.get_game_mut().board.update_board_garbage_amount();

        // println!("{}", clamped_lines_sent)
        // println!("{}", self.get_game().game_data.last_sent);
        true
    }

    fn make_n_moves(&mut self, n: usize) {
        for _ in 0..n {
            if !self.make_move() {
                break;
            }
        }
    }

    fn make_suggest_move(&mut self) -> Suggestion {
        if self.get_game().get_game_over() {
            return Suggestion {
                input_list: Vec::new(),
                info: "bot died".to_string(),
            };
        }
        let action = self.get_next_move();
        let out = Suggestion {
            input_list: Self::command_list_string(&action),
            info: "".to_string(),
        };
        // println!("{:?}", action);
        do_move_list(self.get_game_mut(), action);
        out
    }

    fn command_list_string(commands: &CommandList) -> Vec<String> {
        commands
            .iter()
            .filter(|&&command| command != Command::None)
            .map(|&command| command.to_string())
            .collect()
    }
}

pub fn do_move_list(game: &mut Game, commands: CommandList) {
    for command in commands {
        do_command(game, command);
    }
}

pub fn do_command(game: &mut Game, command: Command) -> bool {
    match command {
        Command::None => true,
        Command::MoveLeft => game.active_left(),
        Command::MoveRight => game.active_right(),
        Command::SoftDrop => game.active_drop(),
        Command::RotateCW => game.active_cw(),
        Command::RotateCCW => game.active_ccw(),
        Command::Rotate180 => game.active_180(),
        Command::Hold => {
            game.hold();
            true
        }
        Command::HardDrop => {
            // println!("{}", game);
            let game_over = !game.hard_drop();
            game.set_game_over(game_over);
            true
        }
    }
}
