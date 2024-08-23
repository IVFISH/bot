#![allow(dead_code)]

pub mod test_api {
    use crate::bitmatrix::*;
    use crate::constants::*;
    use crate::game::*;

    pub fn bitmatrix_from_string(boardstr: &[&str]) -> Bitmatrix {
        let mut board = Bitmatrix::new();
        for (row, rowstr) in boardstr.iter().rev().enumerate() {
            for (col, char) in rowstr.chars().take(W).enumerate() {
                board[col] |= ((char.is_alphanumeric() as COL) << row).reverse_bits();
            }
        }

        board
    }

    pub fn game_from_string(boardstr: &[&str], queue: u64) -> Game {
        Game {
            queue,
            seed: 1,
            board: bitmatrix_from_string(boardstr),
        }
    }

    pub fn assert_contains(movegen: &Vec<Game>, game: Game) {
        let contains = movegen.iter().any(|g| g.board == game.board);
        assert!(contains);
    }

    pub fn assert_not_contains(movegen: &Vec<Game>, game: Game) {
        let contains = movegen.iter().any(|g| g.board == game.board);
        assert!(!contains);
    }

    pub fn i_spin_board_1() -> Game {
        #[rustfmt::skip]
        let boardstr = [
            "........o.",
            "..........",
            "oooooo.oo.",
            "ooo....ooo",
        ];
        game_from_string(&boardstr, PIECE_I as u64)
    }

    pub fn i_spin_board_2() -> Game {
        #[rustfmt::skip]
        let boardstr = [
            ".o........",
            "..........",
            ".oo.ooooo.",
            "ooo....ooo",
        ];
        game_from_string(&boardstr, PIECE_I as u64)
    }

    pub fn i_spin_board_3() -> Game {
        #[rustfmt::skip]
        let boardstr = [
            "oooo.ooooo",
            "ooo....ooo",
        ];
        game_from_string(&boardstr, PIECE_I as u64)
    }

    pub fn i_spin_board_4() -> Game {
        #[rustfmt::skip]
        let boardstr = [
            "oooooo.oo.",
            "ooo....ooo",
        ];
        game_from_string(&boardstr, PIECE_I as u64)
    }

    pub fn l_spin_board_1() -> Game {
        #[rustfmt::skip]
        let boardstr = [
            "oooo..oooo",
            "ooo...oooo",
            "ooo.oooooo",
        ];
        game_from_string(&boardstr, PIECE_L as u64)
    }

    pub fn l_spin_board_2() -> Game {
        #[rustfmt::skip]
        let boardstr = [
            "oo........",
            "o.........",
            "oo.ooooooo",
            "oo.o..oooo",
            "oo....oooo",
            "ooooo.oooo",
            "oooo....oo",
            "oooo..oooo",
            "ooooo.oooo",
            "ooo...oooo",
            "oo..oooooo",
            "o...oooooo",
            "o.o..ooooo",
            "o.oooooooo",
            "o..ooooooo",
        ];
        game_from_string(&boardstr, PIECE_L as u64)
    }

    pub fn l_spin_board_3() -> Game {
        #[rustfmt::skip]
        let boardstr = [
            "oooo.ooooo",
            "ooo...oooo",
            "ooo.oooooo",
        ];
        game_from_string(&boardstr, PIECE_L as u64)
    }

    pub fn l_spin_board_4() -> Game {
        #[rustfmt::skip]
        let boardstr = [
            "ooooo...oo",
            "oooo...ooo",
        ];
        game_from_string(&boardstr, PIECE_L as u64)
    }

    pub fn versus_board_medium() -> Game {
        #[rustfmt::skip]
        let boardstr = [
            "......xx..",
            "..xx.xxxxx",
            "...xxxxxxx",
            "x.xxxxxxxx",
            "x.xxxxxxxx",
            "x.xxxxxxxx",
            "x.xxxxxxxx",
            "x.xxxxxxxx",
            "xxxxxxx.xx",
        ];
        game_from_string(&boardstr, PIECE_T as u64)
    }
}
