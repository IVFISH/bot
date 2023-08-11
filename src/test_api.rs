#[allow(dead_code)]

pub mod functions {
    use crate::board::*;
    use crate::piece::*;
    use crate::placement::*;
    use crate::placement_list::*;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    pub fn add_list(board: &mut Board, list: Vec<[usize; 2]>) {
        for [r, c] in list.into_iter() {
            board.set(r, c, 1);
        }
    }

    pub fn remove_list(board: &mut Board, list: Vec<[usize; 2]>) {
        for [r, c] in list.into_iter() {
            board.set(r, c, 0);
        }
    }

    pub fn assert_placement_contains(placements: &PlacementList, piece: Piece) {
        placements.placements.iter().for_each(|p| println!("{:?}", p.get_last_piece()));
        assert!(placements.placements.iter().any(|p| p.get_last_piece() == piece));
    }

    pub fn assert_location_eq(locations: Option<[[usize; 2]; 4]>, sols: [[usize; 2]; 4]) {
        if let Some(mut locs) = locations {
            locs.sort();
            assert_eq!(locs, sols)
        } else {
            assert!(false)
        }
    }

    pub fn pco_board() -> Board {
        let mut board = Board::new();
        board.arr = [
            0b1111, 0b1111, 0b0111, 0b0010, 0b0, 0b0, 0b1111, 0b1111, 0b1111, 0b1111,
        ];
        board
    }

    pub fn pco_board_1() -> Board {
        let mut board = Board::new();
        board.arr = [
            0b1111, 0b1111, 0b0111, 0b0010, 0b0, 0b0, 0b0, 0b0, 0b1111, 0b1111,
        ];
        board
    }

    pub fn pco_board_2() -> Board {
        let mut board = Board::new();
        board.arr = [
            0b1111, 0b1111, 0b0111, 0b0010, 0b0, 0b0, 0b0, 0b0, 0b0, 0b1111,
        ];
        board
    }

    pub fn z_spin_board_1() -> Board {
        let mut board = Board::new();
        add_list(
            &mut board,
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
        board
    }

    pub fn z_spin_board_2() -> Board {
        let mut board = Board::new();
        add_list(
            &mut board,
            vec![
                [0, 0],
                [1, 0],
                [2, 0],
                [0, 1],
                [1, 1],
                [2, 1],
                [0, 2],
                [1, 2],
                [2, 2],
                [0, 3],
                [1, 3],
                [2, 3],
                [0, 4],
                [1, 6],
                [2, 6],
                [0, 7],
                [1, 7],
                [2, 7],
                [0, 8],
                [1, 8],
                [2, 8],
                [0, 9],
                [1, 9],
                [2, 9],
            ],
        );
        board
    }

    pub fn s_spin_board_1() -> Board {
        let mut board = Board::new();

        add_list(
            &mut board,
            vec![
                [0, 0],
                [1, 0],
                [0, 1],
                [1, 1],
                [0, 2],
                [1, 2],
                [0, 3],
                [1, 3],
                [1, 4],
                [0, 6],
                [0, 7],
                [1, 7],
                [0, 8],
                [1, 8],
                [0, 9],
                [1, 9],
            ],
        );
        board
    }

    pub fn s_spin_board_2() -> Board {
        let mut board = Board::new();
        add_list(
            &mut board,
            vec![
                [0, 0],
                [1, 0],
                [2, 0],
                [0, 1],
                [1, 1],
                [2, 1],
                [0, 2],
                [1, 2],
                [2, 2],
                [1, 3],
                [2, 3],
                [0, 5],
                [0, 6],
                [1, 6],
                [2, 6],
                [0, 7],
                [1, 7],
                [2, 7],
                [0, 8],
                [1, 8],
                [2, 8],
                [0, 9],
                [1, 9],
                [2, 9],
            ],
        );
        board
    }

    pub fn l_spin_board_1() -> Board {
        let mut board = Board::new();
        add_list(
            &mut board,
            vec![
                [0, 0],
                [1, 0],
                [2, 0],
                [0, 1],
                [1, 1],
                [2, 1],
                [0, 2],
                [1, 2],
                [2, 2],
                [2, 3],
                [0, 4],
                [0, 5],
                [0, 6],
                [1, 6],
                [2, 6],
                [0, 7],
                [1, 7],
                [2, 7],
                [0, 8],
                [1, 8],
                [2, 8],
                [0, 9],
                [1, 9],
                [2, 9],
            ],
        );
        board
    }

    pub fn l_spin_board_2() -> Board {
        let mut board = Board::new();
        add_list(
            &mut board,
            vec![
                [0, 0],
                [0, 1],
                [0, 2],
                [0, 3],
                [0, 5],
                [2, 5],
                [0, 6],
                [2, 6],
                [0, 7],
                [1, 7],
                [2, 7],
                [0, 8],
                [1, 8],
                [2, 8],
                [0, 9],
                [1, 9],
                [2, 9],
            ],
        );
        board
    }

    pub fn l_spin_board_3() -> Board {
        let mut board = Board::new();
        add_list(
            &mut board,
            vec![
                [0, 0],
                [1, 0],
                [2, 0],
                [0, 1],
                [1, 1],
                [2, 1],
                [0, 2],
                [1, 2],
                [2, 2],
                [0, 3],
                [1, 3],
                [2, 3],
                [2, 6],
                [0, 7],
                [1, 7],
                [2, 7],
                [0, 8],
                [1, 8],
                [2, 8],
                [0, 9],
            ],
        );
        board
    }

    pub fn j_spin_board_1() -> Board {
        let mut board = Board::new();
        add_list(
            &mut board,
            vec![
                [0, 0],
                [1, 0],
                [0, 1],
                [1, 1],
                [0, 2],
                [1, 2],
                [0, 3],
                [1, 3],
                [1, 5],
                [1, 6],
                [0, 7],
                [1, 7],
                [0, 8],
                [1, 8],
                [0, 9],
                [1, 9],
            ],
        );
        board
    }

    pub fn j_spin_board_2() -> Board {
        let mut board = Board::new();
        add_list(
            &mut board,
            vec![
                [0, 0],
                [1, 0],
                [2, 0],
                [0, 1],
                [1, 1],
                [2, 1],
                [0, 2],
                [1, 2],
                [2, 2],
                [0, 3],
                [1, 3],
                [2, 3],
                [0, 4],
                [0, 5],
                [2, 5],
                [2, 6],
                [0, 7],
                [1, 7],
                [2, 7],
                [0, 8],
                [1, 8],
                [2, 8],
                [0, 9],
                [1, 9],
            ],
        );
        board
    }

    pub fn j_spin_board_3() -> Board {
        let mut board = Board::new();
        add_list(
            &mut board,
            vec![
                [0, 0],
                [1, 0],
                [2, 0],
                [0, 1],
                [1, 1],
                [2, 1],
                [0, 2],
                [1, 2],
                [2, 2],
                [0, 3],
                [2, 3],
                [0, 4],
                [0, 6],
                [1, 6],
                [2, 6],
                [0, 7],
                [1, 7],
                [2, 7],
                [0, 8],
                [1, 8],
                [2, 8],
                [0, 9],
                [2, 9],
                [1, 9],
            ],
        );
        board
    }

    pub fn s_spin_board_3() -> Board {
        let mut board = Board::new();
        add_list(
            &mut board,
            vec![
                [0, 0],
                [1, 0],
                [2, 0],
                [0, 1],
                [1, 1],
                [2, 1],
                [0, 2],
                [1, 2],
                [2, 2],
                [0, 3],
                [2, 4],
                [0, 5],
                [1, 5],
                [2, 5],
                [3, 5],
                [0, 6],
                [1, 6],
                [2, 6],
                [3, 6],
                [0, 7],
                [1, 7],
                [2, 7],
                [3, 7],
                [0, 8],
                [1, 8],
                [2, 8],
                [3, 8],
                [0, 9],
                [1, 9],
                [2, 9],
                [3, 9],
            ],
        );
        board
    }

    pub fn s_spin_board_4() -> Board {
        let mut board = Board::new();
        add_list(
            &mut board,
            vec![
                [0, 0],
                [1, 0],
                [2, 0],
                [3, 0],
                [0, 1],
                [1, 1],
                [2, 1],
                [3, 1],
                [0, 2],
                [1, 2],
                [2, 2],
                [3, 2],
                [4, 2],
                [0, 3],
                [1, 3],
                [2, 3],
                [4, 3],
                [0, 4],
                [2, 5],
                [0, 6],
                [1, 6],
                [2, 6],
                [0, 7],
                [1, 7],
                [2, 7],
                [0, 8],
                [1, 8],
                [2, 8],
                [0, 9],
                [1, 9],
                [2, 9],
            ],
        );
        board
    }

    pub fn s_spin_board_5() -> Board {
        let mut board = Board::new();
        add_list(
            &mut board,
            vec![
                [0, 0],
                [1, 0],
                [2, 0],
                [3, 0],
                [4, 0],
                [0, 1],
                [1, 1],
                [2, 1],
                [3, 1],
                [4, 1],
                [0, 2],
                [1, 2],
                [2, 2],
                [3, 2],
                [4, 2],
                [0, 3],
                [4, 3],
                [2, 4],
                [0, 5],
                [1, 5],
                [2, 5],
                [0, 6],
                [1, 6],
                [2, 6],
                [3, 6],
                [4, 6],
                [5, 6],
                [0, 7],
                [1, 7],
                [2, 7],
                [3, 7],
                [4, 7],
                [5, 7],
                [0, 8],
                [1, 8],
                [2, 8],
                [3, 8],
                [4, 8],
                [5, 8],
                [0, 9],
                [1, 9],
                [2, 9],
                [3, 9],
                [4, 9],
                [5, 9],
            ],
        );
        board
    }

    pub fn z_spin_board_3() -> Board {
        let mut board = Board::new();
        add_list(
            &mut board,
            vec![
                [0, 0],
                [1, 0],
                [2, 0],
                [0, 1],
                [1, 1],
                [2, 1],
                [0, 2],
                [1, 2],
                [2, 2],
                [0, 3],
                [1, 3],
                [2, 3],
                [2, 4],
                [0, 5],
                [5, 5],
                [0, 6],
                [1, 6],
                [2, 6],
                [3, 6],
                [4, 6],
                [0, 7],
                [1, 7],
                [2, 7],
                [3, 7],
                [4, 7],
                [0, 8],
                [1, 8],
                [2, 8],
                [3, 8],
                [4, 8],
                [0, 9],
                [1, 9],
                [2, 9],
                [3, 9],
                [4, 9],
            ],
        );
        board
    }

    pub fn j_spin_board_4() -> Board {
        let mut board = Board::new();
        add_list(
            &mut board,
            vec![
                [0, 0],
                [1, 0],
                [2, 0],
                [0, 1],
                [1, 1],
                [2, 1],
                [0, 2],
                [1, 2],
                [2, 2],
                [2, 3],
                [1, 4],
                [2, 4],
            ],
        );
        board
    }

    pub fn j_spin_board_5() -> Board {
        let mut board = Board::new();
        add_list(
            &mut board,
            vec![
                [0, 0],
                [1, 0],
                [2, 0],
                [3, 0],
                [4, 0],
                [5, 0],
                [0, 1],
                [1, 1],
                [2, 1],
                [3, 1],
                [4, 1],
                [5, 1],
                [0, 2],
                [1, 2],
                [2, 2],
                [3, 2],
                [4, 2],
                [5, 2],
                [0, 3],
                [1, 3],
                [2, 3],
                [1, 4],
                [2, 4],
                [4, 5],
                [0, 6],
                [1, 6],
                [2, 6],
                [3, 6],
                [4, 6],
                [0, 7],
                [1, 7],
                [2, 7],
                [3, 7],
                [4, 7],
                [0, 8],
                [1, 8],
                [2, 8],
                [3, 8],
                [4, 8],
                [0, 9],
                [1, 9],
                [2, 9],
                [3, 9],
                [4, 9],
            ],
        );
        board
    }

    pub fn l_spin_board_4() -> Board {
        let mut board = Board::new();
        add_list(
            &mut board,
            vec![
                [0, 0],
                [1, 0],
                [2, 0],
                [3, 0],
                [4, 0],
                [5, 0],
                [6, 0],
                [0, 1],
                [1, 1],
                [2, 1],
                [3, 1],
                [4, 1],
                [5, 1],
                [6, 1],
                [0, 2],
                [1, 2],
                [2, 2],
                [3, 2],
                [4, 2],
                [5, 2],
                [6, 2],
                [0, 3],
                [1, 3],
                [2, 3],
                [3, 3],
                [4, 3],
                [5, 3],
                [6, 3],
                [4, 4],
                [1, 5],
                [4, 5],
                [0, 6],
                [1, 6],
                [2, 6],
                [4, 6],
                [0, 7],
                [1, 7],
                [2, 7],
                [0, 8],
                [1, 8],
                [2, 8],
                [3, 8],
                [4, 8],
                [5, 8],
                [0, 9],
                [1, 9],
                [2, 9],
                [3, 9],
                [4, 9],
                [5, 9],
            ],
        );
        board
    }

    pub fn l_spin_board_5() -> Board {
        let mut board = Board::new();
        add_list(
            &mut board,
            vec![
                [0, 0],
                [1, 0],
                [2, 0],
                [3, 0],
                [4, 0],
                [5, 0],
                [6, 0],
                [7, 0],
                [8, 0],
                [9, 0],
                [10, 0],
                [11, 0],
                [12, 0],
                [13, 0],
                [14, 0],
                [4, 1],
                [5, 1],
                [6, 1],
                [7, 1],
                [8, 1],
                [9, 1],
                [10, 1],
                [11, 1],
                [12, 1],
                [14, 1],
                [1, 2],
                [2, 2],
                [5, 2],
                [6, 2],
                [7, 2],
                [8, 2],
                [9, 2],
                [0, 3],
                [1, 3],
                [6, 3],
                [7, 3],
                [8, 3],
                [9, 3],
                [11, 3],
                [12, 3],
                [0, 4],
                [1, 4],
                [3, 4],
                [4, 4],
                [6, 4],
                [9, 4],
                [12, 4],
                [0, 5],
                [1, 5],
                [2, 5],
                [3, 5],
                [4, 5],
                [12, 5],
                [0, 6],
                [1, 6],
                [2, 6],
                [3, 6],
                [4, 6],
                [5, 6],
                [6, 6],
                [7, 6],
                [9, 6],
                [10, 6],
                [11, 6],
                [12, 6],
                [0, 7],
                [1, 7],
                [2, 7],
                [3, 7],
                [4, 7],
                [5, 7],
                [6, 7],
                [7, 7],
                [9, 7],
                [10, 7],
                [11, 7],
                [12, 7],
                [0, 8],
                [1, 8],
                [2, 8],
                [3, 8],
                [4, 8],
                [5, 8],
                [6, 8],
                [7, 8],
                [8, 8],
                [9, 8],
                [10, 8],
                [11, 8],
                [12, 8],
                [0, 9],
                [1, 9],
                [2, 9],
                [3, 9],
                [4, 9],
                [5, 9],
                [6, 9],
                [7, 9],
                [8, 9],
                [9, 9],
                [10, 9],
                [11, 9],
                [12, 9],
            ],
        );
        board
    }

    pub fn tst_board() -> Board {
        let mut board = Board::new();
        add_list(
            &mut board,
            vec![
                [1, 0],
                [0, 0],
                [0, 1],
                [0, 2],
                [2, 1],
                [2, 0],
                [1, 1],
                [2, 2],
                [0, 4],
                [2, 4],
                [1, 4],
                [3, 4],
                [4, 4],
                [4, 3],
                [4, 5],
                [3, 5],
                [1, 5],
                [1, 5],
                [2, 5],
                [0, 5],
                [2, 6],
                [1, 6],
                [0, 6],
                [2, 7],
                [1, 7],
                [0, 7],
                [2, 8],
                [1, 8],
                [0, 8],
                [2, 9],
                [1, 9],
                [0, 9],
            ],
        );
        board
    }

    pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }
}
