#![allow(dead_code)]

pub mod types {
    use std::collections::VecDeque;
    use crate::constants::board_constants::*;
    use crate::opener::Dependency;
    use super::bot_constants::*;
    use super::piece_constants::*;
    use crate::piece::Piece;
    use crate::point_vector::Point;

    pub type BoardArray = [usize; BOARD_WIDTH];
    pub type Row = usize;
    pub type Column = usize;
    pub type PieceType = usize;
    pub type RotationState = usize;
    pub type RotationDirection = usize;
    pub type BagNumber = usize;
    pub type RotationLocations = [[Point; PIECE_SIZE]; NUM_ROTATE_STATES];
    pub type CommandList = Vec<Command>;
    pub type Score = f32;
    pub type MoveList = Vec<CommandList>;
    pub type ScoreList = Vec<(Score, Score)>;
    pub type PlacementList = Vec<Piece>;
    pub type BagPlacement = [Piece; 7];
    pub type BagPlacements = Vec<BagPlacement>;
    pub type OpenerSequence = Vec<BagPlacements>;
    pub type Dependencies = Vec<Dependency>;
    pub type PieceOrder = Vec<PieceType>;
}

pub mod board_constants {
    pub const BOARD_WIDTH: usize = 10;
    pub const BOARD_HEIGHT: usize = 40;
    pub const VISIBLE_BOARD_HEIGHT: usize = 23;
    pub const MAX_PLACE_HEIGHT: usize = 20;

    pub const ZERO_ONE: usize = 0x5555555555;
    pub const ONE_ZERO: usize = 0xAAAAAAAAAA;

    pub const CONSOLE_DISPLAY_STATS: bool = true;
}

pub mod piece_constants {
    use crate::point_vector::PointVector;

    pub const PIECE_SIZE: usize = 4;
    pub const NUM_ROTATE_STATES: usize = 4;
    pub const SPAWN_ROW: i8 = 21;
    pub const SPAWN_COL: i8 = 4;
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

pub mod versus_constants {
    #[derive(Debug, PartialEq, Clone, Copy)]
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

    #[derive(Copy, Clone, Debug, PartialEq)]
    pub enum TSpinType {
        None,
        Full,
        Mini,
    }
}

pub mod queue_constants {
    pub const MIN_QUEUE_LENGTH: usize = 13;

    // lehmer RNG (MINSTD)
    pub const MULTIPLIER: usize = 16807;
    pub const MODULUS: usize = 2147483647;

    pub const CONSOLE_DISPLAY_QUEUE: bool = true;
}

pub mod bot_constants {
    use std::fmt::{Display, Formatter};
    use crate::game::Game;

    #[derive(Copy, Clone, Debug, PartialEq)]

    pub enum Command {
        None,
        MoveLeft,
        MoveRight,
        SoftDrop,
        RotateCW,
        RotateCCW,
        Rotate180,
        HardDrop,
        Hold,
    }

    impl Display for Command {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                Command::None => write!(f, "None")?,
                Command::MoveLeft => write!(f, "MoveLeft")?,
                Command::MoveRight => write!(f, "MoveRight")?,
                Command::SoftDrop => write!(f, "SoftDrop")?,
                Command::RotateCW => write!(f, "RotateCW")?,
                Command::RotateCCW => write!(f, "RotateCCW")?,
                Command::Rotate180 => write!(f, "Rotate180")?,
                Command::Hold => write!(f, "Hold")?,
                Command::HardDrop => write!(f, "HardDrop")?,
            }

            Ok(())
        }
    }

    pub const ROTATIONS: [Command; 4] = [
        Command::None,
        Command::RotateCW,
        Command::Rotate180,
        Command::RotateCCW,
    ];

    pub const DIRECTIONS: [Command; 2] = [Command::MoveRight, Command::MoveLeft];

    pub const COMMANDS: [Command; 5] = [
        Command::MoveRight,
        Command::MoveLeft,
        Command::RotateCW,
        Command::RotateCCW,
        Command::Rotate180,
        // Command::SoftDrop,
    ];

    pub const ACTIONS: [fn(&mut Game) -> bool; 5] = [
        Game::active_right,
        Game::active_left,
        Game::active_cw,
        Game::active_ccw,
        Game::active_180,
        // Game::active_drop,
    ];

    pub const MOVEPLACEMENTSCORE: usize = 7;
    pub const PANICBURST: usize = 0; // test and will likely not be the function used later
}

pub mod rotation {
    use super::piece_constants::*;
    use super::types::*;
    use crate::point_vector::Point;

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
    use super::piece_constants::*;
    use crate::point_vector::PointVector;

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

pub mod localbotgameplay {
    pub const ALLOWLOCALGAMEPLAY: bool = true;
    pub const LOCALGAMEPLAYFILEPATH: &str = r".\src\localdata.txt";
    pub const BOTNUM: usize = 0;
    pub const BOTNUM2: usize = 1;
}