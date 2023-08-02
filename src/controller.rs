#![allow(dead_code)]

use crate::board::Board;
use crate::command::Command;
use crate::piece::Piece;

#[derive(Debug)]
pub struct Controller<'a> {
    pub piece: &'a mut Piece,
    pub board: &'a Board,
    commands: Vec<Command>,
    pieces: Vec<Piece>,
}

impl<'a> Controller<'a> {
    // constructor ------------------------------
    /// creates a new controller from a board and piece reference
    pub fn new(piece: &'a mut Piece, board: &'a Board) -> Self {
        let cp = *piece;
        Self {
            piece,
            board,
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
        match command {
            &Command::Null => true, // do nothing
            &Command::MoveHorizontal(mag) => {
                Self::can_move_piece(self.board, self.piece, [0, mag])
                    .then(|| self.piece.r#move(0, mag)) // lazy eval
                    .is_some()
            }
            &Command::MoveDrop => {
                let max_down = self.board.piece_max_down(self.piece);
                (max_down > 0)
                    .then(|| self.piece.r#move(-max_down, 0)) // lazy eval
                    .is_some()
            }
            &Command::Rotate(dir) => {
                for [dir_row, dir_col] in self.piece.get_kicks(dir).into_iter() {
                    if Self::can_rotate_kick_piece(self.board, self.piece, dir, [dir_row, dir_col])
                    {
                        self.piece.rotate_with_kicks(dir, dir_row, dir_col);
                        return true;
                    }
                }
                false
            }
            &Command::Backtrack(mag) => {
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
    pub fn do_commands(&mut self, commands: &Vec<Command>) -> bool {
        commands.iter().all(|command| self.do_command(&command))
    }

    /// does the actions specified by vector of commands
    /// adds to the stack
    pub fn do_commands_mut(&mut self, commands: Vec<Command>) -> bool {
        commands
            .into_iter()
            .all(|command| self.do_command_mut(command))
    }

    /// peeks from the stack without undoing the command
    pub fn peek(&mut self) -> Option<(Command, Piece)> {
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

    // static piece API -------------------------
    /// moves a piece if it can be moved, according to [`Game::can_move_piece`]
    pub fn move_piece(board: &Board, piece: &mut Piece, [dir_row, dir_col]: [i8; 2]) {
        if Self::can_move_piece(board, piece, [dir_row, dir_col]) {
            piece.r#move(dir_row, dir_col);
        }
    }

    /// returns whether the piece can be moved by a vector
    /// this just checks if there is a collision between any other board cells
    pub fn can_move_piece(board: &Board, piece: &Piece, [dir_row, dir_col]: [i8; 2]) -> bool {
        let mut cp = *piece;
        cp.r#move(dir_row, dir_col);
        Piece::can_move(piece, dir_row, dir_col) && !board.piece_collision(&cp)
    }

    /// rotates a piece if it can be rotated, according to [`Game::can_rotate_piece`]
    pub fn rotate_piece(board: &Board, piece: &mut Piece, dir: u8) {
        if Self::can_rotate_piece(board, piece, dir) {
            piece.rotate(dir);
        }
    }

    /// returns whether the piece can be rotated in a direction
    /// checks for collisions with any board cells
    /// does not check for any kicks
    /// see the [`Self::can_rotate_kick_piece] method for a rotation check with kicks
    pub fn can_rotate_piece(board: &Board, piece: &Piece, dir: u8) -> bool {
        let mut cp = *piece;
        cp.rotate(dir);
        Piece::can_rotate(piece, dir) && !board.piece_collision(&cp)
    }

    /// returns whether the piece can be rotated in a direction with a kick
    /// checks for collisions with any board cells
    /// checks for kicks
    /// see the [`Self::can_rotate_piece`] method for a rotation check without kicks
    pub fn can_rotate_kick_piece(
        board: &Board,
        piece: &Piece,
        dir: u8,
        [dir_row, dir_col]: [i8; 2],
    ) -> bool {
        let mut cp = *piece;
        cp.rotate_with_kicks(dir, dir_row, dir_col);
        Piece::can_rotate_kick(piece, dir, dir_row, dir_col) && !board.piece_collision(&cp)
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
        controller.do_command(&Command::Rotate(3));
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
