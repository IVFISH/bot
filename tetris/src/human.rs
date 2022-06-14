use crate::game::*;
use crate::players::*;
use crate::placement::*;
use crate::placement::piece_data::*;
use std::fmt::{Display, Formatter};


pub struct Human {
    pub game: Game,
    pub nextMove: Option<String>
}

impl Display for Human {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.game)?;
        Ok(())
    }
}

impl Default for Human {
    fn default() -> Self {
        Self {
            game: Game::new(None),
            nextMove: None
        }
    }
}

impl Player for Human {
    fn get_game(&mut self) -> &mut Game {
        &mut self.game
    }

    fn get_next_move(&mut self) -> MoveList {
        if let Some(eshan) = &self.nextMove {
            let out = vec!(string_to_command(eshan.clone()));
            self.nextMove = None;
            return out;
        }
        return vec!(Command::None);
    }
}

impl Human {
    pub fn set_next_move(&mut self, eshan: String) {
        self.nextMove = Some(eshan);
    }
}

pub fn string_to_command(command_str: String) -> Command {
    let command_str: &str = &command_str;

    match command_str {
        "MoveLeft" => Command::MoveLeft,
        "MoveRight" => Command::MoveRight,
        "RotateCW" => Command::RotateCW,
        "RotateCCW" => Command::RotateCCW,
        "Rotate180" => Command::Rotate180,
        "HardDrop" => Command::HardDrop,
        "SoftDrop" => Command::SoftDrop,
        "HoldPiece" => Command::Hold,
        _ => Command::None
    }
}
