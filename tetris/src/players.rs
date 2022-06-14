use crate::game::*;

pub trait Player {

    fn make_move(&mut self) {
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
    Hold,

}

pub fn do_move_list(game: &mut Game, commands: MoveList) {
    for command in commands {
        do_command(game, command);
    }
}

fn do_command(game: &mut Game, command: Command) {
    match command {
        Command::MoveLeft => game.piece_left(),
        Command::MoveRight => game.piece_right(),
        Command::SoftDrop => game.piece_soft_drop(),
        Command::RotateCW => game.piece_rotate_cw(),
        Command::RotateCCW => game.piece_rotate_ccw(),
        Command::Rotate180 => game.piece_rotate_180(),
        Command::HardDrop => game.piece_hard_drop(true).is_ok(),
        Command::Hold => {
            game.hold();
            true
        }
        Command::None => false,
    };
}
