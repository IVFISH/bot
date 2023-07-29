//! parent module for various constants
//! such as for board and piece logic

pub mod board_constants {
    pub const BOARD_WIDTH: usize = 10;
    pub const BOARD_HEIGHT: usize = 40;
    pub const VISIBLE_BOARD_HEIGHT: usize = 23;
    pub const MAX_PLACE_HEIGHT: usize = 20;
}

pub mod piece_constants {
    pub const PIECE_SIZE: usize = 4;
    pub const NUM_ROTATE_STATES: usize = 4;
    pub const SPAWN_ROW: i8 = 21;
    pub const SPAWN_COL: i8 = 4;
    pub const NUM_PIECES: usize = 7;
}

pub mod bot_constants {}
