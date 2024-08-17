#![allow(dead_code)]

use std::fmt::{Display, Formatter};

use crate::board::Board;
use crate::command::Command;
use crate::constants::board_constants::{BOARD_WIDTH, VISIBLE_BOARD_HEIGHT};
use crate::constants::piece_constants::{SpinType, NUM_ROTATE_STATES, PIECE_ROTATIONS, PIECE_T};
use crate::piece::Piece;

#[derive(Debug)]
pub struct Controller<'a> {
    pub piece: &'a mut Piece,
    pub board: &'a Board,
    cmaps: CollisionMaps,
    commands: Vec<Command>,
    pieces: Vec<Piece>,
}

impl<'a> Controller<'a> {
    // constructor ------------------------------
    /// creates a new controller from a board and piece reference
    pub fn new(piece: &'a mut Piece, board: &'a Board) -> Self {
        let cp = *piece; // copy
        Self {
            piece,
            board,
            cmaps: CollisionMaps::new(board, cp.r#type),
            commands: vec![Command::Null],
            pieces: vec![cp],
        }
    }

    // bot API ----------------------------------
    /// undos the last command on the stack. panics if the stack is empty
    pub fn undo(&mut self) {
        self.pop(); // returns an option, doesn't panic
        self.update_piece(*self.pieces.last().expect("Undoing empty stack"));
    }

    /// resets the piece and clears its stack
    /// back to the state it had when it was new
    pub fn reset(&mut self) {
        self.do_command_mut(Command::Backtrack(self.size() - 1));
    }

    /// sets the piece to a new piece
    pub fn update_piece(&mut self, new_piece: Piece) {
        *self.piece = new_piece;
    }

    /// tries to execute the command on the piece
    pub fn do_command(&mut self, command: &Command) -> bool {
        match *command {
            Command::Null => true, // do nothing
            Command::MoveHorizontal(mag) => self
                .cmaps
                .not_obstructed(
                    self.piece.dir,
                    self.piece.row as i8,
                    self.piece.col as i8 + mag,
                )
                .then(|| self.piece.move_unchecked(0, mag))
                .is_some(),
            Command::MoveDrop => {
                let max_down = self
                    .cmaps
                    .max_down(self.piece.dir, self.piece.row, self.piece.col);
                (max_down > 0)
                    .then(|| self.piece.move_unchecked(-max_down, 0))
                    .is_some()
            }
            Command::Rotate(dir) => {
                for [dir_row, dir_col] in self.piece.get_kicks(dir).iter() {
                    if self.cmaps.not_obstructed(
                        (self.piece.dir + dir) % 4,
                        self.piece.row as i8 + dir_row,
                        self.piece.col as i8 + dir_col,
                    ) {
                        let spin = match self.piece.r#type {
                            PIECE_T => SpinType::Full,
                            _ => match self.cmaps.immobile(dir, *dir_row, *dir_col) {
                                true => SpinType::Mini,
                                false => SpinType::None,
                            },
                        };

                        self.piece
                            .rotate_with_kicks_unchecked(dir, *dir_row, *dir_col);
                        self.piece.spin = spin;
                        return true;
                    }
                }
                false
            }
            Command::Backtrack(mag) => {
                *self.piece = self.pieces[self.size() - mag - 1]; // revert piece
                true
            }
        }
    }

    /// does the action specified by a command. adds to the stack
    pub fn do_command_mut(&mut self, command: Command) -> bool {
        if self.do_command(&command) {
            if let &Command::Backtrack(mag) = &command {
                let new_length = self.size() - mag;
                self.commands.truncate(new_length);
                self.pieces.truncate(new_length);
            } else {
                self.commands.push(command);
                self.pieces.push(*self.piece);
            }
            true
        } else {
            false
        }
    }

    /// executes a list of commands onto a piece
    pub fn do_commands(&mut self, commands: &[Command]) -> bool {
        commands.iter().all(|command| self.do_command(command))
    }

    /// does the actions specified by vector of commands
    /// adds to the stack
    pub fn do_commands_mut(&mut self, commands: Vec<Command>) -> bool {
        commands
            .into_iter()
            .all(|command| self.do_command_mut(command))
    }

    /// peeks from the stack without undoing the command
    pub fn peek(&self) -> Option<(Command, Piece)> {
        if self.is_empty() {
            None
        } else {
            Some((*self.commands.last().unwrap(), *self.pieces.last().unwrap()))
        }
    }

    /// pops from the stack without undoing the command
    pub fn pop(&mut self) -> Option<(Command, Piece)> {
        if self.is_empty() {
            None
        } else {
            Some((self.commands.pop().unwrap(), self.pieces.pop().unwrap()))
        }
    }

    /// returns whether the stack is empty
    pub fn is_empty(&self) -> bool {
        // pieces and commands have the same length
        self.commands.is_empty()
    }

    /// returns the size of the stack
    pub fn size(&self) -> usize {
        self.commands.len()
    }
}

#[derive(Debug)]
struct CollisionMaps {
    boards: [[u32; BOARD_WIDTH]; NUM_ROTATE_STATES],
}

impl Display for CollisionMaps {
    /// returns a string representation of the board
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for board in self.boards.iter() {
            for row in (0..VISIBLE_BOARD_HEIGHT).rev() {
                for col in board.iter() {
                    if (col >> row & 1) == 1 {
                        write!(f, "■ ")?
                    } else {
                        write!(f, "□ ")?
                    }
                }
                writeln!(f)?
            }
            writeln!(f)?
        }
        Ok(())
    }
}

impl CollisionMaps {
    fn new(board: &Board, piecetype: u8) -> Self {
        let mut boards = [[0; 10]; 4];
        // for dir in 0..NUM_ROTATE_STATES {
        for (dir, cmap) in boards.iter_mut().enumerate() {
            for [row, col] in PIECE_ROTATIONS[piecetype as usize][dir] {
                for x in 0..10 {
                    let c = board.arr.get((x + col) as usize).copied().unwrap_or(!0);
                    let c = match row < 0 {
                        true => !(!c << -row),
                        false => c >> row,
                    };
                    cmap[x as usize] |= c;
                }
            }
        }
        CollisionMaps { boards }
    }

    fn not_obstructed(&self, dir: u8, row: i8, col: i8) -> bool {
        let v = row < 0
            || self.boards[dir as usize]
                .get(col as usize)
                .map(|&c| c & 1 << row != 0)
                .unwrap_or(true);
        // println!("\n{}\n{}", dir, Board {arr: self.boards[dir as usize]});
        !v
    }

    fn immobile(&self, dir: u8, row: i8, col: i8) -> bool {
        !(self.not_obstructed(dir, row + 1, col)
            | self.not_obstructed(dir, row - 1, col)
            | self.not_obstructed(dir, row, col - 1)
            | self.not_obstructed(dir, row, col - 1))
    }

    fn max_down(&self, dir: u8, row: usize, col: usize) -> i8 {
        row as i8
            - (u32::BITS - (self.boards[dir as usize][col] & !(u32::MAX << row)).leading_zeros())
                as i8
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::piece_constants::*;
    use crate::test_api::functions::*;

    #[test]
    fn test_wall_kick() {
        let mut piece = Piece::new(PIECE_T);
        let board = Board::new();
        let mut controller = Controller::new(&mut piece, &board);

        controller.do_command(&Command::Rotate(1));
        assert_eq!(controller.piece.dir, 1);
        controller.do_command(&Command::MoveHorizontal(-4));
        assert_eq!(controller.piece.col, 0);
        controller.do_command(&Command::Rotate(3));
        assert_eq!(controller.piece.dir, 0);
        assert_location_eq(
            controller.piece.abs_locations(),
            [[21, 0], [21, 1], [21, 2], [22, 1]],
        );

        let mut piece = Piece::new(PIECE_I);
        let mut controller = Controller::new(&mut piece, &board);

        controller.do_command(&Command::Rotate(3));
        assert_eq!(controller.piece.dir, 3);
        controller.do_command(&Command::MoveHorizontal(5));
        assert_eq!(controller.piece.col, 9);
        controller.do_command(&Command::Rotate(3));
        assert_location_eq(
            controller.piece.abs_locations(),
            [[20, 6], [20, 7], [20, 8], [20, 9]],
        );
    }

    #[test]
    fn test_floor_kick() {
        let mut piece = Piece::new(PIECE_L);
        let board = Board::new();
        let mut controller = Controller::new(&mut piece, &board);

        controller.do_command(&Command::MoveDrop);
        assert_eq!(controller.piece.row, 0);
        controller.do_command(&Command::Rotate(3));
        assert_eq!(controller.piece.dir, 3);
        assert_location_eq(
            controller.piece.abs_locations(),
            [[0, 5], [1, 5], [2, 4], [2, 5]],
        );
    }

    #[test]
    fn test_z_spin_1() {
        // z spin 1
        let board = z_spin_board_1();
        let mut piece = Piece::new(PIECE_Z);
        let mut controller = Controller::new(&mut piece, &board);

        controller.do_command(&Command::Rotate(3));
        controller.do_command(&Command::MoveHorizontal(1));
        controller.do_command(&Command::MoveDrop);
        controller.do_command(&Command::Rotate(3));
        assert_location_eq(
            controller.piece.abs_locations(),
            [[0, 4], [0, 5], [1, 3], [1, 4]],
        );
    }

    #[test]
    fn test_z_spin_2() {
        // z spin 2
        let mut piece = Piece::new(PIECE_Z);
        let board = z_spin_board_2();
        let mut controller = Controller::new(&mut piece, &board);

        controller.do_command(&Command::MoveDrop);
        controller.do_command(&Command::Rotate(1));
        controller.do_command(&Command::Rotate(1));
        assert_location_eq(
            controller.piece.abs_locations(),
            [[0, 5], [0, 6], [1, 4], [1, 5]],
        );
    }

    #[test]
    fn test_s_spin_1() {
        // s spin 1
        let mut piece = Piece::new(PIECE_S);
        let board = s_spin_board_1();
        let mut controller = Controller::new(&mut piece, &board);

        controller.do_command(&Command::Rotate(1));
        controller.do_command(&Command::MoveDrop);
        controller.do_command(&Command::Rotate(1));
        assert_location_eq(
            controller.piece.abs_locations(),
            [[0, 4], [0, 5], [1, 5], [1, 6]],
        );
    }

    #[test]
    fn test_s_spin_2() {
        // s spin 2
        let mut piece = Piece::new(PIECE_S);
        let board = s_spin_board_2();
        let mut controller = Controller::new(&mut piece, &board);

        controller.do_command(&Command::Rotate(3));
        controller.do_command(&Command::MoveHorizontal(1));
        controller.do_command(&Command::MoveDrop);
        controller.do_command(&Command::Rotate(3));
        assert_location_eq(
            controller.piece.abs_locations(),
            [[0, 3], [0, 4], [1, 4], [1, 5]],
        );
    }

    #[test]
    fn test_l_spin_1() {
        // l spin 1
        let mut piece = Piece::new(PIECE_L);
        let board = l_spin_board_1();
        let mut controller = Controller::new(&mut piece, &board);

        controller.do_command(&Command::Rotate(3));
        controller.do_command(&Command::MoveHorizontal(1));
        controller.do_command(&Command::MoveDrop);
        controller.do_command(&Command::Rotate(3));
        assert_location_eq(
            controller.piece.abs_locations(),
            [[0, 3], [1, 3], [1, 4], [1, 5]],
        );
    }

    #[test]
    fn test_l_spin_2() {
        // l spin 2
        let mut piece = Piece::new(PIECE_L);
        let board = l_spin_board_2();
        let mut controller = Controller::new(&mut piece, &board);

        controller.do_command(&Command::Rotate(1));
        controller.do_command(&Command::MoveHorizontal(-1));
        controller.do_command(&Command::MoveDrop);
        controller.do_command(&Command::MoveHorizontal(1));
        controller.do_command(&Command::Rotate(1));
        assert_location_eq(
            controller.piece.abs_locations(),
            [[0, 4], [1, 4], [1, 5], [1, 6]],
        );
    }

    #[test]
    fn test_l_spin_3() {
        // l spin 3
        let mut piece = Piece::new(PIECE_L);
        let board = l_spin_board_3();
        let mut controller = Controller::new(&mut piece, &board);

        controller.do_command(&Command::Rotate(1));
        controller.do_command(&Command::MoveDrop);
        controller.do_command(&Command::Rotate(3));
        assert_location_eq(
            controller.piece.abs_locations(),
            [[0, 4], [0, 5], [0, 6], [1, 6]],
        );
    }

    #[test]
    fn test_j_spin_1() {
        // j spin 1
        let mut piece = Piece::new(PIECE_J);
        let board = j_spin_board_1();
        let mut controller = Controller::new(&mut piece, &board);

        controller.do_command(&Command::Rotate(1));
        controller.do_command(&Command::MoveDrop);
        controller.do_command(&Command::Rotate(3));
        assert_location_eq(
            controller.piece.abs_locations(),
            [[0, 4], [0, 5], [0, 6], [1, 4]],
        );
    }

    #[test]
    fn test_j_spin_2() {
        // j spin 2
        let mut piece = Piece::new(PIECE_J);
        let board = j_spin_board_2();
        let mut controller = Controller::new(&mut piece, &board);

        controller.do_command(&Command::Rotate(1));
        controller.do_command(&Command::MoveDrop);
        controller.do_command(&Command::Rotate(1));
        assert_location_eq(
            controller.piece.abs_locations(),
            [[0, 6], [1, 4], [1, 5], [1, 6]],
        );
    }

    #[test]
    fn test_j_spin_3() {
        // j spin 3
        let mut piece = Piece::new(PIECE_J);
        let board = j_spin_board_3();
        let mut controller = Controller::new(&mut piece, &board);

        controller.do_command(&Command::Rotate(3));
        controller.do_command(&Command::MoveHorizontal(1));
        controller.do_command(&Command::MoveDrop);
        controller.do_command(&Command::Rotate(3));
        assert_location_eq(
            controller.piece.abs_locations(),
            [[0, 5], [1, 3], [1, 4], [1, 5]],
        );
    }

    #[test]
    fn test_s_spin_3() {
        // s spin triple 1
        let mut piece = Piece::new(PIECE_S);
        let board = s_spin_board_3();
        let mut controller = Controller::new(&mut piece, &board);

        controller.do_command(&Command::MoveDrop);
        controller.do_command(&Command::Rotate(3));
        assert_location_eq(
            controller.piece.abs_locations(),
            [[0, 4], [1, 3], [1, 4], [2, 3]],
        );
    }

    #[test]
    fn test_s_spin_4() {
        // s spin triple 2
        let mut piece = Piece::new(PIECE_S);
        let board = s_spin_board_4();
        let mut controller = Controller::new(&mut piece, &board);

        controller.do_command(&Command::MoveHorizontal(1));
        controller.do_command(&Command::MoveDrop);
        controller.do_command(&Command::MoveHorizontal(-1));
        controller.do_command(&Command::Rotate(1));
        assert_location_eq(
            controller.piece.abs_locations(),
            [[0, 5], [1, 4], [1, 5], [2, 4]],
        );
    }

    #[test]
    fn test_s_spin_5() {
        // s spin triple 3
        let mut piece = Piece::new(PIECE_S);
        let board = s_spin_board_5();
        let mut controller = Controller::new(&mut piece, &board);

        controller.do_command(&Command::MoveHorizontal(1));
        controller.do_command(&Command::Rotate(3));
        controller.do_command(&Command::MoveDrop);
        controller.do_command(&Command::Rotate(1));
        controller.do_command(&Command::Rotate(1));
        assert_location_eq(
            controller.piece.abs_locations(),
            [[0, 4], [1, 3], [1, 4], [2, 3]],
        );
    }

    #[test]
    fn test_z_spin_3() {
        // z spin triple 1
        let mut piece = Piece::new(PIECE_Z);
        let board = z_spin_board_3();
        let mut controller = Controller::new(&mut piece, &board);

        controller.do_command(&Command::MoveHorizontal(-1));
        controller.do_command(&Command::MoveDrop);
        controller.do_command(&Command::MoveHorizontal(1));
        controller.do_command(&Command::Rotate(3));
        assert_location_eq(
            controller.piece.abs_locations(),
            [[0, 4], [1, 4], [1, 5], [2, 5]],
        );
    }

    #[test]
    fn test_j_spin_4() {
        // j spin 1
        let mut piece = Piece::new(PIECE_J);
        let board = j_spin_board_4();
        let mut controller = Controller::new(&mut piece, &board);

        controller.do_command(&Command::MoveHorizontal(1));
        controller.do_command(&Command::MoveHorizontal(1));
        controller.do_command(&Command::Rotate(3));
        controller.do_command(&Command::MoveDrop);
        controller.do_command(&Command::MoveHorizontal(-1));
        controller.do_command(&Command::Rotate(1));
        assert_location_eq(
            controller.piece.abs_locations(),
            [[0, 3], [0, 4], [0, 5], [1, 3]],
        );
    }

    #[test]
    fn test_j_spin_5() {
        // j spin triple
        let mut piece = Piece::new(PIECE_J);
        let board = j_spin_board_5();
        let mut controller = Controller::new(&mut piece, &board);

        controller.do_command(&Command::MoveHorizontal(-1));
        controller.do_command(&Command::Rotate(1));
        controller.do_command(&Command::MoveDrop);
        controller.do_command(&Command::Rotate(3));
        controller.do_command(&Command::Rotate(3));
        assert_location_eq(
            controller.piece.abs_locations(),
            [[0, 4], [0, 5], [1, 5], [2, 5]],
        );
    }

    #[test]
    fn test_l_spin_4() {
        // l spin 180 (?)
        let mut piece = Piece::new(PIECE_L);
        let board = l_spin_board_4();
        let mut controller = Controller::new(&mut piece, &board);

        controller.do_command(&Command::Rotate(3));
        controller.do_command(&Command::MoveHorizontal(1));
        controller.do_command(&Command::MoveHorizontal(1));
        controller.do_command(&Command::MoveHorizontal(1));
        controller.do_command(&Command::MoveDrop);
        controller.do_command(&Command::Rotate(1));
        controller.do_command(&Command::Rotate(2));
        controller.do_command(&Command::MoveHorizontal(-1));
        controller.do_command(&Command::Rotate(3));
        assert_location_eq(
            controller.piece.abs_locations(),
            [[0, 4], [0, 5], [1, 4], [2, 4]],
        );
    }

    #[test]
    fn test_l_spin_5() {
        // l spin fuckery
        let mut piece = Piece::new(PIECE_L);
        let board = l_spin_board_5();
        let mut controller = Controller::new(&mut piece, &board);

        controller.do_command(&Command::MoveDrop);
        controller.do_command(&Command::MoveHorizontal(-1));
        controller.do_command(&Command::MoveHorizontal(-1));
        controller.do_command(&Command::Rotate(1));
        controller.do_command(&Command::Rotate(3));
        controller.do_command(&Command::MoveHorizontal(1));
        controller.do_command(&Command::Rotate(3));
        controller.do_command(&Command::MoveDrop);
        controller.do_command(&Command::Rotate(2));
        controller.do_command(&Command::Rotate(1));
        controller.do_command(&Command::MoveHorizontal(-1));
        controller.do_command(&Command::Rotate(1));
        controller.do_command(&Command::Rotate(1));
        controller.do_command(&Command::Rotate(1));
        controller.do_command(&Command::Rotate(2));
        controller.do_command(&Command::Rotate(3));
        controller.do_command(&Command::Rotate(3));
        assert_location_eq(
            controller.piece.abs_locations(),
            [[0, 1], [0, 2], [1, 1], [2, 1]],
        );
    }

    #[test]
    fn test_tst() {
        let mut piece = Piece::new(PIECE_T);
        let board = tst_board();
        let mut controller = Controller::new(&mut piece, &board);
        controller.do_command(&Command::MoveHorizontal(-3));
        controller.do_command(&Command::MoveDrop);
        controller.do_command(&Command::MoveHorizontal(1));
        println!("{:?}", *controller.piece);
        controller.do_command(&Command::Rotate(3));
        println!("{:?}", *controller.piece);
        assert_location_eq(
            controller.piece.abs_locations(),
            [[0, 3], [1, 2], [1, 3], [2, 3]],
        );
    }

    #[test]
    fn test_undo() {
        let mut piece = Piece::new(PIECE_T);
        let board = tst_board();
        let mut controller = Controller::new(&mut piece, &board);

        controller.do_command_mut(Command::MoveHorizontal(-3));
        controller.do_command_mut(Command::MoveDrop);
        controller.do_command_mut(Command::MoveHorizontal(1));
        let piece_save = *controller.piece;
        controller.do_command_mut(Command::Rotate(3));
        controller.undo();
        assert_eq!(piece, piece_save);
    }
}
