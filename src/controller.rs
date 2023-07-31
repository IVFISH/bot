#![allow(dead_code)]

use crate::board::Board;
use crate::command::Command;
use crate::piece::Piece;

#[derive(Debug, Default)]
pub struct Controller {
    commands: Vec<Command>,
    pieces: Vec<Piece>,
}

impl Controller {
    // constructor ------------------------------
    /// creates a new controller from a board and piece reference
    pub fn new() -> Self {
        Default::default()
    }

    // bot API ----------------------------------
    /// undos the last command on the stack. panics if the stack is empty
    pub fn undo(&mut self, piece: &mut Piece) {
        *piece = self.pop().unwrap().1;
    }

    /// tries to execute the command on the piece
    pub fn do_command(&self, command: &Command, piece: &mut Piece, board: &Board) -> bool {
        match command {
            &Command::Null => true, // do nothing
            &Command::MoveHorizontal(mag) => {
                let [dir_row, dir_col] = [0, mag];
                Self::can_move_piece(board, piece, [dir_row, dir_col])
                    .then(|| piece.r#move(dir_row, dir_col))
                    .is_some()
            }
            &Command::MoveDrop => {
                let [dir_row, dir_col] = [-1, 0];
                let can_drop = Self::can_move_piece(board, piece, [dir_row, dir_col]);
                while Self::can_move_piece(board, piece, [dir_row, dir_col]) {
                    piece.r#move(dir_row, dir_col);
                }
                can_drop
            }
            &Command::Rotate(dir) => {
                for [dir_row, dir_col] in piece.get_kicks(dir).into_iter() {
                    if Self::can_rotate_kick_piece(board, piece, dir, [dir_row, dir_col])
                    {
                        piece.rotate_with_kicks(dir, dir_row, dir_col);
                        return true;
                    }
                }
                false
            }
            &Command::Backtrack(mag) => {
                *piece = self.pieces[self.size() - mag - 1]; // revert piece
                true
            }
        }
    }

    /// does the action specified by a command. adds to the stack
    pub fn do_command_mut(&mut self, command: Command, piece: &mut Piece, board: &Board) -> bool {
        if self.do_command(&command, piece, board) {
            if let &Command::Backtrack(mag) = &command {
                let new_length = self.size() - mag;
                self.commands.truncate(new_length);
                self.pieces.truncate(new_length);
            } else {
                self.commands.push(command);
                self.pieces.push(*piece);
            }
            true
        } else {
            false
        }
    }

    /// executes a list of commands onto a piece
    pub fn do_commands(&self, commands: &Vec<Command>, piece: &mut Piece, board: &Board) -> bool {
        commands
            .iter()
            .all(|command| self.do_command(&command, piece, board))
    }

    /// does the actions specified by vector of commands
    /// adds to the stack
    pub fn do_commands_mut(
        &mut self,
        commands: Vec<Command>,
        piece: &mut Piece,
        board: &Board,
    ) -> bool {
        commands
            .into_iter()
            .all(|command| self.do_command_mut(command, piece, board))
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

    fn assert_location_eq(locations: Option<[[usize; 2]; 4]>, sols: [[usize; 2]; 4]) {
        if let Some(mut locs) = locations {
            locs.sort();
            assert_eq!(locs, sols)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn test_wall_kick() {
        let mut piece = Piece::new(PIECE_T);
        let controller = Controller::new();
        let board = Board::new();

        controller.do_command(&Command::Rotate(1), &mut piece, &board);
        assert_eq!(piece.dir, 1);
        controller.do_command(&Command::MoveHorizontal(-4), &mut piece, &board);
        assert_eq!(piece.col, 0);
        controller.do_command(&Command::Rotate(3), &mut piece, &board);
        assert_eq!(piece.dir, 0);
        assert_location_eq(piece.abs_locations(), [[21, 0], [21, 1], [21, 2], [22, 1]]);

        let mut piece = Piece::new(PIECE_I);
        controller.do_command(&Command::Rotate(3), &mut piece, &board);
        assert_eq!(piece.dir, 3);
        controller.do_command(&Command::MoveHorizontal(5), &mut piece, &board);
        assert_eq!(piece.col, 9);
        controller.do_command(&Command::Rotate(3), &mut piece, &board);
        assert_location_eq(piece.abs_locations(), [[20, 6], [20, 7], [20, 8], [20, 9]]);
    }

    #[test]
    fn test_floor_kick() {
        let mut piece = Piece::new(PIECE_L);
        let controller = Controller::new();
        let board = Board::new();

        controller.do_command(&Command::MoveDrop, &mut piece, &board);
        assert_eq!(piece.row, 0);
        controller.do_command(&Command::Rotate(3), &mut piece, &board);
        assert_eq!(piece.dir, 3);
        assert_location_eq(piece.abs_locations(), [[0, 5], [1, 5], [2, 4], [2, 5]]);
    }
}
