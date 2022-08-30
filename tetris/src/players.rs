use crate::game::*;
use std::fmt::{Display, Formatter};
use crate::Weights;

pub trait Player {
    fn make_move(&mut self) -> bool {
        if self.get_game().game_over {
            return false;
        }

        let action = self.get_next_move();
        // println!("{:?}", action);
        self.get_game_mut().reset_active_piece();
        do_move_list(&mut self.get_game_mut(), action);
        true
    }

    fn do_moves(&mut self, moves: &MoveList) {
        do_move_list(&mut self.get_game_mut(), moves.clone());
    }

    fn get_game_mut(&mut self) -> &mut Game;
    fn get_game(&self) -> &Game;
    fn get_next_move(&mut self) -> MoveList;

    fn make_n_moves(&mut self, n: usize) {
        for _ in 0..n {
            if !self.make_move() {
                break;
            }
        }
    }
}

pub type MoveList = Vec<Command>;

pub fn move_list_to_string(move_list: &MoveList) -> Vec<String> {
    move_list
        .iter()
        .filter(|command| **command != Command::None)
        .map(|command| command.to_string())
        .collect()
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
    DasLeft,
    DasRight,
    Hold,
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::None => write!(f, "None")?,
            Command::MoveLeft => write!(f, "MoveLeft")?,
            Command::MoveRight => write!(f, "MoveRight")?,
            Command::SoftDrop => write!(f, "SoftDrop")?,
            Command::RotateCW => write!(f, "RotateCW")?,
            Command::RotateCCW => write!(f, "RotateCCW")?,
            Command::Rotate180 => write!(f, "Rotate180")?,
            Command::DasLeft => write!(f, "DasLeft")?,
            Command::DasRight => write!(f, "DasRight")?,
            Command::Hold => write!(f, "Hold")?,
            Command::HardDrop => write!(f, "HardDrop")?,
        }

        Ok(())
    }
}

pub fn do_move_list(game: &mut Game, commands: MoveList) {
    for command in commands {
        do_command(game, command)
    }
}

fn do_command(game: &mut Game, command: Command) {
    match command {
        Command::None => true,
        Command::MoveLeft => game.active_piece_left(),
        Command::MoveRight => game.active_piece_right(),
        Command::SoftDrop => game.active_piece_soft_drop(),
        Command::RotateCW => game.active_piece_rotate_cw(),
        Command::RotateCCW => game.active_piece_rotate_ccw(),
        Command::Rotate180 => game.active_piece_rotate_180(),
        Command::DasLeft => game.active_piece_das_left(),
        Command::DasRight => game.active_piece_das_right(),
        Command::Hold => game.hold(),
        Command::HardDrop => {
            game.game_over = game.piece_hard_drop(true).is_err();
            true
        }
    };
}
