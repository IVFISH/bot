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
        [0b001, 0b011, 0b001],
        // E
        [0b000, 0b111, 0b010],
        // S
        [0b010, 0b011, 0b010],
        // W
        [0b010, 0b111, 0b000],
    ],
    // L
    [
        // N
        [0b001, 0b001, 0b011],
        // E
        [0b000, 0b111, 0b001],
        // S
        [0b011, 0b010, 0b010],
        // W
        [0b100, 0b111, 0b000],
    ],
];

pub const MASKS: [[[COL; 3]; 4]; 2] = [
    // T
    [
        // N
        [0x1249, 0x36db, 0x1249],
        // E
        [0x0000, 0x7fff, 0x2492],
        // S
        [0x2492, 0x36db, 0x2492],
        // W
        [0x2492, 0x7fff, 0x0000],
    ],
    // L
    [
        // N
        [0x1249, 0x1249, 0x36db],
        // E
        [0x0000, 0x7fff, 0x1249],
        // S
        [0x36db, 0x2492, 0x2492],
        // W
        [0x4924, 0x7fff, 0x0000],
    ],
];

// [OFFSET_R, OFFSET_C] = KICKS[piece][rot][ {CW, CCW} ][test#]
pub const KICKS: [[[[[i32; 2]; 4]; 2]; 4]; 5] = [
    // W/O OFFSET
    // [
    //     // N
    //     [
    //         // E
    //         [[-1, 0], [-1, 1], [0, -2], [-1, -2]],
    //         // W
    //         [[1, 0], [1, 1], [0, -2], [1, -2]],
    //     ],
    //     // S
    //     [
    //         // W
    //         [[1, 0], [1, 1], [0, -2], [1, -2]],
    //         // E
    //         [[-1, 0], [-1, 1], [0, -2], [-1, -2]],
    //     ],
    //     // E
    //     [
    //         // S
    //         [[-1, 0], [-1, 1], [0, -2], [-1, -2]],
    //         // N
    //         [[1, 0], [1, -1], [0, 2], [1, 2]],
    //     ],
    //     // W
    //     [
    //         // S
    //         [[-1, 0], [-1, -1], [0, 2], [-1, 2]],
    //         // N
    //         [[-1, 0], [-1, -1], [0, 2], [-1, 2]],
    //     ],
    // ]

    // for T, L, J, S, Z piece: 
    // all N offsets are 1 lower than in SRS standard
    [
        // N
        [
            // E
            [[-2, 0], [-2, 1], [-1, -2], [-2, -2]],
            // W
            [[0, 0], [0, 1], [-1, -2], [0, -2]],
        ],
        // S
        [
            // W
            [[1, 0], [1, 1], [0, -2], [1, -2]],
            // E
            [[-1, 0], [-1, 1], [0, -2], [-1, -2]],
        ],
        // E
        [
            // S
            [[0, 0], [0, 1], [1, -2], [0, -2]],
            // N
            [[2, 0], [2, -1], [1, 2], [2, 2]],
        ],
        // W
        [
            // S
            [[-1, 0], [-1, -1], [0, 2], [-1, 2]],
            // N
            [[0, 0], [0, -1], [1, 2], [0, 2]],
        ],
    ]; 5
];
