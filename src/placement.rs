#![allow(dead_code)]

use crate::command::Command;
use crate::constants::board_constants::*;
use crate::controller::Controller;
use crate::game::Game;
use crate::piece::Piece;
use fumen;

#[derive(Clone, Debug, Copy)]
pub struct Placement {
    pub game: Game, // game after the piece has been placed
}

impl Placement {
    pub fn new(game: Game) -> Self {
        Self { game }
    }

    pub fn get_last_piece(&self) -> Piece {
        Piece::decode((self.game.history & 0xFFFF) as u16)
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
