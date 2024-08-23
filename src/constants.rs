pub type COL = u16;
pub const W: usize = 10;
pub const H: usize = COL::BITS as usize;

pub const NUM_PIECES: usize = 7;
pub const PIECE_Z: usize = 1;
pub const PIECE_L: usize = 2;
pub const PIECE_O: usize = 3;
pub const PIECE_S: usize = 4;
pub const PIECE_I: usize = 5;
pub const PIECE_J: usize = 6;
pub const PIECE_T: usize = 7;

pub const PIECES: [[&[COL]; 4]; NUM_PIECES] = [
    // Z
    [
        // N
        &[0b010, 0b110, 0b100],
        // E
        &[0b000, 0b110, 0b011],
        // S
        &[0b010, 0b110, 0b100],
        // W
        &[0b110, 0b011, 0b000],
    ],
    // L
    [
        // N
        &[0b100, 0b100, 0b110],
        // E
        &[0b000, 0b111, 0b100],
        // S
        &[0b110, 0b010, 0b010],
        // W
        &[0b001, 0b111, 0b000],
    ],
    // O
    [
        // N
        &[0b110, 0b110, 0b000],
        // E
        &[0b110, 0b110, 0b000],
        // S
        &[0b110, 0b110, 0b000],
        // W
        &[0b110, 0b110, 0b000],
    ],
    // S
    [
        // N
        &[0b100, 0b110, 0b010],
        // E
        &[0b000, 0b011, 0b110],
        // S
        &[0b100, 0b110, 0b010],
        // W
        &[0b011, 0b110, 0b000],
    ],
    // I
    // representative bit for I is in row 0 col 2
    [
        // N
        &[0b1000, 0b1000, 0b1000, 0b1000],
        // E
        &[0b0000, 0b0000, 0b1111, 0b0000],
        // S
        &[0b1000, 0b1000, 0b1000, 0b1000],
        // W
        &[0b0000, 0b1111, 0b0000, 0b0000],
    ],
    // J
    [
        // N
        &[0b110, 0b100, 0b100],
        // E
        &[0b000, 0b111, 0b001],
        // S
        &[0b010, 0b010, 0b110],
        // W
        &[0b100, 0b111, 0b000],
    ],
    // T
    [
        // N
        &[0b100, 0b110, 0b100],
        // E
        &[0b000, 0b111, 0b010],
        // S
        &[0b010, 0b110, 0b010],
        // W
        &[0b010, 0b111, 0b000],
    ],
];

// WARNING: mask for I piece is fake
pub const MASKS: [[[COL; 3]; 4]; NUM_PIECES] = generate_masks();

const fn generate_masks() -> [[[COL; 3]; 4]; NUM_PIECES] {
    let mut masks = [[[0; 3]; 4]; NUM_PIECES];

    // https://rust-lang.github.io/rfcs/2344-const-looping.html
    let mut piece = 0;
    while piece < NUM_PIECES {
        let mut rot = 0;
        while rot < 4 {
            let mut col = 0;
            while col < 3 {
                let p = PIECES[piece][rot][col];
                masks[piece][rot][col] = (p << 1) | (p << 4) | (p << 7) | (p << 10) | (p << 13);

                col += 1;
            }
            rot += 1;
        }
        piece += 1;
    }

    masks
}

// for T, L, J, S, Z piece:
// in our implementation of PIECES,
// NORTH pieces are 1 lower from the standard SRS uses
// we compensate for this in our kick table:
// all X -> N offsets are 1 lower than in SRS standard
// all N -> X offsets are 1 higher than in SRS standard
const TLJSZ_KICKS: [[&[[i32; 2]]; 3]; 4] = [
    // N
    [
        // E
        &[[0, -1], [-1, -1], [-1, 0], [0, -3], [-1, -3]],
        // S
        &[[0, -1], [0, 0], [1, 0], [-1, 0], [1, -1], [-1, -1]],
        // W
        &[[0, -1], [1, -1], [1, 0], [0, -3], [1, -3]],
    ],
    // E
    [
        // S
        &[[0, 0], [1, 0], [1, -1], [0, 2], [1, 2]],
        // W
        &[[0, 0], [1, 0], [1, 2], [1, 1], [0, 2], [0, -1]],
        // N
        &[[0, 1], [1, 1], [1, 0], [0, 3], [1, 3]],
    ],
    // S
    [
        // W
        &[[0, 0], [1, 0], [1, 1], [0, -2], [1, -2]],
        // N
        &[[0, 1], [0, 0], [-1, 0], [1, 0], [-1, 1], [1, 1]],
        // E
        &[[0, 0], [-1, 0], [-1, 1], [0, -2], [-1, -2]],
    ],
    // W
    [
        // N
        &[[0, 1], [-1, 1], [-1, 0], [0, 3], [-1, 3]],
        // E
        &[[0, 0], [-1, 0], [-1, 2], [-1, 1], [0, 2], [0, -1]],
        // S
        &[[0, 0], [-1, 0], [-1, -1], [0, 2], [-1, 2]],
    ],
];

const O_KICKS: [[&[[i32; 2]]; 3]; 4] = [[&[[0; 2]]; 3]; 4];

// between SRS and our implementation: (+X, +Y)
// NORTH OFFSET: (-1, 0)
// EAST  OFFSET: (0, 2)
// SOUTH OFFSET: (0, 0)
// WEST  OFFSET: (-1, 1)
// ie: our N->W kick would have an explicit (0, -1) (N - W)
const I_KICKS: [[&[[i32; 2]]; 3]; 4] = [
    // N
    [
        // E
        // &[[1, 0], [2, 0], [-1, 0], [-1, -1], [2, 2]],
        &[[0, -2], [1, -2], [-2, -2], [-2, -3], [1, 0]],
        // S
        // &[[1, -1], [1, 0]],
        &[[0, -1], [0, 0]],
        // W
        // &[[0, -1], [-1, -1], [2, -1], [2, -2], [-1, 2]],
        &[[0, -2], [-1, -2], [2, -2], [2, -3], [-1, 1]],
    ],
    // E
    [
        // S
        // &[[0, -1], [-1, -1], [2, -1], [-1, 1], [2, -2]],
        &[[0, 1], [-1, 1], [2, 1], [-1, 3], [2, 0]],
        // W
        // &[[-1, -1], [0, -1]],
        &[[-1, 0], [0, 0]],
        // N
        // &[[-1, 0], [-2, 0], [1, 0], [-2, -2], [1, 1]],
        &[[0, 2], [-1, 2], [2, 2], [-1, 0], [2, 1]],
    ],
    // S
    [
        // W
        // &[[-1, 0], [1, 0], [-2, 0], [1, 1], [-2, -2]],
        &[[0, -1], [2, -1], [-1, -1], [2, -1], [-1, -3]],
        // N
        // &[[-1, 1], [-1, 0]],
        &[[0, 1], [0, 0]],
        // E
        // &[0, 1], [-2, 1], [1, 1], [-2, 2], [1, -1]
        &[[0, -1], [-2, -1], [1, -1], [-2, 0], [1, -3]],
    ],
    // W
    [
        // N
        // &[[0, 1], [1, 1], [-2, 1], [1, -1], [-2, 2]],
        &[[0, 2], [1, 2], [-2, 2], [1, 0], [-2, 3]],
        // E
        // &[[1, 1], [0, 1]],
        &[[0, 0], [-1, 0]],
        // S
        // &[[1, 0], [2, 0], [-1, 0], [2, 2], [-1, -1]],
        &[[0, 1], [1, 1], [-2, 1], [1, 3], [-2, 0]],
    ],
];

// [+X, +Y] = KICKS[piece][rot][ {CW, CCW} ][test#]
pub const KICKS: [[[&[[i32; 2]]; 3]; 4]; NUM_PIECES] = [
    // Z
    TLJSZ_KICKS,
    // L
    TLJSZ_KICKS,
    // O
    O_KICKS,
    // S
    TLJSZ_KICKS,
    // I
    I_KICKS,
    // J
    TLJSZ_KICKS,
    // T
    TLJSZ_KICKS,
];
