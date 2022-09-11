use std::collections::VecDeque;
use crate::constants::bot_constants::Command;
use crate::game::Game;
use crate::point_vector::PointVector;

#[derive(Default)]
pub struct Controller {
    command_stack: VecDeque<Command>,
    translation_stack: VecDeque<PointVector>,
}

impl Controller {
    pub fn clear(&mut self) {
        self.command_stack.clear();
        self.translation_stack.clear();
    }

    pub fn execute(&mut self, command: Command, game: &mut Game) -> bool {
        let out = do_command(game, command);

        self.command_stack.push_front(command);

        let direction = rotation_to_number(command);
        if direction == 0 {
            self.translation_stack.push_front(PointVector(0, 0));
        } else {
            self.translation_stack.push_front(game.get_active_piece().get_kicks(direction)[game.get_active_piece().get_last_kick()]);
        }

        out
    }

    pub fn undo(&mut self, game: &mut Game) -> bool {
        if let Some(command) = self.command_stack.pop_front() {
            let translate = self.translation_stack.pop_front().unwrap();
            let command = complement_command(command);

            do_command(game, command);
            game.move_active(translate);
            return true;
        }
        false
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
            let game_over = !game.hard_drop();
            game.set_game_over(game_over);
            true
        }
    }
}

fn complement_command(command: Command) -> Command {
    match command {
        Command::MoveLeft => Command::MoveRight,
        Command::MoveRight => Command::MoveLeft,
        Command::RotateCW => Command::RotateCCW,
        Command::RotateCCW =>  Command::RotateCW,
        Command::Rotate180 => Command::Rotate180,
        _ => Command::None
    }
}

fn rotation_to_number(command: Command) -> usize {
    match command {
        Command::RotateCW => 1,
        Command::RotateCCW => 3,
        Command::Rotate180 => 2,
        _ => 0
    }
}