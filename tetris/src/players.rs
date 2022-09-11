#![allow(dead_code)]

use crate::constants::types::*;
use crate::game::Game;
use crate::command_controller::do_command;

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
        true
    }

    fn make_n_moves(&mut self, n: usize) {
        for _ in 0..n {
            if !self.make_move() {
                break;
            }
        }
    }
}

pub fn do_move_list(game: &mut Game, commands: CommandList) {
    for command in commands {
        do_command(game, command);
    }
}
