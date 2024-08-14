#![allow(dead_code)]

use crate::board::Board;
use crate::constants::board_constants::*;
use crate::game::Game;
use crate::piece::Piece;

#[derive(Clone, Debug)]
pub struct Placement {
    pub game: Game, // game after the piece has been placed
    pub base_piece: Piece,
}

impl Ord for Placement {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.eval().total_cmp(&other.eval())
    }
}

impl Eq for Placement {}

impl PartialOrd for Placement {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Placement {
    fn eq(&self, other: &Self) -> bool {
        self.eval().eq(&other.eval())
    }
}

impl Placement {
    pub fn new(game: Game) -> Self {
        Self {
            game,
            base_piece: Piece::default(),
        }
    }

    pub fn get_last_piece(&self) -> Piece {
        panic!("deprecate this");
        Piece::decode((self.game.history & 0xFFFF) as u16)
    }

    // Eval functions ---------------
    // TODO: Change this so its not lazily evaluated. Currently only called when sorting, resulting in single-threaded eval
    pub fn eval(&self) -> f32 {
        let board = self.game.board.arr;

        let max = Board::get_max_height(&board);
        let min = Board::get_min_height(&board);
        let holes = Board::get_total_holes(&board);

        let messiness = Board::get_messiness(&board);
        let coveredness = Board::get_cell_coveredness(&board);

        let adj_diff = Board::get_adjacent_height_differences(&board);
        let stack_diff = Board::get_stack_height_differences(&board);

        // TODO: figure out why tslot isn't working
        // let tslot = Board::t_slot(&board);
        let tslot = 0;

        // (10 * max as u32 + holes) as f32

        (max + min + messiness + adj_diff + stack_diff - 5 * tslot + (holes + coveredness) as usize)
            as f32
    }

    /// returns the fumen string that represents the
    /// series of pieces that the placement stores
    pub fn get_fumen(&self) -> String {
        fn to_fumen(game: Game) -> fumen::Fumen {
            let mut fumen = fumen::Fumen::default();
            let page = fumen.add_page();
            for row in (0..VISIBLE_BOARD_HEIGHT).rev() {
                for col in 0..BOARD_WIDTH {
                    if game.board.get(row, col) {
                        page.field[row][col] = fumen::CellColor::Grey;
                    }
                }
            }
            fumen
        }

        fn add_page(fumen: &mut fumen::Fumen, game: Game) {
            let page = fumen.add_page();
            page.field = [[fumen::CellColor::Empty; 10]; 23]; // clear page
            for row in (0..VISIBLE_BOARD_HEIGHT).rev() {
                for col in 0..BOARD_WIDTH {
                    if game.board.get(row, col) {
                        page.field[row][col] = fumen::CellColor::Grey;
                    }
                }
            }
        }

        let mut games = self.game.past_states();
        let mut f = to_fumen(games.pop().unwrap());
        for game in games.into_iter().rev() {
            add_page(&mut f, game);
        }
        f.encode()
    }
}
