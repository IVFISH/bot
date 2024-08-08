#![allow(dead_code)]

#[derive(Debug, Copy, Clone)]
pub struct Game {
    pub board: [u16; 10],
    pub queue: usize,
}

impl Game {
    pub fn new(queue: usize) -> Self {
        Self {
            board: [0; 10],
            queue,
        }
    }

    pub fn next(queue: usize) -> (usize, usize) {
        (queue & 0xF, queue >> 4)
    }
}

pub const PIECES: [[[usize; 3]; 4]; 3] = [
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

