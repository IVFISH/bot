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
        self.pop();
        *piece = *self.pieces.last().unwrap();
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
                    if Self::can_rotate_kick_piece(board, piece, dir, [dir_row, dir_col]) {
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
pub mod tests {
    use super::*;
    use crate::constants::piece_constants::*;

    fn assert_location_eq(locations: Option<[[usize; 2]; 4]>, mut sols: [[usize; 2]; 4]) {
        if let Some(mut locs) = locations {
            locs.sort();
            sols.sort();
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

    #[test]
    fn test_z_spin_1() {
        // z spin 1
        let board = z_spin_board_1();
        let controller = Controller::new();
        let mut piece = Piece::new(PIECE_Z);

        controller.do_command(&Command::Rotate(3), &mut piece, &board);
        controller.do_command(&Command::MoveHorizontal(1), &mut piece, &board);
        controller.do_command(&Command::MoveDrop, &mut piece, &board);
        controller.do_command(&Command::Rotate(3), &mut piece, &board);
        assert_location_eq(piece.abs_locations(), [[0, 4], [0, 5], [1, 3], [1, 4]]);
    }

    #[test]
    fn test_z_spin_2() {
        // z spin 2
        let mut piece = Piece::new(PIECE_Z);
        let board = z_spin_board_2();
        let controller = Controller::new();

        controller.do_command(&Command::MoveDrop, &mut piece, &board);
        controller.do_command(&Command::Rotate(1), &mut piece, &board);
        controller.do_command(&Command::Rotate(1), &mut piece, &board);
        assert_location_eq(piece.abs_locations(), [[0, 5], [0, 6], [1, 4], [1, 5]]);
    }

    #[test]
    fn test_s_spin_1() {
        // s spin 1
        let mut piece = Piece::new(PIECE_S);
        let board = s_spin_board_1();
        let controller = Controller::new();

        controller.do_command(&Command::Rotate(1), &mut piece, &board);
        controller.do_command(&Command::MoveDrop, &mut piece, &board);
        controller.do_command(&Command::Rotate(1), &mut piece, &board);
        assert_location_eq(piece.abs_locations(), [[0, 4], [0, 5], [1, 5], [1, 6]]);
    }

    #[test]
    fn test_s_spin_2() {
        // s spin 2
        let mut piece = Piece::new(PIECE_S);
        let board = s_spin_board_2();
        let controller = Controller::new();

        controller.do_command(&Command::Rotate(3), &mut piece, &board);
        controller.do_command(&Command::MoveHorizontal(1), &mut piece, &board);
        controller.do_command(&Command::MoveDrop, &mut piece, &board);
        controller.do_command(&Command::Rotate(3), &mut piece, &board);
        assert_location_eq(piece.abs_locations(), [[0, 3], [0, 4], [1, 4], [1, 5]]);
    }

    #[test]
    fn test_l_spin_1() {
        // l spin 1
        let mut piece = Piece::new(PIECE_L);
        let board = l_spin_board_1();
        let controller = Controller::new();

        controller.do_command(&Command::Rotate(3), &mut piece, &board);
        controller.do_command(&Command::MoveHorizontal(1), &mut piece, &board);
        controller.do_command(&Command::MoveDrop, &mut piece, &board);
        controller.do_command(&Command::Rotate(3), &mut piece, &board);
        assert_location_eq(piece.abs_locations(), [[0, 3], [1, 3], [1, 4], [1, 5]]);
    }

    #[test]
    fn test_l_spin_2() {
        // l spin 2
        let mut piece = Piece::new(PIECE_L);
        let board = l_spin_board_2();
        let controller = Controller::new();

        controller.do_command(&Command::Rotate(1), &mut piece, &board);
        controller.do_command(&Command::MoveHorizontal(-1), &mut piece, &board);
        controller.do_command(&Command::MoveDrop, &mut piece, &board);
        controller.do_command(&Command::MoveHorizontal(1), &mut piece, &board);
        controller.do_command(&Command::Rotate(1), &mut piece, &board);
        assert_location_eq(piece.abs_locations(), [[0, 4], [1, 4], [1, 5], [1, 6]]);
    }

    #[test]
    fn test_l_spin_3() {
        // l spin 3
        let mut piece = Piece::new(PIECE_L);
        let board = l_spin_board_3();
        let controller = Controller::new();

        controller.do_command(&Command::Rotate(1), &mut piece, &board);
        controller.do_command(&Command::MoveDrop, &mut piece, &board);
        controller.do_command(&Command::Rotate(3), &mut piece, &board);
        assert_location_eq(piece.abs_locations(), [[0, 4], [0, 5], [0, 6], [1, 6]]);
    }

    #[test]
    fn test_j_spin_1() {
        // j spin 1
        let mut piece = Piece::new(PIECE_J);
        let board = j_spin_board_1();
        let controller = Controller::new();

        controller.do_command(&Command::Rotate(1), &mut piece, &board);
        controller.do_command(&Command::MoveDrop, &mut piece, &board);
        controller.do_command(&Command::Rotate(3), &mut piece, &board);
        assert_location_eq(piece.abs_locations(), [[0, 4], [0, 5], [0, 6], [1, 4]]);
    }

    #[test]
    fn test_j_spin_2() {
        // j spin 2
        let mut piece = Piece::new(PIECE_J);
        let board = j_spin_board_2();
        let controller = Controller::new();
        controller.do_command(&Command::Rotate(1), &mut piece, &board);
        controller.do_command(&Command::MoveDrop, &mut piece, &board);
        controller.do_command(&Command::Rotate(1), &mut piece, &board);
        assert_location_eq(piece.abs_locations(), [[0, 6], [1, 6], [1, 5], [1, 4]]);
    }

    #[test]
    fn test_j_spin_3() {
        // j spin 3
        let mut piece = Piece::new(PIECE_J);
        let board = j_spin_board_3();
        let controller = Controller::new();

        controller.do_command(&Command::Rotate(3), &mut piece, &board);
        controller.do_command(&Command::MoveHorizontal(1), &mut piece, &board);
        controller.do_command(&Command::MoveDrop, &mut piece, &board);
        controller.do_command(&Command::Rotate(3), &mut piece, &board);
        assert_location_eq(piece.abs_locations(), [[0, 5], [1, 3], [1, 4], [1, 5]]);
    }

    #[test]
    fn test_s_spin_3() {
        // s spin triple 1
        let mut piece = Piece::new(PIECE_S);
        let board = s_spin_board_3();
        let controller = Controller::new();

        controller.do_command(&Command::MoveDrop, &mut piece, &board);
        controller.do_command(&Command::Rotate(3), &mut piece, &board);
        assert_location_eq(piece.abs_locations(), [[0, 4], [1, 3], [1, 4], [2, 3]]);
    }

    #[test]
    fn test_s_spin_4() {
        // s spin triple 2
        let mut piece = Piece::new(PIECE_S);
        let board = s_spin_board_4();
        let controller = Controller::new();

        controller.do_command(&Command::MoveHorizontal(1), &mut piece, &board);
        controller.do_command(&Command::MoveDrop, &mut piece, &board);
        controller.do_command(&Command::MoveHorizontal(-1), &mut piece, &board);
        controller.do_command(&Command::Rotate(1), &mut piece, &board);
        assert_location_eq(piece.abs_locations(), [[0, 5], [1, 4], [1, 5], [2, 4]]);
    }

    #[test]
    fn test_s_spin_5() {
        // s spin triple 3
        let mut piece = Piece::new(PIECE_S);
        let board = s_spin_board_5();
        let controller = Controller::new();

        controller.do_command(&Command::MoveHorizontal(1), &mut piece, &board);
        controller.do_command(&Command::Rotate(3), &mut piece, &board);
        controller.do_command(&Command::MoveDrop, &mut piece, &board);
        controller.do_command(&Command::Rotate(1), &mut piece, &board);
        controller.do_command(&Command::Rotate(1), &mut piece, &board);
        assert_location_eq(piece.abs_locations(), [[0, 4], [1, 3], [1, 4], [2, 3]]);
    }

    #[test]
    fn test_z_spin_3() {
        // z spin triple 1
        let mut piece = Piece::new(PIECE_Z);
        let board = z_spin_board_3();
        let controller = Controller::new();

        controller.do_command(&Command::MoveHorizontal(-1), &mut piece, &board);
        controller.do_command(&Command::MoveDrop, &mut piece, &board);
        controller.do_command(&Command::MoveHorizontal(1), &mut piece, &board);
        controller.do_command(&Command::Rotate(3), &mut piece, &board);
        assert_location_eq(piece.abs_locations(), [[0, 4], [1, 4], [1, 5], [2, 5]]);
    }

    #[test]
    fn test_j_spin_4() {
        // j spin 1
        let mut piece = Piece::new(PIECE_J);
        let board = j_spin_board_4();
        let controller = Controller::new();

        controller.do_command(&Command::MoveHorizontal(1), &mut piece, &board);
        controller.do_command(&Command::MoveHorizontal(1), &mut piece, &board);
        controller.do_command(&Command::Rotate(3), &mut piece, &board);
        controller.do_command(&Command::MoveDrop, &mut piece, &board);
        controller.do_command(&Command::MoveHorizontal(-1), &mut piece, &board);
        controller.do_command(&Command::Rotate(1), &mut piece, &board);
        assert_location_eq(piece.abs_locations(), [[0, 3], [0, 4], [0, 5], [1, 3]]);
    }

    #[test]
    fn test_j_spin_5() {
        // j spin triple
        let mut piece = Piece::new(PIECE_J);
        let board = j_spin_board_5();
        let controller = Controller::new();

        controller.do_command(&Command::MoveHorizontal(-1), &mut piece, &board);
        controller.do_command(&Command::Rotate(1), &mut piece, &board);
        controller.do_command(&Command::MoveDrop, &mut piece, &board);
        controller.do_command(&Command::Rotate(3), &mut piece, &board);
        controller.do_command(&Command::Rotate(3), &mut piece, &board);
        assert_location_eq(piece.abs_locations(), [[0, 4], [0, 5], [1, 5], [2, 5]]);
    }

    #[test]
    fn test_l_spin_4() {
        // l spin 180 (?)
        let mut piece = Piece::new(PIECE_L);
        let board = l_spin_board_4();
        let controller = Controller::new();

        controller.do_command(&Command::Rotate(3), &mut piece, &board);
        controller.do_command(&Command::MoveHorizontal(1), &mut piece, &board);
        controller.do_command(&Command::MoveHorizontal(1), &mut piece, &board);
        controller.do_command(&Command::MoveHorizontal(1), &mut piece, &board);
        controller.do_command(&Command::MoveDrop, &mut piece, &board);
        controller.do_command(&Command::Rotate(1), &mut piece, &board);
        controller.do_command(&Command::Rotate(2), &mut piece, &board);
        controller.do_command(&Command::MoveHorizontal(-1), &mut piece, &board);
        controller.do_command(&Command::Rotate(3), &mut piece, &board);
        assert_location_eq(piece.abs_locations(), [[0, 4], [0, 5], [1, 4], [2, 4]]);
    }

    #[test]
    fn test_l_spin_5() {
        // l spin fuckery
        let mut piece = Piece::new(PIECE_L);
        let board = l_spin_board_5();
        let controller = Controller::new();

        controller.do_command(&Command::MoveDrop, &mut piece, &board);
        controller.do_command(&Command::MoveHorizontal(-1), &mut piece, &board);
        controller.do_command(&Command::MoveHorizontal(-1), &mut piece, &board);
        controller.do_command(&Command::Rotate(1), &mut piece, &board);
        controller.do_command(&Command::Rotate(3), &mut piece, &board);
        controller.do_command(&Command::MoveHorizontal(1), &mut piece, &board);
        controller.do_command(&Command::Rotate(3), &mut piece, &board);
        controller.do_command(&Command::MoveDrop, &mut piece, &board);
        controller.do_command(&Command::Rotate(2), &mut piece, &board);
        controller.do_command(&Command::Rotate(1), &mut piece, &board);
        controller.do_command(&Command::MoveHorizontal(-1), &mut piece, &board);
        controller.do_command(&Command::Rotate(1), &mut piece, &board);
        controller.do_command(&Command::Rotate(1), &mut piece, &board);
        controller.do_command(&Command::Rotate(1), &mut piece, &board);
        controller.do_command(&Command::Rotate(2), &mut piece, &board);
        controller.do_command(&Command::Rotate(3), &mut piece, &board);
        controller.do_command(&Command::Rotate(3), &mut piece, &board);
        assert_location_eq(piece.abs_locations(), [[0, 1], [0, 2], [1, 1], [2, 1]]);
    }

    #[test]
    fn test_tst() {
        let mut piece = Piece::new(PIECE_T);
        let board = tst_board();
        let controller = Controller::new();
        controller.do_command(&Command::MoveHorizontal(-3), &mut piece, &board);
        controller.do_command(&Command::MoveDrop, &mut piece, &board);
        controller.do_command(&Command::MoveHorizontal(1), &mut piece, &board);
        controller.do_command(&Command::Rotate(3), &mut piece, &board);
        assert_location_eq(piece.abs_locations(), [[0, 3], [1, 2], [1, 3], [2, 3]]);
    }

    #[test]
    fn test_undo() {
        let mut piece = Piece::new(PIECE_T);
        let board = tst_board();
        let mut controller = Controller::new();
        controller.do_command_mut(Command::MoveHorizontal(-3), &mut piece, &board);
        controller.do_command_mut(Command::MoveDrop, &mut piece, &board);
        controller.do_command_mut(Command::MoveHorizontal(1), &mut piece, &board);
        let piece_save = piece.clone();
        controller.do_command_mut(Command::Rotate(3), &mut piece, &board);
        controller.undo(&mut piece);
        assert_eq!(piece, piece_save);
    }

    fn add_list(board: &mut Board, list: Vec<[usize; 2]>) {
        for [r, c] in list.into_iter() {
            board.set(r, c, 1);
        }
    }

    pub fn z_spin_board_1() -> Board {
        let mut board = Board::new();
        add_list(
            &mut board,
            vec![
                [0, 0],
                [1, 0],
                [0, 1],
                [1, 1],
                [0, 2],
                [1, 2],
                [0, 3],
                [1, 5],
                [0, 6],
                [1, 6],
                [0, 7],
                [1, 7],
                [0, 8],
                [1, 8],
                [0, 9],
                [1, 9],
            ],
        );
        board
    }

    fn z_spin_board_2() -> Board {
        let mut board = Board::new();
        add_list(
            &mut board,
            vec![
                [0, 0],
                [1, 0],
                [2, 0],
                [0, 1],
                [1, 1],
                [2, 1],
                [0, 2],
                [1, 2],
                [2, 2],
                [0, 3],
                [1, 3],
                [2, 3],
                [0, 4],
                [1, 6],
                [2, 6],
                [0, 7],
                [1, 7],
                [2, 7],
                [0, 8],
                [1, 8],
                [2, 8],
                [0, 9],
                [1, 9],
                [2, 9],
            ],
        );
        board
    }

    fn s_spin_board_1() -> Board {
        let mut board = Board::new();

        add_list(
            &mut board,
            vec![
                [0, 0],
                [1, 0],
                [0, 1],
                [1, 1],
                [0, 2],
                [1, 2],
                [0, 3],
                [1, 3],
                [1, 4],
                [0, 6],
                [0, 7],
                [1, 7],
                [0, 8],
                [1, 8],
                [0, 9],
                [1, 9],
            ],
        );
        board
    }

    fn s_spin_board_2() -> Board {
        let mut board = Board::new();
        add_list(
            &mut board,
            vec![
                [0, 0],
                [1, 0],
                [2, 0],
                [0, 1],
                [1, 1],
                [2, 1],
                [0, 2],
                [1, 2],
                [2, 2],
                [1, 3],
                [2, 3],
                [0, 5],
                [0, 6],
                [1, 6],
                [2, 6],
                [0, 7],
                [1, 7],
                [2, 7],
                [0, 8],
                [1, 8],
                [2, 8],
                [0, 9],
                [1, 9],
                [2, 9],
            ],
        );
        board
    }

    fn l_spin_board_1() -> Board {
        let mut board = Board::new();
        add_list(
            &mut board,
            vec![
                [0, 0],
                [1, 0],
                [2, 0],
                [0, 1],
                [1, 1],
                [2, 1],
                [0, 2],
                [1, 2],
                [2, 2],
                [2, 3],
                [0, 4],
                [0, 5],
                [0, 6],
                [1, 6],
                [2, 6],
                [0, 7],
                [1, 7],
                [2, 7],
                [0, 8],
                [1, 8],
                [2, 8],
                [0, 9],
                [1, 9],
                [2, 9],
            ],
        );
        board
    }

    fn l_spin_board_2() -> Board {
        let mut board = Board::new();
        add_list(
            &mut board,
            vec![
                [0, 0],
                [0, 1],
                [0, 2],
                [0, 3],
                [0, 5],
                [2, 5],
                [0, 6],
                [2, 6],
                [0, 7],
                [1, 7],
                [2, 7],
                [0, 8],
                [1, 8],
                [2, 8],
                [0, 9],
                [1, 9],
                [2, 9],
            ],
        );
        board
    }

    fn l_spin_board_3() -> Board {
        let mut board = Board::new();
        add_list(
            &mut board,
            vec![
                [0, 0],
                [1, 0],
                [2, 0],
                [0, 1],
                [1, 1],
                [2, 1],
                [0, 2],
                [1, 2],
                [2, 2],
                [0, 3],
                [1, 3],
                [2, 3],
                [2, 6],
                [0, 7],
                [1, 7],
                [2, 7],
                [0, 8],
                [1, 8],
                [2, 8],
                [0, 9],
            ],
        );
        board
    }

    fn j_spin_board_1() -> Board {
        let mut board = Board::new();
        add_list(
            &mut board,
            vec![
                [0, 0],
                [1, 0],
                [0, 1],
                [1, 1],
                [0, 2],
                [1, 2],
                [0, 3],
                [1, 3],
                [1, 5],
                [1, 6],
                [0, 7],
                [1, 7],
                [0, 8],
                [1, 8],
                [0, 9],
                [1, 9],
            ],
        );
        board
    }

    fn j_spin_board_2() -> Board {
        let mut board = Board::new();
        add_list(
            &mut board,
            vec![
                [0, 0],
                [1, 0],
                [2, 0],
                [0, 1],
                [1, 1],
                [2, 1],
                [0, 2],
                [1, 2],
                [2, 2],
                [0, 3],
                [1, 3],
                [2, 3],
                [0, 4],
                [0, 5],
                [2, 5],
                [2, 6],
                [0, 7],
                [1, 7],
                [2, 7],
                [0, 8],
                [1, 8],
                [2, 8],
                [0, 9],
                [1, 9],
            ],
        );
        board
    }

    fn j_spin_board_3() -> Board {
        let mut board = Board::new();
        add_list(
            &mut board,
            vec![
                [0, 0],
                [1, 0],
                [2, 0],
                [0, 1],
                [1, 1],
                [2, 1],
                [0, 2],
                [1, 2],
                [2, 2],
                [0, 3],
                [2, 3],
                [0, 4],
                [0, 6],
                [1, 6],
                [2, 6],
                [0, 7],
                [1, 7],
                [2, 7],
                [0, 8],
                [1, 8],
                [2, 8],
                [0, 9],
                [2, 9],
                [1, 9],
            ],
        );
        board
    }

    fn s_spin_board_3() -> Board {
        let mut board = Board::new();
        add_list(
            &mut board,
            vec![
                [0, 0],
                [1, 0],
                [2, 0],
                [0, 1],
                [1, 1],
                [2, 1],
                [0, 2],
                [1, 2],
                [2, 2],
                [0, 3],
                [2, 4],
                [0, 5],
                [1, 5],
                [2, 5],
                [3, 5],
                [0, 6],
                [1, 6],
                [2, 6],
                [3, 6],
                [0, 7],
                [1, 7],
                [2, 7],
                [3, 7],
                [0, 8],
                [1, 8],
                [2, 8],
                [3, 8],
                [0, 9],
                [1, 9],
                [2, 9],
                [3, 9],
            ],
        );
        board
    }

    fn s_spin_board_4() -> Board {
        let mut board = Board::new();
        add_list(
            &mut board,
            vec![
                [0, 0],
                [1, 0],
                [2, 0],
                [3, 0],
                [0, 1],
                [1, 1],
                [2, 1],
                [3, 1],
                [0, 2],
                [1, 2],
                [2, 2],
                [3, 2],
                [4, 2],
                [0, 3],
                [1, 3],
                [2, 3],
                [4, 3],
                [0, 4],
                [2, 5],
                [0, 6],
                [1, 6],
                [2, 6],
                [0, 7],
                [1, 7],
                [2, 7],
                [0, 8],
                [1, 8],
                [2, 8],
                [0, 9],
                [1, 9],
                [2, 9],
            ],
        );
        board
    }

    fn s_spin_board_5() -> Board {
        let mut board = Board::new();
        add_list(
            &mut board,
            vec![
                [0, 0],
                [1, 0],
                [2, 0],
                [3, 0],
                [4, 0],
                [0, 1],
                [1, 1],
                [2, 1],
                [3, 1],
                [4, 1],
                [0, 2],
                [1, 2],
                [2, 2],
                [3, 2],
                [4, 2],
                [0, 3],
                [4, 3],
                [2, 4],
                [0, 5],
                [1, 5],
                [2, 5],
                [0, 6],
                [1, 6],
                [2, 6],
                [3, 6],
                [4, 6],
                [5, 6],
                [0, 7],
                [1, 7],
                [2, 7],
                [3, 7],
                [4, 7],
                [5, 7],
                [0, 8],
                [1, 8],
                [2, 8],
                [3, 8],
                [4, 8],
                [5, 8],
                [0, 9],
                [1, 9],
                [2, 9],
                [3, 9],
                [4, 9],
                [5, 9],
            ],
        );
        board
    }

    fn z_spin_board_3() -> Board {
        let mut board = Board::new();
        add_list(
            &mut board,
            vec![
                [0, 0],
                [1, 0],
                [2, 0],
                [0, 1],
                [1, 1],
                [2, 1],
                [0, 2],
                [1, 2],
                [2, 2],
                [0, 3],
                [1, 3],
                [2, 3],
                [2, 4],
                [0, 5],
                [5, 5],
                [0, 6],
                [1, 6],
                [2, 6],
                [3, 6],
                [4, 6],
                [0, 7],
                [1, 7],
                [2, 7],
                [3, 7],
                [4, 7],
                [0, 8],
                [1, 8],
                [2, 8],
                [3, 8],
                [4, 8],
                [0, 9],
                [1, 9],
                [2, 9],
                [3, 9],
                [4, 9],
            ],
        );
        board
    }

    fn j_spin_board_4() -> Board {
        let mut board = Board::new();
        add_list(
            &mut board,
            vec![
                [0, 0],
                [1, 0],
                [2, 0],
                [0, 1],
                [1, 1],
                [2, 1],
                [0, 2],
                [1, 2],
                [2, 2],
                [2, 3],
                [1, 4],
                [2, 4],
            ],
        );
        board
    }

    fn j_spin_board_5() -> Board {
        let mut board = Board::new();
        add_list(
            &mut board,
            vec![
                [0, 0],
                [1, 0],
                [2, 0],
                [3, 0],
                [4, 0],
                [5, 0],
                [0, 1],
                [1, 1],
                [2, 1],
                [3, 1],
                [4, 1],
                [5, 1],
                [0, 2],
                [1, 2],
                [2, 2],
                [3, 2],
                [4, 2],
                [5, 2],
                [0, 3],
                [1, 3],
                [2, 3],
                [1, 4],
                [2, 4],
                [4, 5],
                [0, 6],
                [1, 6],
                [2, 6],
                [3, 6],
                [4, 6],
                [0, 7],
                [1, 7],
                [2, 7],
                [3, 7],
                [4, 7],
                [0, 8],
                [1, 8],
                [2, 8],
                [3, 8],
                [4, 8],
                [0, 9],
                [1, 9],
                [2, 9],
                [3, 9],
                [4, 9],
            ],
        );
        board
    }

    fn l_spin_board_4() -> Board {
        let mut board = Board::new();
        add_list(
            &mut board,
            vec![
                [0, 0],
                [1, 0],
                [2, 0],
                [3, 0],
                [4, 0],
                [5, 0],
                [6, 0],
                [0, 1],
                [1, 1],
                [2, 1],
                [3, 1],
                [4, 1],
                [5, 1],
                [6, 1],
                [0, 2],
                [1, 2],
                [2, 2],
                [3, 2],
                [4, 2],
                [5, 2],
                [6, 2],
                [0, 3],
                [1, 3],
                [2, 3],
                [3, 3],
                [4, 3],
                [5, 3],
                [6, 3],
                [4, 4],
                [1, 5],
                [4, 5],
                [0, 6],
                [1, 6],
                [2, 6],
                [4, 6],
                [0, 7],
                [1, 7],
                [2, 7],
                [0, 8],
                [1, 8],
                [2, 8],
                [3, 8],
                [4, 8],
                [5, 8],
                [0, 9],
                [1, 9],
                [2, 9],
                [3, 9],
                [4, 9],
                [5, 9],
            ],
        );
        board
    }

    pub fn l_spin_board_5() -> Board {
        let mut board = Board::new();
        add_list(
            &mut board,
            vec![
                [0, 0],
                [1, 0],
                [2, 0],
                [3, 0],
                [4, 0],
                [5, 0],
                [6, 0],
                [7, 0],
                [8, 0],
                [9, 0],
                [10, 0],
                [11, 0],
                [12, 0],
                [13, 0],
                [14, 0],
                [4, 1],
                [5, 1],
                [6, 1],
                [7, 1],
                [8, 1],
                [9, 1],
                [10, 1],
                [11, 1],
                [12, 1],
                [14, 1],
                [1, 2],
                [2, 2],
                [5, 2],
                [6, 2],
                [7, 2],
                [8, 2],
                [9, 2],
                [0, 3],
                [1, 3],
                [6, 3],
                [7, 3],
                [8, 3],
                [9, 3],
                [11, 3],
                [12, 3],
                [0, 4],
                [1, 4],
                [3, 4],
                [4, 4],
                [6, 4],
                [9, 4],
                [12, 4],
                [0, 5],
                [1, 5],
                [2, 5],
                [3, 5],
                [4, 5],
                [12, 5],
                [0, 6],
                [1, 6],
                [2, 6],
                [3, 6],
                [4, 6],
                [5, 6],
                [6, 6],
                [7, 6],
                [9, 6],
                [10, 6],
                [11, 6],
                [12, 6],
                [0, 7],
                [1, 7],
                [2, 7],
                [3, 7],
                [4, 7],
                [5, 7],
                [6, 7],
                [7, 7],
                [9, 7],
                [10, 7],
                [11, 7],
                [12, 7],
                [0, 8],
                [1, 8],
                [2, 8],
                [3, 8],
                [4, 8],
                [5, 8],
                [6, 8],
                [7, 8],
                [8, 8],
                [9, 8],
                [10, 8],
                [11, 8],
                [12, 8],
                [0, 9],
                [1, 9],
                [2, 9],
                [3, 9],
                [4, 9],
                [5, 9],
                [6, 9],
                [7, 9],
                [8, 9],
                [9, 9],
                [10, 9],
                [11, 9],
                [12, 9],
            ],
        );
        board
    }

    pub fn tst_board() -> Board {
        let mut board = Board::new();
        add_list(
            &mut board,
            vec![
                [1, 0],
                [0, 0],
                [0, 1],
                [0, 2],
                [2, 1],
                [2, 0],
                [1, 1],
                [2, 2],
                [0, 4],
                [2, 4],
                [1, 4],
                [3, 4],
                [4, 4],
                [4, 3],
                [4, 5],
                [3, 5],
                [1, 5],
                [1, 5],
                [2, 5],
                [0, 5],
                [2, 6],
                [1, 6],
                [0, 6],
                [2, 7],
                [1, 7],
                [0, 7],
                [2, 8],
                [1, 8],
                [0, 8],
                [2, 9],
                [1, 9],
                [0, 9],
            ],
        );
        board
    }
}
