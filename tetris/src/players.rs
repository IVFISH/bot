use crate::errors::GameError;
use crate::game::*;

pub trait Player {
    fn make_move(&mut self) {
        if self.get_game().game_over {
            return;
        }

        let action = self.get_next_move();
        do_move_list(&mut self.get_game(), action);
    }

    fn get_game(&mut self) -> &mut Game;
    fn get_next_move(&mut self) -> MoveList;
}

pub type MoveList = Vec<Command>;

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

pub fn do_move_list(game: &mut Game, commands: MoveList) {
    for command in commands {
        do_command(game, command)
    }
}

fn do_command(game: &mut Game, command: Command) {
    match command {
        Command::None => true,
        Command::MoveLeft => game.piece_left(),
        Command::MoveRight => game.piece_right(),
        Command::SoftDrop => game.piece_soft_drop(),
        Command::RotateCW => game.piece_rotate_cw(),
        Command::RotateCCW => game.piece_rotate_ccw(),
        Command::Rotate180 => game.piece_rotate_180(),
        Command::DasLeft => game.piece_das_left(),
        Command::DasRight => game.piece_das_right(),
        Command::Hold => {
            println!("HOLDING");
            game.hold()
        },

        Command::HardDrop => {
            game.game_over = game.piece_hard_drop(true).is_err();
            true
        }
    };
}
