#![allow(dead_code)]

pub type COL = u16;
pub const W: usize = 10;
pub const H: usize = COL::BITS as usize;

#[derive(Debug, Copy, Clone)]
pub struct Game {
    pub board: [COL; W],
    pub queue: usize,
}

impl Game {
    pub fn new(queue: usize) -> Self {
        Self {
            board: [0; W],
            queue,
        }
    }

    pub fn next(queue: usize) -> (usize, usize) {
        (queue & 0xF, queue >> 4)
    }
}

pub const PIECES: [[[COL; 3]; 4]; 3] = [
    // T
    [
        // N
        [0b010, 0b011, 0b010],
        // E
        [0b000, 0b111, 0b010],
        // S
        [0b001, 0b011, 0b001],
        // W
        [0b010, 0b111, 0b000],
    ],
    // S
    [
        // N
        [0b010, 0b011, 0b001],
        // E
        [0b000, 0b011, 0b110],
        // S
        [0b010, 0b011, 0b001],
        // W
        [0b011, 0b110, 0b000],
    ],
    // L
    [
        // N
        [0b010, 0b010, 0b011],
        // E
        [0b000, 0b111, 0b100],
        // S
        [0b011, 0b001, 0b001],
        // W
        [0b001, 0b111, 0b000],
    ],
];

pub const MASKS: [[[COL; 3]; 4]; 3] = [
    // T
    [
        // N
        [0x1249, 0x36db, 0x1249],
        // E
        [
            0b0_000_000_000_000_000,
            0b0_111_111_111_111_111,
            0b0_010_010_010_010_010,
        ],
        // S
        [
            0b0_010_010_010_010_010,
            0b0_110_110_110_100_110,
            0b0_010_010_010_010_010,
        ],
        // W
        [
            0b0_010_010_010_010_010,
            0b0_111_111_111_111_111,
            0b0_000_000_000_000_000,
        ],
    ],
    // S
    [
        // N
        [0b010, 0b011, 0b001],
        // E
        [0b000, 0b011, 0b110],
        // S
        [0b010, 0b011, 0b001],
        // W
        [0b011, 0b110, 0b000],
    ],
    // L
    [
        // N
        [0b010, 0b010, 0b011],
        // E
        [0b000, 0b111, 0b100],
        // S
        [0b011, 0b001, 0b001],
        // W
        [0b001, 0b111, 0b000],
    ],
];
