use crate::errors::GameError;
use crate::board::Board;

use std::fmt::{Display, Formatter};

use piece_data::*;
use piece_data::rotation::*;

#[derive(PartialEq, Copy, Clone)]
pub struct Placement {
    pub piece_type: Piece,
    pub rotation_state: RotationState,
    pub center: Point,
}

impl Default for Placement {
    fn default() -> Self {
        Self {
            piece_type: 0,
            rotation_state: 0,
            center: Point { row: SPAWN_ROW, col: SPAWN_COL },
        }
    }
}

impl Display for Placement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let locations = &PIECE_ROTATIONS[self.piece_type][self.rotation_state];

        let size: i8;

        if self.piece_type == 4 {
            size = 5;
        } else {
            size = 3;
        }
        let half_size = size / 2;

        for row in (0..size).rev() {
            for col in 0..size {
                let p = Mino(row - half_size, col - half_size);
                if locations.contains(&p) {
                    write!(f, "■ ")?
                } else { write!(f, "□ ")? }
            }
            write!(f, "\n")?
        }
        Ok(())
    }
}


impl Placement {
    pub fn get_size(&self) -> usize {
        return if self.piece_type == 4 {
            5
        } else {
            3
        };
    }

    pub fn abs_locations(&self) -> Result<[Point; PIECE_SIZE], GameError> {
        // errors if theres a negative index
        // TODO: REFACTOR, can use filter_map/map_err maybe

        let rotation_locations = &PIECE_ROTATIONS[self.piece_type][self.rotation_state];
        let mut out = [Point::default(); PIECE_SIZE];

        for i in 0..PIECE_SIZE {
            let added = rotation_locations[i].add(&self.center);
            if let Ok(add) = added {
                out[i] = add;
            } else {
                return Err(GameError::NotInBounds);
            }
        }

        Ok(out)
    }

    pub fn move_by_vector(&mut self, v: MoveVector) -> bool {
        if let Ok(p) = v.add_to_point(&self.center) {
            self.center = p;
            return true;
        }
        return false;
    }

    pub fn move_center_to_column(&mut self, col: usize) {
        self.center.col = col;
    }

    pub fn rotate(&mut self, direction: RotationDirection) {
        self.rotation_state = (self.rotation_state + direction) % NUM_ROTATE_STATES;
    }

    #[allow(unused)]
    pub fn new(piece_type: Piece) -> Self {
        Self {
            piece_type,
            ..Default::default()
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point {
    pub row: usize,
    pub col: usize,
}

impl Default for Point {
    fn default() -> Self {
        Self {
            row: 0,
            col: 0,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Mino(i8, i8);

impl Mino {
    fn add(&self, other: &Point) -> Result<Point, GameError> {
        let row = self.0 + other.row as i8;
        let col = self.1 + other.col as i8;

        if row < 0 || col < 0 {
            return Err(GameError::NotInBounds);
        }

        let row = row as usize;
        let col = col as usize;

        if Board::in_bounds(row, col).is_err() {
            return Err(GameError::NotInBounds);
        }

        Ok(Point { row, col })
    }
}

#[derive(Debug, Copy, Clone)]
pub struct MoveVector(pub i8, pub i8);

impl MoveVector {
    fn add_to_point(&self, other: &Point) -> Result<Point, GameError> {
        let row = self.0 + other.row as i8;
        let col = self.1 + other.col as i8;

        if row < 0 || col < 0 {
            return Err(GameError::NotInBounds);
        }

        let row = row as usize;
        let col = col as usize;

        if Board::in_bounds(row, col).is_err() {
            return Err(GameError::NotInBounds);
        }

        Ok(Point { row, col })
    }

    pub fn negative(&self) -> Self {
        Self(-self.0, -self.1)
    }
}

pub mod piece_data {
    use super::*;

    pub const PIECE_SIZE: usize = 4;
    pub const NUM_ROTATE_STATES: usize = 4;
    pub const SPAWN_ROW: usize = 21;
    pub const SPAWN_COL: usize = 4;

    pub type Piece = usize;

    pub mod rotation {
        use super::*;

        pub type RotationState = usize;
        pub type RotationDirection = usize;
        pub type RotationLocations = [[Mino; PIECE_SIZE]; NUM_ROTATE_STATES];

        pub const PIECE_ROTATIONS: [RotationLocations; 7] = [Z_ROTATIONS, L_ROTATIONS, O_ROTATIONS,
            S_ROTATIONS, I_ROTATIONS, J_ROTATIONS, T_ROTATIONS];

        const Z_ROTATIONS: RotationLocations = [
            [Mino(1, -1), Mino(1, 0), Mino(0, 0), Mino(0, 1)],
            [Mino(1, 1), Mino(0, 1), Mino(0, 0), Mino(-1, 0)],
            [Mino(-1, 1), Mino(-1, 0), Mino(0, 0), Mino(0, -1)],
            [Mino(-1, -1), Mino(0, -1), Mino(0, 0), Mino(1, 0)]
        ];


        const L_ROTATIONS: RotationLocations = [
            [Mino(1, 1), Mino(0, -1), Mino(0, 0), Mino(0, 1)],
            [Mino(-1, 1), Mino(1, 0), Mino(0, 0), Mino(-1, 0)],
            [Mino(-1, -1), Mino(0, 1), Mino(0, 0), Mino(0, -1)],
            [Mino(1, -1), Mino(-1, 0), Mino(0, 0), Mino(1, 0)]
        ];

        const O_ROTATIONS: RotationLocations = [
            [Mino(1, 0), Mino(1, 1), Mino(0, 0), Mino(0, 1)],
            [Mino(0, 1), Mino(-1, 1), Mino(0, 0), Mino(-1, 0)],
            [Mino(-1, 0), Mino(-1, -1), Mino(0, 0), Mino(0, -1)],
            [Mino(0, -1), Mino(1, -1), Mino(0, 0), Mino(1, 0)]
        ];

        const S_ROTATIONS: RotationLocations = [
            [Mino(1, 0), Mino(1, 1), Mino(0, -1), Mino(0, 0)],
            [Mino(0, 1), Mino(-1, 1), Mino(1, 0), Mino(0, 0)],
            [Mino(-1, 0), Mino(-1, -1), Mino(0, 1), Mino(0, 0)],
            [Mino(0, -1), Mino(1, -1), Mino(-1, 0), Mino(0, 0)]
        ];

        const I_ROTATIONS: RotationLocations = [
            [Mino(0, -1), Mino(0, 0), Mino(0, 1), Mino(0, 2)],
            [Mino(1, 0), Mino(0, 0), Mino(-1, 0), Mino(-2, 0)],
            [Mino(0, 1), Mino(0, 0), Mino(0, -1), Mino(0, -2)],
            [Mino(-1, 0), Mino(0, 0), Mino(1, 0), Mino(2, 0)]
        ];

        const J_ROTATIONS: RotationLocations = [
            [Mino(1, -1), Mino(0, -1), Mino(0, 0), Mino(0, 1)],
            [Mino(1, 1), Mino(1, 0), Mino(0, 0), Mino(-1, 0)],
            [Mino(-1, 1), Mino(0, 1), Mino(0, 0), Mino(0, -1)],
            [Mino(-1, -1), Mino(-1, 0), Mino(0, 0), Mino(1, 0)]
        ];

        const T_ROTATIONS: RotationLocations = [
            [Mino(1, 0), Mino(0, -1), Mino(0, 0), Mino(0, 1)],
            [Mino(0, 1), Mino(1, 0), Mino(0, 0), Mino(-1, 0)],
            [Mino(-1, 0), Mino(0, 1), Mino(0, 0), Mino(0, -1)],
            [Mino(0, -1), Mino(-1, 0), Mino(0, 0), Mino(1, 0)]
        ];
    }

    pub mod offset {
        use super::*;

        pub const THREE_OFFSETS: [[[MoveVector; 5]; 2]; NUM_ROTATE_STATES] = [
            THREE_OFFSET_ZERO, THREE_OFFSET_ONE, THREE_OFFSET_TWO, THREE_OFFSET_THREE
        ];

        pub const THREE_180_OFFSETS: [[MoveVector; 6]; NUM_ROTATE_STATES] = [
            [MoveVector(0, 0), MoveVector(1, 0), MoveVector(1, 1), MoveVector(1, -1), MoveVector(0, 1), MoveVector(0, -1)],
            [MoveVector(0, 0), MoveVector(0, 1), MoveVector(2, 1), MoveVector(1, 1), MoveVector(2, 0), MoveVector(-1, 0)],
            [MoveVector(0, 0), MoveVector(-1, 0), MoveVector(-1, -1), MoveVector(-1, 1), MoveVector(0, -1), MoveVector(0, 1)],
            [MoveVector(0, 0), MoveVector(0, -1), MoveVector(2, -1), MoveVector(1, -1), MoveVector(2, 0), MoveVector(-1, 0)],
        ];

        pub const FIVE_OFFSETS: [[[MoveVector; 5]; 2]; NUM_ROTATE_STATES] = [
            FIVE_OFFSET_ZERO, FIVE_OFFSET_ONE, FIVE_OFFSET_TWO, FIVE_OFFSET_THREE
        ];

        pub const FIVE_180_OFFSETS: [[MoveVector; 2]; NUM_ROTATE_STATES] = [
            [MoveVector(-1, 1), MoveVector(0, 1)],
            [MoveVector(-1, -1), MoveVector(-1, 0)],
            [MoveVector(1, -1), MoveVector(0, -1)],
            [MoveVector(1, 1), MoveVector(1, 0)]
        ];

        pub const O_OFFSETS: [[MoveVector; 3]; NUM_ROTATE_STATES] = [
            [MoveVector(1, 0), MoveVector(1, 1), MoveVector(0, 1)],
            [MoveVector(0, 1), MoveVector(-1, 1), MoveVector(-1, 0)],
            [MoveVector(-1, 0), MoveVector(-1, -1), MoveVector(0, -1)],
            [MoveVector(0, -1), MoveVector(1, -1), MoveVector(1, 0)
            ]
        ];

        const THREE_OFFSET_ZERO: [[MoveVector; 5]; 2] = [
            [MoveVector(0, 0), MoveVector(0, -1), MoveVector(1, -1), MoveVector(-2, 0), MoveVector(-2, -1)],
            [MoveVector(0, 0), MoveVector(0, 1), MoveVector(1, 1), MoveVector(-2, 0), MoveVector(-2, 1)],
        ];

        const THREE_OFFSET_ONE: [[MoveVector; 5]; 2] = [
            [MoveVector(0, 0), MoveVector(0, 1), MoveVector(-1, 1), MoveVector(2, 0), MoveVector(2, 1)],
            [MoveVector(0, 0), MoveVector(0, 1), MoveVector(-1, 1), MoveVector(2, 0), MoveVector(2, 1)],
        ];

        const THREE_OFFSET_TWO: [[MoveVector; 5]; 2] = [
            [MoveVector(0, 0), MoveVector(0, 1), MoveVector(1, 1), MoveVector(-2, 0), MoveVector(-2, 1)],
            [MoveVector(0, 0), MoveVector(0, -1), MoveVector(1, -1), MoveVector(-2, 0), MoveVector(-2, -1)],
        ];

        const THREE_OFFSET_THREE: [[MoveVector; 5]; 2] = [
            [MoveVector(0, 0), MoveVector(0, -1), MoveVector(-1, -1), MoveVector(2, 0), MoveVector(2, -1)],
            [MoveVector(0, 0), MoveVector(0, -1), MoveVector(-1, -1), MoveVector(2, 0), MoveVector(2, -1)],
        ];

        const FIVE_OFFSET_ZERO: [[MoveVector; 5]; 2] = [
            [MoveVector(0, 1), MoveVector(0, 2), MoveVector(0, -1), MoveVector(-1, -1), MoveVector(2, 2)],
            [MoveVector(-1, 0), MoveVector(-1, -1), MoveVector(-1, 2), MoveVector(-2, 2), MoveVector(2, -1)]
        ];

        const FIVE_OFFSET_ONE: [[MoveVector; 5]; 2] = [
            [MoveVector(-1, 0), MoveVector(-1, -1), MoveVector(-1, 2), MoveVector(1, -1), MoveVector(-2, 2)],
            [MoveVector(0, -1), MoveVector(0, -2), MoveVector(0, 1), MoveVector(-2, -2), MoveVector(1, 1)]
        ];

        const FIVE_OFFSET_TWO: [[MoveVector; 5]; 2] = [
            [MoveVector(0, -1), MoveVector(0, 1), MoveVector(0, -2), MoveVector(1, 1), MoveVector(-2, -2)],
            [MoveVector(1, 0), MoveVector(1, -2), MoveVector(1, 1), MoveVector(2, -2), MoveVector(-1, 1)]
        ];

        const FIVE_OFFSET_THREE: [[MoveVector; 5]; 2] = [
            [MoveVector(1, 0), MoveVector(1, 1), MoveVector(1, -2), MoveVector(-1, 1), MoveVector(2, -2)],
            [MoveVector(0, 1), MoveVector(0, 2), MoveVector(0, -1), MoveVector(2, 2), MoveVector(-1, -1)]
        ];
    }
}

#[cfg(test)]
mod piece_tests {
    use super::*;


    #[test]
    fn test_abs_locations() {
        // TODO: FIX TESTS WITH OFFSETS LATER

        let mut piece = create_preset_t();
        if let Ok(locations) = piece.abs_locations() {
            assert!(locations.contains(&Point { row: 2, col: 2 }));
            assert!(locations.contains(&Point { row: 3, col: 2 }));
            assert!(locations.contains(&Point { row: 2, col: 3 }));

            assert!(!locations.contains(&Point { row: 2, col: 4 }));
        } else {
            assert!(false)
        }

        let mut piece = create_preset_i();
        if let Ok(locations) = piece.abs_locations() {
            assert!(locations.contains(&Point { row: 4, col: 6 }));
            assert!(!locations.contains(&Point { row: 2, col: 4 }));
        } else {
            assert!(false)
        }

        let mut piece = Placement {
            piece_type: 0,
            rotation_state: 0,
            center: Point { row: 0, col: 0 },
        };

        assert!(piece.abs_locations().is_err());
    }

    #[test]
    fn test_rotate() {
        let mut piece = create_preset_i();
        piece.rotate(1);

        let locations = piece.abs_locations();

        assert_eq!(piece.rotation_state, 3);

        piece.rotate(2);

        assert_eq!(piece.rotation_state, 1);
    }

    #[test]
    fn test_move() {
        let mut piece = create_preset_s();
    }


    #[test]
    fn test_negative_center() {
        let piece = Placement {
            piece_type: 2,
            rotation_state: 3,
            center: Point { row: 0, col: 0 },
        };

        assert!(piece.abs_locations().is_err());
    }


    fn create_preset_i() -> Placement {
        Placement {
            piece_type: 4,
            rotation_state: 2,
            center: Point { row: 4, col: 6 },
        }
    }

    fn create_preset_s() -> Placement {
        Placement {
            piece_type: 3,
            rotation_state: 1,
            center: Point { row: 15, col: 5 },
        }
    }

    fn create_preset_t() -> Placement {
        Placement {
            piece_type: 6,
            rotation_state: 0,
            center: Point { row: 2, col: 2 },
        }
    }
}
