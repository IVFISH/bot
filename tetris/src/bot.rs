use crate::{Game, MoveVector, Placement};
use crate::players::*;
use crate::placement::piece_data::*;

use rand::Rng;

pub struct Bot {
    pub(crate) game: Game,
}

#[derive(Copy, Clone, Debug)]
pub enum Command {
    None,
    MoveLeft,
    MoveRight,
    SoftDrop,
    RotateCW,
    RotateCCW,
    Rotate180,
    HardDrop,

}

type MoveList = Vec<Command>;

impl Default for Bot {
    fn default() -> Self {
        Self {
            game: Game::new(None)
        }
    }
}

impl Player for Bot {
    fn make_move(&mut self) {
        let mut moves = self.all_moves();
        let num = rand::thread_rng().gen_range(0..moves.len());

        let mut action = moves.remove(num);
        action.push(Command::HardDrop);

        self.do_move_list(action);
    }

    fn get_next_move(&self) {
        todo!()
    }

    fn score_board(&self) -> i32 {
        todo!()
    }

    fn score(&self) -> i32 {
        todo!()
    }
}

impl Bot {
    pub fn all_moves(&mut self) -> Vec<MoveList> {
        let (moves, used) = self.find_trivial();
        let (moves, used) = self.add_tucks_to_trivial(moves, used);
        let moves = self.add_spins_to_tucks_and_trivial(moves, used);

        moves
    }

    fn find_trivial(&mut self) -> (Vec<MoveList>, Vec<Placement>) {
        let mut trivial_moves = Vec::new();
        let mut trivial_placements = Vec::new();

        let rotations = [Command::None, Command::RotateCW, Command::Rotate180, Command::RotateCCW];
        for rotation in rotations {
            self.game.active_piece.move_center_to_column(0);
            for col in 0..10 {
                if self.game.valid_location_for_active() {
                    let mut inputs: MoveList = vec![rotation];
                    inputs.append(Bot::column_to_move_list(col).as_mut());
                    trivial_moves.push(inputs);

                    self.game.piece_soft_drop();
                    trivial_placements.push(self.game.active_piece.clone());
                }

                self.game.active_piece.move_by_vector(MoveVector(0, 1));
            }
            self.game.active_piece.rotate(1);
        }

        (trivial_moves, trivial_placements)
    }

    fn add_tucks_to_trivial(&mut self, mut trivial: Vec<MoveList>, mut used_placements: Vec<Placement>)
                            -> (Vec<MoveList>, Vec<Placement>) {
        (trivial, used_placements)
    }

    fn add_spins_to_tucks_and_trivial(&mut self, trivial_and_tucks: Vec<MoveList>,
                                      used_placements: Vec<Placement>) -> Vec<MoveList> {
        trivial_and_tucks
    }

    fn do_command(&mut self, command: Command) {
        match command {
            Command::MoveLeft => self.game.piece_left(),
            Command::MoveRight => self.game.piece_right(),
            Command::SoftDrop => self.game.piece_soft_drop(),
            Command::RotateCW => self.game.piece_rotate_cw(),
            Command::RotateCCW => self.game.piece_rotate_ccw(),
            Command::Rotate180 => self.game.piece_rotate_180(),
            Command::HardDrop => self.game.piece_hard_drop(true).is_ok(),
            Command::None => false,
        };
    }

    fn do_move_list(&mut self, commands: MoveList) {
        println!("{:?}", commands);
        println!("{}", self.game);
        for command in commands {
            self.do_command(command);
        }
    }

    fn column_to_move_list(col: usize) -> MoveList {
        if col == SPAWN_COL {
            return vec![Command::None];
        }
        if col < SPAWN_COL {
            return vec![Command::MoveLeft; SPAWN_COL - col];
        }
        return vec![Command::MoveRight; col - SPAWN_COL];
    }
}

