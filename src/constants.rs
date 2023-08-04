//! parent module for various constants
//! such as for board and piece logic

#![allow(dead_code)]

pub mod board_constants {
    pub const BOARD_WIDTH: usize = 10;
    pub const BOARD_HEIGHT: usize = 40;
    pub const VISIBLE_BOARD_HEIGHT: usize = 23;
    pub const MAX_PLACE_HEIGHT: usize = 20;
}

pub mod piece_constants {
    pub const PIECE_SIZE: usize = 4;
    pub const NUM_ROTATE_STATES: usize = 4;
    pub const SPAWN_ROW: usize = 21;
    pub const SPAWN_COL: usize = 4;
    pub const NUM_PIECES: usize = 7;

    pub const PIECE_Z: u8 = 0;
    pub const PIECE_L: u8 = 1;
    pub const PIECE_O: u8 = 2;
    pub const PIECE_S: u8 = 3;
    pub const PIECE_I: u8 = 4;
    pub const PIECE_J: u8 = 5;
    pub const PIECE_T: u8 = 6;

    pub const PIECE_ROTATIONS: [[[[i8; 2]; PIECE_SIZE]; NUM_ROTATE_STATES]; NUM_PIECES] = [
        Z_ROTATIONS,
        L_ROTATIONS,
        O_ROTATIONS,
        S_ROTATIONS,
        I_ROTATIONS,
        J_ROTATIONS,
        T_ROTATIONS,
    ];

    const Z_ROTATIONS: [[[i8; 2]; PIECE_SIZE]; NUM_ROTATE_STATES] = [
        [[1, -1], [1, 0], [0, 0], [0, 1]],
        [[1, 1], [0, 1], [0, 0], [-1, 0]],
        [[-1, 1], [-1, 0], [0, 0], [0, -1]],
        [[-1, -1], [0, -1], [0, 0], [1, 0]],
    ];

    const L_ROTATIONS: [[[i8; 2]; PIECE_SIZE]; NUM_ROTATE_STATES] = [
        [[1, 1], [0, -1], [0, 0], [0, 1]],
        [[-1, 1], [1, 0], [0, 0], [-1, 0]],
        [[-1, -1], [0, 1], [0, 0], [0, -1]],
        [[1, -1], [-1, 0], [0, 0], [1, 0]],
    ];

    const O_ROTATIONS: [[[i8; 2]; PIECE_SIZE]; NUM_ROTATE_STATES] = [
        [[1, 0], [1, 1], [0, 0], [0, 1]],
        [[0, 1], [-1, 1], [0, 0], [-1, 0]],
        [[-1, 0], [-1, -1], [0, 0], [0, -1]],
        [[0, -1], [1, -1], [0, 0], [1, 0]],
    ];

    const S_ROTATIONS: [[[i8; 2]; PIECE_SIZE]; NUM_ROTATE_STATES] = [
        [[1, 0], [1, 1], [0, -1], [0, 0]],
        [[0, 1], [-1, 1], [1, 0], [0, 0]],
        [[-1, 0], [-1, -1], [0, 1], [0, 0]],
        [[0, -1], [1, -1], [-1, 0], [0, 0]],
    ];

    const I_ROTATIONS: [[[i8; 2]; PIECE_SIZE]; NUM_ROTATE_STATES] = [
        [[0, -1], [0, 0], [0, 1], [0, 2]],
        [[1, 0], [0, 0], [-1, 0], [-2, 0]],
        [[0, 1], [0, 0], [0, -1], [0, -2]],
        [[-1, 0], [0, 0], [1, 0], [2, 0]],
    ];

    const J_ROTATIONS: [[[i8; 2]; PIECE_SIZE]; NUM_ROTATE_STATES] = [
        [[1, -1], [0, -1], [0, 0], [0, 1]],
        [[1, 1], [1, 0], [0, 0], [-1, 0]],
        [[-1, 1], [0, 1], [0, 0], [0, -1]],
        [[-1, -1], [-1, 0], [0, 0], [1, 0]],
    ];

    const T_ROTATIONS: [[[i8; 2]; PIECE_SIZE]; NUM_ROTATE_STATES] = [
        [[1, 0], [0, -1], [0, 0], [0, 1]],
        [[0, 1], [1, 0], [0, 0], [-1, 0]],
        [[-1, 0], [0, 1], [0, 0], [0, -1]],
        [[0, -1], [-1, 0], [0, 0], [1, 0]],
    ];

    pub const THREE_OFFSETS: [[[[i8; 2]; 5]; 2]; NUM_ROTATE_STATES] = [
        [
            [[0, 0], [0, -1], [1, -1], [-2, 0], [-2, -1]],
            [[0, 0], [0, 1], [1, 1], [-2, 0], [-2, 1]],
        ],
        [
            [[0, 0], [0, 1], [-1, 1], [2, 0], [2, 1]],
            [[0, 0], [0, 1], [-1, 1], [2, 0], [2, 1]],
        ],
        [
            [[0, 0], [0, 1], [1, 1], [-2, 0], [-2, 1]],
            [[0, 0], [0, -1], [1, -1], [-2, 0], [-2, -1]],
        ],
        [
            [[0, 0], [0, -1], [-1, -1], [2, 0], [2, -1]],
            [[0, 0], [0, -1], [-1, -1], [2, 0], [2, -1]],
        ],
    ];

    pub const FIVE_OFFSETS: [[[[i8; 2]; 5]; 2]; NUM_ROTATE_STATES] = [
        [
            [[0, 1], [0, 2], [0, -1], [-1, -1], [2, 2]],
            [[-1, 0], [-1, -1], [-1, 2], [-2, 2], [2, -1]],
        ],
        [
            [[-1, 0], [-1, -1], [-1, 2], [1, -1], [-2, 2]],
            [[0, -1], [0, -2], [0, 1], [-2, -2], [1, 1]],
        ],
        [
            [[0, -1], [0, 1], [0, -2], [1, 1], [-2, -2]],
            [[1, 0], [1, -2], [1, 1], [2, -2], [-1, 1]],
        ],
        [
            [[1, 0], [1, 1], [1, -2], [-1, 1], [2, -2]],
            [[0, 1], [0, 2], [0, -1], [2, 2], [-1, -1]],
        ],
    ];

    pub const THREE_180_OFFSETS: [[[i8; 2]; 6]; NUM_ROTATE_STATES] = [
        [[0, 0], [1, 0], [1, 1], [1, -1], [0, 1], [0, -1]],
        [[0, 0], [0, 1], [2, 1], [1, 1], [2, 0], [-1, 0]],
        [[0, 0], [-1, 0], [-1, -1], [-1, 1], [0, -1], [0, 1]],
        [[0, 0], [0, -1], [2, -1], [1, -1], [2, 0], [-1, 0]],
    ];

    pub const FIVE_180_OFFSETS: [[[i8; 2]; 2]; NUM_ROTATE_STATES] = [
        [[-1, 1], [0, 1]],
        [[-1, -1], [-1, 0]],
        [[1, -1], [0, -1]],
        [[1, 1], [1, 0]],
    ];

    pub const O_OFFSETS: [[[i8; 2]; 3]; NUM_ROTATE_STATES] = [
        [[1, 0], [1, 1], [0, 1]],
        [[0, 1], [-1, 1], [-1, 0]],
        [[-1, 0], [-1, -1], [0, -1]],
        [[0, -1], [1, -1], [1, 0]],
    ];
}

pub mod queue_constants {
    pub const MULTIPLIER: usize = 16807;
    pub const MODULUS: usize = 2147483647;
    /// amount of bits occupied per piece
    pub const PIECE_BITS: usize = 3;
}
