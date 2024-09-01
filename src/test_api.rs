#![allow(dead_code)]

pub mod test_api {
    use crate::bitmatrix::*;
    use crate::constants::*;
    use crate::game::*;

    pub fn bitmatrix_from_string(gamestr: &[&str]) -> Bitmatrix {
        let mut game = Bitmatrix::new();
        for (row, rowstr) in gamestr.iter().rev().enumerate() {
            for (col, char) in rowstr.chars().take(W).enumerate() {
                game[col] |= ((char.is_alphanumeric() as COL) << row).reverse_bits();
            }
        }

        game
    }

    pub fn game_from_string(gamestr: &[&str], queue: u64) -> Game {
        Game {
            queue,
            seed: 1,
            hold: 0,
            board: bitmatrix_from_string(gamestr),
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

    pub fn i_spin_game_1() -> Game {
        #[rustfmt::skip]
        let gamestr = [
            "........o.",
            "..........",
            "oooooo.oo.",
            "ooo....ooo",
        ];
        game_from_string(&gamestr, PIECE_I as u64)
    }

    pub fn i_spin_game_2() -> Game {
        #[rustfmt::skip]
        let gamestr = [
            ".o........",
            "..........",
            ".oo.ooooo.",
            "ooo....ooo",
        ];
        game_from_string(&gamestr, PIECE_I as u64)
    }

    pub fn i_spin_game_3() -> Game {
        #[rustfmt::skip]
        let gamestr = [
            "oooo.ooooo",
            "ooo....ooo",
        ];
        game_from_string(&gamestr, PIECE_I as u64)
    }

    pub fn i_spin_game_4() -> Game {
        #[rustfmt::skip]
        let gamestr = [
            "oooooo.oo.",
            "ooo....ooo",
        ];
        game_from_string(&gamestr, PIECE_I as u64)
    }

    pub fn l_spin_game_1() -> Game {
        #[rustfmt::skip]
        let gamestr = [
            "oooo..oooo",
            "ooo...oooo",
            "ooo.oooooo",
        ];
        game_from_string(&gamestr, PIECE_L as u64)
    }

    pub fn l_spin_game_2() -> Game {
        #[rustfmt::skip]
        let gamestr = [
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
        game_from_string(&gamestr, PIECE_L as u64)
    }

    pub fn l_spin_game_3() -> Game {
        #[rustfmt::skip]
        let gamestr = [
            "oooo.ooooo",
            "ooo...oooo",
            "ooo.oooooo",
        ];
        game_from_string(&gamestr, PIECE_L as u64)
    }

    pub fn l_spin_game_4() -> Game {
        #[rustfmt::skip]
        let gamestr = [
            "ooooo...oo",
            "oooo...ooo",
        ];
        game_from_string(&gamestr, PIECE_L as u64)
    }

    pub fn versus_game_medium() -> Game {
        #[rustfmt::skip]
        let gamestr = [
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
        game_from_string(&gamestr, PIECE_T as u64)
    }
}
