pub mod types {
    use crate::point_vector::Point;
    use super::piece_constants::*;

    pub type PieceType = usize;
    pub type RotationState = usize;
    pub type RotationDirection = usize;
    pub type RotationLocations = [[Point; PIECE_SIZE]; NUM_ROTATE_STATES];
}

pub mod board_constants {
    pub const BOARD_WIDTH: usize = 10;
    pub const BOARD_HEIGHT: usize = 40;
}

pub mod piece_constants {
    use crate::point_vector::PointVector;

    pub const PIECE_SIZE: usize = 4;
    pub const NUM_ROTATE_STATES: usize = 4;
    pub const SPAWN_ROW: usize = 21;
    pub const SPAWN_COL: usize = 4;
    pub const NUM_PIECES: usize = 7;


    pub const RELATIVE_CORNERS: [([PointVector; 2], [PointVector; 2]); 4] = [
        (
            [PointVector(1, -1), PointVector(1, 1)],
            [PointVector(-1, -1), PointVector(-1, 1)],
        ),
        (
            [PointVector(-1, 1), PointVector(1, 1)],
            [PointVector(-1, -1), PointVector(1, -1)],
        ),
        (
            [PointVector(-1, -1), PointVector(-1, 1)],
            [PointVector(1, -1), PointVector(1, 1)],
        ),
        (
            [PointVector(-1, -1), PointVector(1, -1)],
            [PointVector(-1, 1), PointVector(1, 1)],
        ),
    ];
}

pub mod rotation {
    use crate::point_vector::Point;
    use super::piece_constants::*;
    use super::types::*;

    pub const PIECE_ROTATIONS: [RotationLocations; NUM_PIECES] = [
        Z_ROTATIONS,
        L_ROTATIONS,
        O_ROTATIONS,
        S_ROTATIONS,
        I_ROTATIONS,
        J_ROTATIONS,
        T_ROTATIONS,
    ];

    const Z_ROTATIONS: RotationLocations = [
        [Point(1, -1), Point(1, 0), Point(0, 0), Point(0, 1)],
        [Point(1, 1), Point(0, 1), Point(0, 0), Point(-1, 0)],
        [Point(-1, 1), Point(-1, 0), Point(0, 0), Point(0, -1)],
        [Point(-1, -1), Point(0, -1), Point(0, 0), Point(1, 0)],
    ];

    const L_ROTATIONS: RotationLocations = [
        [Point(1, 1), Point(0, -1), Point(0, 0), Point(0, 1)],
        [Point(-1, 1), Point(1, 0), Point(0, 0), Point(-1, 0)],
        [Point(-1, -1), Point(0, 1), Point(0, 0), Point(0, -1)],
        [Point(1, -1), Point(-1, 0), Point(0, 0), Point(1, 0)],
    ];

    const O_ROTATIONS: RotationLocations = [
        [Point(1, 0), Point(1, 1), Point(0, 0), Point(0, 1)],
        [Point(0, 1), Point(-1, 1), Point(0, 0), Point(-1, 0)],
        [Point(-1, 0), Point(-1, -1), Point(0, 0), Point(0, -1)],
        [Point(0, -1), Point(1, -1), Point(0, 0), Point(1, 0)],
    ];

    const S_ROTATIONS: RotationLocations = [
        [Point(1, 0), Point(1, 1), Point(0, -1), Point(0, 0)],
        [Point(0, 1), Point(-1, 1), Point(1, 0), Point(0, 0)],
        [Point(-1, 0), Point(-1, -1), Point(0, 1), Point(0, 0)],
        [Point(0, -1), Point(1, -1), Point(-1, 0), Point(0, 0)],
    ];

    const I_ROTATIONS: RotationLocations = [
        [Point(0, -1), Point(0, 0), Point(0, 1), Point(0, 2)],
        [Point(1, 0), Point(0, 0), Point(-1, 0), Point(-2, 0)],
        [Point(0, 1), Point(0, 0), Point(0, -1), Point(0, -2)],
        [Point(-1, 0), Point(0, 0), Point(1, 0), Point(2, 0)],
    ];

    const J_ROTATIONS: RotationLocations = [
        [Point(1, -1), Point(0, -1), Point(0, 0), Point(0, 1)],
        [Point(1, 1), Point(1, 0), Point(0, 0), Point(-1, 0)],
        [Point(-1, 1), Point(0, 1), Point(0, 0), Point(0, -1)],
        [Point(-1, -1), Point(-1, 0), Point(0, 0), Point(1, 0)],
    ];

    const T_ROTATIONS: RotationLocations = [
        [Point(1, 0), Point(0, -1), Point(0, 0), Point(0, 1)],
        [Point(0, 1), Point(1, 0), Point(0, 0), Point(-1, 0)],
        [Point(-1, 0), Point(0, 1), Point(0, 0), Point(0, -1)],
        [Point(0, -1), Point(-1, 0), Point(0, 0), Point(1, 0)],
    ];
}

pub mod offset {
    use crate::point_vector::PointVector;
    use super::piece_constants::*;

    pub const THREE_OFFSETS: [[[PointVector; 5]; 2]; NUM_ROTATE_STATES] = [
        THREE_OFFSET_ZERO,
        THREE_OFFSET_ONE,
        THREE_OFFSET_TWO,
        THREE_OFFSET_THREE,
    ];

    pub const THREE_180_OFFSETS: [[PointVector; 6]; NUM_ROTATE_STATES] = [
        [
            PointVector(0, 0),
            PointVector(1, 0),
            PointVector(1, 1),
            PointVector(1, -1),
            PointVector(0, 1),
            PointVector(0, -1),
        ],
        [
            PointVector(0, 0),
            PointVector(0, 1),
            PointVector(2, 1),
            PointVector(1, 1),
            PointVector(2, 0),
            PointVector(-1, 0),
        ],
        [
            PointVector(0, 0),
            PointVector(-1, 0),
            PointVector(-1, -1),
            PointVector(-1, 1),
            PointVector(0, -1),
            PointVector(0, 1),
        ],
        [
            PointVector(0, 0),
            PointVector(0, -1),
            PointVector(2, -1),
            PointVector(1, -1),
            PointVector(2, 0),
            PointVector(-1, 0),
        ],
    ];

    pub const FIVE_OFFSETS: [[[PointVector; 5]; 2]; NUM_ROTATE_STATES] = [
        FIVE_OFFSET_ZERO,
        FIVE_OFFSET_ONE,
        FIVE_OFFSET_TWO,
        FIVE_OFFSET_THREE,
    ];

    pub const FIVE_180_OFFSETS: [[PointVector; 2]; NUM_ROTATE_STATES] = [
        [PointVector(-1, 1), PointVector(0, 1)],
        [PointVector(-1, -1), PointVector(-1, 0)],
        [PointVector(1, -1), PointVector(0, -1)],
        [PointVector(1, 1), PointVector(1, 0)],
    ];

    pub const O_OFFSETS: [[PointVector; 3]; NUM_ROTATE_STATES] = [
        [PointVector(1, 0), PointVector(1, 1), PointVector(0, 1)],
        [PointVector(0, 1), PointVector(-1, 1), PointVector(-1, 0)],
        [PointVector(-1, 0), PointVector(-1, -1), PointVector(0, -1)],
        [PointVector(0, -1), PointVector(1, -1), PointVector(1, 0)],
    ];

    const THREE_OFFSET_ZERO: [[PointVector; 5]; 2] = [
        [
            PointVector(0, 0),
            PointVector(0, -1),
            PointVector(1, -1),
            PointVector(-2, 0),
            PointVector(-2, -1),
        ],
        [
            PointVector(0, 0),
            PointVector(0, 1),
            PointVector(1, 1),
            PointVector(-2, 0),
            PointVector(-2, 1),
        ],
    ];

    const THREE_OFFSET_ONE: [[PointVector; 5]; 2] = [
        [
            PointVector(0, 0),
            PointVector(0, 1),
            PointVector(-1, 1),
            PointVector(2, 0),
            PointVector(2, 1),
        ],
        [
            PointVector(0, 0),
            PointVector(0, 1),
            PointVector(-1, 1),
            PointVector(2, 0),
            PointVector(2, 1),
        ],
    ];

    const THREE_OFFSET_TWO: [[PointVector; 5]; 2] = [
        [
            PointVector(0, 0),
            PointVector(0, 1),
            PointVector(1, 1),
            PointVector(-2, 0),
            PointVector(-2, 1),
        ],
        [
            PointVector(0, 0),
            PointVector(0, -1),
            PointVector(1, -1),
            PointVector(-2, 0),
            PointVector(-2, -1),
        ],
    ];

    const THREE_OFFSET_THREE: [[PointVector; 5]; 2] = [
        [
            PointVector(0, 0),
            PointVector(0, -1),
            PointVector(-1, -1),
            PointVector(2, 0),
            PointVector(2, -1),
        ],
        [
            PointVector(0, 0),
            PointVector(0, -1),
            PointVector(-1, -1),
            PointVector(2, 0),
            PointVector(2, -1),
        ],
    ];

    const FIVE_OFFSET_ZERO: [[PointVector; 5]; 2] = [
        [
            PointVector(0, 1),
            PointVector(0, 2),
            PointVector(0, -1),
            PointVector(-1, -1),
            PointVector(2, 2),
        ],
        [
            PointVector(-1, 0),
            PointVector(-1, -1),
            PointVector(-1, 2),
            PointVector(-2, 2),
            PointVector(2, -1),
        ],
    ];

    const FIVE_OFFSET_ONE: [[PointVector; 5]; 2] = [
        [
            PointVector(-1, 0),
            PointVector(-1, -1),
            PointVector(-1, 2),
            PointVector(1, -1),
            PointVector(-2, 2),
        ],
        [
            PointVector(0, -1),
            PointVector(0, -2),
            PointVector(0, 1),
            PointVector(-2, -2),
            PointVector(1, 1),
        ],
    ];

    const FIVE_OFFSET_TWO: [[PointVector; 5]; 2] = [
        [
            PointVector(0, -1),
            PointVector(0, 1),
            PointVector(0, -2),
            PointVector(1, 1),
            PointVector(-2, -2),
        ],
        [
            PointVector(1, 0),
            PointVector(1, -2),
            PointVector(1, 1),
            PointVector(2, -2),
            PointVector(-1, 1),
        ],
    ];

    const FIVE_OFFSET_THREE: [[PointVector; 5]; 2] = [
        [
            PointVector(1, 0),
            PointVector(1, 1),
            PointVector(1, -2),
            PointVector(-1, 1),
            PointVector(2, -2),
        ],
        [
            PointVector(0, 1),
            PointVector(0, 2),
            PointVector(0, -1),
            PointVector(2, 2),
            PointVector(-1, -1),
        ],
    ];
}

pub mod versus_constants {
    #[derive(Debug, PartialEq, Clone)]
    pub enum AttackType {
        None,
        S,
        D,
        T,
        Q,
        TSM,
        TDM,
        TS,
        TD,
        TT,
    }

    #[derive(Debug, PartialEq)]
    pub enum TSpinType {
        None,
        Full,
        Mini,
    }
}