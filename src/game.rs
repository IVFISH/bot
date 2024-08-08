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

    pub fn place(self, piece: usize, rot: usize, col: usize, queue: usize) -> Self
    {
        self
    }

    pub fn next(queue: usize) -> (usize, usize) {
        (queue & 0xF, queue >> 4)
    }
}
