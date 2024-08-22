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
    use serde::{Deserialize, Serialize};

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

    pub const PIECE_NAME: [char; 7] = ['Z', 'L', 'O', 'S', 'I', 'J', 'T'];

    #[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
    pub enum SpinType {
        None,
        Mini,
        Full,
    }

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

    pub const O_OFFSETS: [[[[i8; 2]; 1]; 3]; NUM_ROTATE_STATES] = [
        [[[1, 0]], [[1, 1]], [[0, 1]]],
        [[[0, 1]], [[-1, 1]], [[-1, 0]]],
        [[[-1, 0]], [[-1, -1]], [[0, -1]]],
        [[[0, -1]], [[1, -1]], [[1, 0]]],
    ];

    pub const T_CORNERS: [[[[i8; 2]; 2]; 2]; 4] = [
        // DIR, FRONT/BACK, COORDS
        [[[1, -1], [1, 1]], [[-1, -1], [-1, 1]]],
        [[[1, 1], [-1, 1]], [[1, -1], [-1, -1]]],
        [[[-1, -1], [-1, 1]], [[1, -1], [1, 1]]],
        [[[-1, -1], [1, -1]], [[1, 1], [-1, 1]]],
    ];
}

pub mod queue_constants {
    pub const MULTIPLIER: usize = 16807;
    pub const MODULUS: usize = 2147483647;
    /// amount of bits occupied per piece
    pub const PIECE_BITS: usize = 3;
}

pub mod versus_constants {
    // Multipliers
    // 0: ????  |  S, TMS
    // 1: 0.25  |  D, TMD, B2B-TMS
    // 2: 0.50  |  T, TS, TMT, B2B-TMD,
    // 3: 0.75  |  B2B-TMT, B2B-TS
    // 4: 1.00  |  Q, TD
    // 5: 1.25  |  B2B-Q, B2B-TD
    // 6: 1.50  |  TT
    // 7: 1.75  |  B2B-TT

    pub const ATTACK0: [u8; 24] = [
        0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3,
    ];
    pub const ATTACK1: [u8; 24] = [
        1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 6, 6, 6, 6,
    ];
    pub const ATTACK2: [u8; 24] = [
        2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13,
    ];
    pub const ATTACK3: [u8; 24] = [
        3, 3, 4, 5, 6, 6, 7, 8, 9, 9, 10, 11, 12, 12, 13, 14, 15, 15, 16, 17, 18, 18, 19, 20,
    ];
    pub const ATTACK4: [u8; 24] = [
        4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27,
    ];
    pub const ATTACK5: [u8; 24] = [
        5, 6, 7, 8, 10, 11, 12, 13, 15, 16, 17, 18, 20, 21, 22, 23, 25, 26, 27, 28, 30, 31, 32, 33,
    ];

    pub const ATTACK6: [u8; 24] = [
        6, 7, 9, 10, 12, 13, 15, 17, 18, 19, 21, 22, 24, 25, 27, 28, 30, 31, 33, 34, 36, 37, 39, 40,
    ];

    pub const ATTACK7: [u8; 24] = [
        7, 8, 10, 12, 14, 15, 17, 19, 21, 22, 24, 26, 28, 29, 31, 33, 35, 36, 38, 40, 42, 43, 45,
        47,
    ];
}
