use crate::board::*;
use crate::placement::*;
use crate::queue::*;


struct Game {
    board: Board,

    piece_queue: PieceQueue,
    garbage_queue: GarbageQueue,

    game_data: GameData,

}

struct GameData {
    all_clear: bool,
    combo: i8,
    b2b: i8,

    pieces_placed: u8,
    lines_cleared: u8,
    lines_sent: u8,

    game_over: bool,

    time: f32
}