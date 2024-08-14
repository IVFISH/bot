#[allow(dead_code)]
pub mod functions {
    use crate::board::*;
    use crate::constants::board_constants::BOARD_WIDTH;
    use crate::game::*;
    use crate::piece::*;

    use crate::placement_list::*;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    pub fn add_list(board: &mut Board, list: Vec<[usize; 2]>) {
        for [r, c] in list.into_iter() {
            board.set(r, c, 1);
        }
    }

    pub fn board_from_string(boardstr: &[&str]) -> Board {
        let mut board = Board::new();
        for (row, rowstr) in boardstr.iter().rev().enumerate() {
            for (col, char) in rowstr.chars().take(BOARD_WIDTH).enumerate() {
                board.set(row, col, char.is_alphanumeric() as usize)
            }
        }
        board
    }

    pub fn remove_list(board: &mut Board, list: Vec<[usize; 2]>) {
        for [r, c] in list.into_iter() {
            board.set(r, c, 0);
        }
    }

    /// Only checks the base piece
    pub fn assert_placement_contains(placements: &PlacementList, piece: Piece) {
        placements
            .placements
            .iter()
            .for_each(|p| println!("{:?}", p.base_piece));
        assert!(placements
            .placements
            .iter()
            .any(|p| p.base_piece == piece));
    }

    pub fn assert_location_eq(locations: Option<[[usize; 2]; 4]>, sols: [[usize; 2]; 4]) {
        if let Some(mut locs) = locations {
            locs.sort();
            assert_eq!(locs, sols)
        } else {
            panic!()
        }
    }

    pub fn assert_games_eq(v1: &[Game], v2: &[Game]) {
        assert_eq!(v1.len(), v2.len());
        for i in 0..v1.len() {
            println!("{}", i);
            assert_eq!(v1[i].board, v2[i].board);
        }
    }

    pub fn pco_board() -> Board {
        #[rustfmt::skip]
        let boardstr = [
            "oo....oooo",
            "ooo...oooo",
            "oooo..oooo",
            "ooo...oooo",
        ];
        board_from_string(&boardstr)
    }

    pub fn pco_board_1() -> Board {
        #[rustfmt::skip]
        let boardstr = [
            "oo......oo",
            "ooo.....oo",
            "oooo....oo",
            "ooo.....oo",
        ];
        board_from_string(&boardstr)
    }

    pub fn pco_board_2() -> Board {
        #[rustfmt::skip]
        let boardstr = [
            "oo.......o",
            "ooo......o",
            "oooo.....o",
            "ooo......o",
        ];
        board_from_string(&boardstr)
    }

    pub fn z_spin_board_1() -> Board {
        #[rustfmt::skip]
        let boardstr = [
            "ooo..ooooo",
            "oooo..oooo",
        ];
        board_from_string(&boardstr)
    }

    pub fn z_spin_board_2() -> Board {
        #[rustfmt::skip]
        let boardstr = [
            "oooo..oooo",
            "oooo..oooo",
            "ooooo..ooo",
        ];
        board_from_string(&boardstr)
    }

    pub fn s_spin_board_1() -> Board {
        #[rustfmt::skip]
        let boardstr = [
            "ooooo..ooo",
            "oooo..oooo",
        ];
        board_from_string(&boardstr)
    }

    pub fn s_spin_board_2() -> Board {
        #[rustfmt::skip]
        let boardstr = [
            "oooo..oooo",
            "oooo..oooo",
            "ooo..ooooo",
        ];
        board_from_string(&boardstr)
    }

    pub fn l_spin_board_1() -> Board {
        #[rustfmt::skip]
        let boardstr = [
            "oooo..oooo",
            "ooo...oooo",
            "ooo.oooooo",
        ];
        board_from_string(&boardstr)
    }

    pub fn l_spin_board_2() -> Board {
        #[rustfmt::skip]
        let boardstr = [
            ".....ooooo",
            ".......ooo",
            "oooo.ooooo",
        ];
        board_from_string(&boardstr)
    }

    pub fn l_spin_board_3() -> Board {
        #[rustfmt::skip]
        let boardstr = [
            "xxxx..xxx.",
            "xxxx...xx.",
            "xxxx...xxx",
        ];
        board_from_string(&boardstr)
    }

    pub fn j_spin_board_1() -> Board {
        #[rustfmt::skip]
        let boardstr = [
            "xxxx.xxxxx",
            "xxxx...xxx",
        ];
        board_from_string(&boardstr)
    }

    pub fn j_spin_board_2() -> Board {
        #[rustfmt::skip]
        let boardstr = [
            "xxxx.xxxx.",
            "xxxx...xxx",
            "xxxxxx.xxx",
        ];
        board_from_string(&boardstr)
    }

    pub fn j_spin_board_3() -> Board {
        #[rustfmt::skip]
        let boardstr = [
            "xxxx..xxxx",
            "xxx...xxxx",
            "xxxxx.xxxx",
        ];
        board_from_string(&boardstr)
    }

    pub fn s_spin_board_3() -> Board {
        #[rustfmt::skip]
        let boardstr = [
            ".....xxxxx",
            "xxx.xxxxxx",
            "xxx..xxxxx",
            "xxxx.xxxxx",
        ];
        board_from_string(&boardstr)
    }
    pub fn s_spin_board_4() -> Board {
        #[rustfmt::skip]
        let boardstr = [
            "..xx......",
            "xxx.......",
            "xxxx.xxxxx",
            "xxxx..xxxx",
            "xxxxx.xxxx",
        ];
        board_from_string(&boardstr)
    }

    pub fn s_spin_board_5() -> Board {
        #[rustfmt::skip]
        let boardstr = [
            "......xxxx",
            "xxxx..xxxx",
            "xxx...xxxx",
            "xxx.xxxxxx",
            "xxx..xxxxx",
            "xxxx.xxxxx",
        ];
        board_from_string(&boardstr)
    }

    pub fn z_spin_board_3() -> Board {
        #[rustfmt::skip]
        let boardstr = [
            ".....x....",
            "......xxxx",
            "......xxxx",
            "xxxxx.xxxx",
            "xxxx..xxxx",
            "xxxx.xxxxx",
        ];
        board_from_string(&boardstr)
    }

    pub fn j_spin_board_4() -> Board {
        #[rustfmt::skip]
        let boardstr = [
            "xxxxx.....",
            "xxx.x.....",
            "xxx.......",
        ];
        board_from_string(&boardstr)
    }

    pub fn j_spin_board_5() -> Board {
        #[rustfmt::skip]
        let boardstr = [
            "xxx.......",
            "xxx..xxxxx",
            "xxx...xxxx",
            "xxxxx.xxxx",
            "xxxxx.xxxx",
            "xxxx..xxxx",
        ];
        board_from_string(&boardstr)
    }

    pub fn l_spin_board_4() -> Board {
        #[rustfmt::skip]
        let boardstr = [
            "xxxx......",
            "xxxx....xx",
            "xxxxxxx.xx",
            "xxxx....xx",
            "xxxx..xxxx",
            "xxxx.xxxxx",
            "xxxx..xxxx",
        ];
        board_from_string(&boardstr)
    }

    pub fn l_spin_board_5() -> Board {
        #[rustfmt::skip]
        let boardstr = [
            "xx........",
            "x.........",
            "xx.xxxxxxx",
            "xx.x..xxxx",
            "xx....xxxx",
            "xxxxx.xxxx",
            "xxxx....xx",
            "xxxx..xxxx",
            "xxxxx.xxxx",
            "xxx...xxxx",
            "xx..xxxxxx",
            "x...xxxxxx",
            "x.x..xxxxx",
            "x.xxxxxxxx",
            "x..xxxxxxx",
        ];
        board_from_string(&boardstr)
    }

    pub fn tst_board() -> Board {
        #[rustfmt::skip]
        let boardstr = [
            "...xxx....",
            "....xx....",
            "xxx.xxxxxx",
            "xx..xxxxxx",
            "xxx.xxxxxx",
        ];
        board_from_string(&boardstr)
    }

    pub fn versus_board_short() -> Board {
        #[rustfmt::skip]
        let boardstr = [
            "......xx..",
            "..xx.xxxxx",
            "...xxxxxxx",
            "x.xxxxxxxx",
        ];
        board_from_string(&boardstr)
    }

    pub fn versus_board_medium() -> Board {
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
        board_from_string(&boardstr)
    }

    pub fn versus_board_tall() -> Board {
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
            "xxxx.xxxxx",
            "xxxxxxx.xx",
            "xxxxxxx.xx",
            "xxxxxxx.xx",
            "xxxxxxx.xx",
            "xxxxxxxx.x",
            "xxxxxxxx.x",
            "xxxxxxxx.x",
            "xx.xxxxxxx",
        ];
        board_from_string(&boardstr)
    }

    pub fn versus_board_cheese() -> Board {
        #[rustfmt::skip]
        let boardstr = [
            "x.xxxxxxxx",
            "xxxxxxx.xx",
            "xxxxxxxx.x",
            ".xxxxxxxxx",
            "xxx.xxxxxx",
            "x.xxxxxxxx",
            "xxxxxxx.xx",
            ".xxxxxxxxx",
            "xxxxxx.xxx",
            "xxxxxxxx.x",
            "x.xxxxxxxx",
            "xxx.xxxxxx",
        ];
        board_from_string(&boardstr)
    }

    pub fn versus_board_speculative() -> Board {
        #[rustfmt::skip]
        let boardstr = [
            ".....x....",
            "....xxx...",
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
        board_from_string(&boardstr)
    }

    pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::functions::*;
    use crate::board::Board;

    #[test]
    fn test_board_from_string() {
        #[rustfmt::skip]
        let boardstr = [];
        let board = board_from_string(&boardstr);
        println!("{}", board);
        assert_eq!(board, Board::new());

        #[rustfmt::skip]
        let boardstr = [
            "ooo..ooooo",
            "oooo..oooo"
        ];
        let board = board_from_string(&boardstr);

        let mut test_board = Board::new();
        add_list(
            &mut test_board,
            vec![
                [0, 0],
                [1, 0],
                [0, 1],
                [1, 1],
                [0, 2],
                [1, 2],
                [0, 3],
                [1, 5],
                [0, 6],
                [1, 6],
                [0, 7],
                [1, 7],
                [0, 8],
                [1, 8],
                [0, 9],
                [1, 9],
            ],
        );
        assert_eq!(test_board, board);
    }
}
