use crate::game::*;
use crate::players::*;
use crate::placement::*;
use crate::placement::piece_data::*;


struct Human {
    game: Game
}

impl Player for Human {
    fn get_game(&mut self) -> &mut Game {
        &mut self.game
    }

    fn get_next_move(&mut self) -> MoveList {
        todo!()
    }
}

fn string_to_command(command_str: String) -> Command {
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