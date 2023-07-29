use crate::board::Board;
use crate::command::Command;
use crate::piece::Piece;

#[derive(Debug, Default)]
pub struct Controller {
    stack: Vec<Command>,
}

impl Controller {
    // constructor ------------------------------
    /// creates a new controller from a board and piece reference
    pub fn new() -> Self {
        Default::default()
    }

    // bot API ----------------------------------
    /// undos the last command on the stack
    pub fn undo(&mut self, piece: &mut Piece) {
        unimplemented!()
    }

    /// does the action specified by a command
    /// the add parameter specifies if it should add to its stack
    pub fn do_command(
        &mut self,
        command: Command,
        piece: &mut Piece,
        board: &Board,
        add: bool,
    ) -> bool {
        unimplemented!()
    }

    /// does the actions specified by vector of  commands
    /// the add parameter specifies if it should add to its stack
    pub fn do_commands(
        &mut self,
        commands: &Vec<Command>,
        piece: &mut Piece,
        board: &Board,
        add: bool,
    ) -> bool {
        unimplemented!()
    }

    /// pops from the stack without undoing the command
    pub fn pop(&mut self) {
        unimplemented!()
    }

    // static piece API -------------------------
    /// moves a piece if it can be moved, according to [`Game::can_move_piece`]
    pub fn move_piece(board: &Board, piece: &mut Piece, [dir_row, dir_col]: [i8; 2]) {
        if Self::can_move_piece(board, piece, [dir_row, dir_col]) {
            piece.r#move(dir_row, dir_col)
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
            piece.rotate(dir)
        }
    }

    /// returns whether the piece can be rotated in a direction
    /// this just checks if there is a collision between any other board cells
    /// does not check for any kicks, simply moves the piece around the same center
    /// see the [] method for a rotation with kicks
    pub fn can_rotate_piece(board: &Board, piece: &Piece, dir: u8) -> bool {
        let mut cp = *piece;
        cp.rotate(dir);
        Piece::can_rotate(piece, dir) && !board.piece_collision(&cp)
    }
}
