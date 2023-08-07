#![allow(dead_code)]

use crate::command::Command;
use crate::constants::board_constants::*;
use crate::controller::Controller;
use crate::game::Game;
use crate::piece::Piece;
use itertools::chain;
use fumen;

pub struct Placement {
    // the last piece in the move sequence
    pub piece: Piece,
    pub held: bool,

    // add the score and any other info here

    // games
    pub game_before: Game, // game at depth = n-1
    pub game_after: Game,  // game at depth = n
}

impl Placement {
    /// returns the fumen string that represents the
    /// series of pieces that the placement stores
    pub fn get_fumen(&self) -> String {
        fn to_fumen_piece(piece: Piece) -> fumen::Piece {
            let rotation = match piece.dir {
                0 => fumen::RotationState::North,
                1 => fumen::RotationState::East,
                2 => fumen::RotationState::South,
                _ => fumen::RotationState::West,
            };

            let kind = match piece.r#type {
                0 => fumen::PieceType::Z,
                1 => fumen::PieceType::L,
                2 => fumen::PieceType::O,
                3 => fumen::PieceType::S,
                4 => fumen::PieceType::I,
                5 => fumen::PieceType::J,
                _ => fumen::PieceType::T,
            };

            fumen::Piece {
                x: piece.col as u32,
                y: piece.row as u32,
                kind,
                rotation,
            }
        }

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

        unimplemented!()

        // let mut f = to_fumen(*self.game);
        // for piece in self.pieces.iter() {
        //     f.add_page().piece = Some(to_fumen_piece(*piece));
        // }
        // f.add_page().piece = Some(to_fumen_piece(self.piece));
        // f.encode()
    }
}

