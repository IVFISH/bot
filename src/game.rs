#![allow(dead_code)]

use std::fmt::Display;

use crate::bitmatrix::Bitmatrix;

pub type COL = u16;
pub const W: usize = 10;
pub const H: usize = COL::BITS as usize;

#[derive(Debug, Copy, Clone)]
pub struct Game {
    pub board: Bitmatrix,
    pub queue: usize,
}

impl Game {
    pub fn new(queue: usize) -> Self {
        Self {
            board: Bitmatrix::new(),
            queue,
        }
    }

    pub fn next(queue: usize) -> (usize, usize) {
        (queue & 0xF, queue >> 4)
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.board.fmt(f)
    }
}

pub const PIECES: [[[COL; 3]; 4]; 2] = [
    // T
    [
        // N
        [0b100, 0b110, 0b100],
        // E
        [0b000, 0b111, 0b010],
        // S
        [0b010, 0b110, 0b010],
        // W
        [0b010, 0b111, 0b000],
    ],
    // L
    [
        // N
        [0b100, 0b100, 0b110],
        // E
        [0b000, 0b111, 0b100],
        // S
        [0b110, 0b010, 0b010],
        // W
        [0b001, 0b111, 0b000],
    ],
];

pub const MASKS: [[[COL; 3]; 4]; 2] = [
    // T
    [
        // N
        [0x9248, 0xdb6c, 0x9248],
        // E
        [0x0000, 0xfffe, 0x4924],
        // S
        [0x4924, 0xdb6c, 0x4924],
        // W
        [0x4924, 0xfffe, 0x0000],
    ],
    // L
    [
        // N
        [0x9248, 0x9248, 0xdb6c],
        // E
        [0x0000, 0xfffe, 0x9248],
        // S
        [0xdb6c, 0x4924, 0x4924],
        // W
        [0x2492, 0xfffe, 0x0000],
    ],
];

// [+X, +Y] = KICKS[piece][rot][ {CW, CCW} ][test#]
pub const KICKS: [[[[[i32; 2]; 5]; 2]; 4]; 5] = [
    // for T, L, J, S, Z piece: 
    // going to N has an implicit -1 offset (so explicitely should be +1)
    // going from N has an implicit +1 offset (explicit -1)
    // all N offsets are 1 lower than in SRS standard
    [
        // N
        [
            // E
            [[0, -1], [-1, -1], [-1, 0], [0, -3], [-1, -3]],
            // W
            [[0, -1], [1, -1], [1, 0], [0, -3], [1, -3]]
        ],
        // E
        [
            // S
            [[0, 0], [1, 0], [1, -1], [0, 2], [1, 2]],
            // N
            [[0, 1], [1, 1], [1, 0], [0, 3], [1, 3]],
        ],
        // S
        [
            // W
            [[0, 0], [1, 0], [1, 1], [0, -2], [1, -2]],
            // E
            [[0, 0], [-1, 0], [-1, 1], [0, -2], [-1, -2]],
        ],
        // W
        [
            // N
            [[0, 1], [-1, 1], [-1, 0], [0, 3], [-1, 3]],
            // S
            [[0, 0], [-1, 0], [-1, -1], [0, 2], [-1, 2]],
        ],
        ]; 5
        ];

// [+X, +Y] = KICKS[piece][rot][ {CW, CCW} ][test#]
pub const KICKS_180: [[[[i32; 2]; 6]; 4]; 5] = [
            // for T, L, J, S, Z piece: 
            // going to N has an implicit -1 offset (so explicitely should be +1)
            // going from N has an implicit +1 offset (explicit -1)
            // all N offsets are 1 lower than in SRS standard
            [
                // N
                [
                    // S
                    [0, -1], [0, 0], [1, 0], [-1, 0], [1, -1], [-1, -1],
                ],
                // E
                [
                    // W
                    [0, 0], [1, 0], [1, 2], [1, 1], [0, 2], [0, -1]
                ],
                // S
                [
                    // N
                    [0, 1], [0, 0], [-1, 0], [1, 0], [-1, 1], [1, 1]
                ],
                // W
                [
                    // E
                    [0, 0], [-1, 0], [-1, 2], [-1, 1], [0, 2], [0, -1]
                ],
                ]; 5
                ];
