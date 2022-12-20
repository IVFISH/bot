#![allow(dead_code)]


pub mod openers {
    use crate::{Dependencies, Dependency, Opener, Piece, Point};

    pub fn ndt() -> Opener {
        Opener::new(
            vec![
                vec![
                    [
                        Piece { piece_type: 0, rotation_state: 0, center: Point(1, 8), last_kick: 0 },
                        Piece { piece_type: 1, rotation_state: 0, center: Point(0, 4), last_kick: 0 },
                        Piece { piece_type: 2, rotation_state: 0, center: Point(0, 0), last_kick: 0 },
                        Piece { piece_type: 3, rotation_state: 0, center: Point(1, 4), last_kick: 0 },
                        Piece { piece_type: 4, rotation_state: 1, center: Point(2, 6), last_kick: 0 },
                        Piece { piece_type: 5, rotation_state: 0, center: Point(0, 8), last_kick: 0 },
                        Piece { piece_type: 6, rotation_state: 0, center: Point(3, 4), last_kick: 0 },
                    ],
                    [
                        Piece { piece_type: 0, rotation_state: 1, center: Point(1, 3), last_kick: 0 },
                        Piece { piece_type: 1, rotation_state: 1, center: Point(1, 7), last_kick: 0 },
                        Piece { piece_type: 2, rotation_state: 0, center: Point(0, 0), last_kick: 0 },
                        Piece { piece_type: 3, rotation_state: 1, center: Point(1, 8), last_kick: 0 },
                        Piece { piece_type: 4, rotation_state: 1, center: Point(2, 6), last_kick: 0 },
                        Piece { piece_type: 5, rotation_state: 3, center: Point(1, 5), last_kick: 0 },
                        Piece { piece_type: 6, rotation_state: 0, center: Point(3, 4), last_kick: 0 },
                    ]],
                vec![
                    [
                        Piece { piece_type: 0, rotation_state: 1, center: Point(5, 6), last_kick: 0 },
                        Piece { piece_type: 1, rotation_state: 3, center: Point(5, 3), last_kick: 0 },
                        Piece { piece_type: 2, rotation_state: 0, center: Point(3, 7), last_kick: 0 },
                        Piece { piece_type: 3, rotation_state: 1, center: Point(5, 4), last_kick: 0 },
                        Piece { piece_type: 4, rotation_state: 1, center: Point(4, 9), last_kick: 0 },
                        Piece { piece_type: 5, rotation_state: 1, center: Point(3, 0), last_kick: 0 },
                        Piece { piece_type: 6, rotation_state: 2, center: Point(2, 2), last_kick: 0 },
                    ],
                    [
                        Piece { piece_type: 0, rotation_state: 1, center: Point(5, 6), last_kick: 0 },
                        Piece { piece_type: 1, rotation_state: 3, center: Point(5, 3), last_kick: 0 },
                        Piece { piece_type: 2, rotation_state: 0, center: Point(3, 7), last_kick: 0 },
                        Piece { piece_type: 3, rotation_state: 1, center: Point(5, 4), last_kick: 0 },
                        Piece { piece_type: 4, rotation_state: 1, center: Point(4, 9), last_kick: 0 },
                        Piece { piece_type: 5, rotation_state: 1, center: Point(3, 0), last_kick: 0 },
                        Piece { piece_type: 6, rotation_state: 2, center: Point(2, 2), last_kick: 0 },
                    ],
                    [
                        Piece { piece_type: 0, rotation_state: 1, center: Point(5, 6), last_kick: 0 },
                        Piece { piece_type: 1, rotation_state: 3, center: Point(5, 3), last_kick: 0 },
                        Piece { piece_type: 2, rotation_state: 0, center: Point(3, 7), last_kick: 0 },
                        Piece { piece_type: 3, rotation_state: 1, center: Point(5, 4), last_kick: 0 },
                        Piece { piece_type: 4, rotation_state: 1, center: Point(4, 9), last_kick: 0 },
                        Piece { piece_type: 5, rotation_state: 1, center: Point(3, 0), last_kick: 0 },
                        Piece { piece_type: 6, rotation_state: 2, center: Point(2, 2), last_kick: 0 },
                    ],
                ],
                vec![
                    [
                        Piece { piece_type: 0, rotation_state: 1, center: Point(6, 8), last_kick: 0 },
                        Piece { piece_type: 1, rotation_state: 3, center: Point(8, 4), last_kick: 0 },
                        Piece { piece_type: 2, rotation_state: 0, center: Point(7, 2), last_kick: 0 },
                        Piece { piece_type: 3, rotation_state: 2, center: Point(8, 8), last_kick: 0 },
                        Piece { piece_type: 4, rotation_state: 1, center: Point(8, 5), last_kick: 0 },
                        Piece { piece_type: 5, rotation_state: 1, center: Point(7, 6), last_kick: 0 },
                        Piece { piece_type: 6, rotation_state: 3, center: Point(3, 2), last_kick: 0 },
                    ],
                ],
                vec![
                    [
                        Piece { piece_type: 0, rotation_state: 0, center: Point(10, 4), last_kick: 0 },
                        Piece { piece_type: 1, rotation_state: 1, center: Point(6, 1), last_kick: 0 },
                        Piece { piece_type: 2, rotation_state: 0, center: Point(10, 6), last_kick: 0 },
                        Piece { piece_type: 3, rotation_state: 3, center: Point(9, 1), last_kick: 0 },
                        Piece { piece_type: 4, rotation_state: 0, center: Point(9, 7), last_kick: 0 },
                        Piece { piece_type: 5, rotation_state: 3, center: Point(11, 9), last_kick: 0 },
                        Piece { piece_type: 6, rotation_state: 2, center: Point(10, 2), last_kick: 0 },
                    ],
                    [
                        Piece { piece_type: 0, rotation_state: 0, center: Point(10, 4), last_kick: 0 },
                        Piece { piece_type: 1, rotation_state: 1, center: Point(6, 1), last_kick: 0 },
                        Piece { piece_type: 2, rotation_state: 0, center: Point(9, 7), last_kick: 0 },
                        Piece { piece_type: 3, rotation_state: 3, center: Point(9, 1), last_kick: 0 },
                        Piece { piece_type: 4, rotation_state: 1, center: Point(11, 9), last_kick: 0 },
                        Piece { piece_type: 5, rotation_state: 1, center: Point(10, 6), last_kick: 0 },
                        Piece { piece_type: 6, rotation_state: 2, center: Point(10, 2), last_kick: 0 },
                    ],
                    [
                        Piece { piece_type: 0, rotation_state: 0, center: Point(10, 4), last_kick: 0 },
                        Piece { piece_type: 1, rotation_state: 1, center: Point(6, 1), last_kick: 0 },
                        Piece { piece_type: 2, rotation_state: 0, center: Point(9, 7), last_kick: 0 },
                        Piece { piece_type: 3, rotation_state: 3, center: Point(9, 1), last_kick: 0 },
                        Piece { piece_type: 4, rotation_state: 1, center: Point(11, 9), last_kick: 0 },
                        Piece { piece_type: 5, rotation_state: 1, center: Point(10, 6), last_kick: 0 },
                        Piece { piece_type: 6, rotation_state: 2, center: Point(10, 2), last_kick: 0 },
                    ],
                ]
            ],
            vec![
                vec![
                    vec![
                        Dependency { dependency: vec![1, 3, 6] }, // L<S<T
                        Dependency { dependency: vec![5, 0] },    // J<Z
                    ],
                    vec![
                        Dependency { dependency: vec![5, 6] },    // J<T
                        Dependency { dependency: vec![1, 4] },    // L<I
                        Dependency { dependency: vec![5, 4] },    // J<I
                    ]],
                vec![
                    vec![
                        Dependency { dependency: vec![2, 0] },   // O<Z
                        Dependency { dependency: vec![0, 6] },   // T must be last
                        Dependency { dependency: vec![1, 6] },
                        Dependency { dependency: vec![2, 6] },
                        Dependency { dependency: vec![3, 6] },
                        Dependency { dependency: vec![4, 6] },
                        Dependency { dependency: vec![5, 6] },
                    ],
                    vec![
                        Dependency { dependency: vec![2, 4] },   // O<I
                        Dependency { dependency: vec![0, 6] },   // T must be last
                        Dependency { dependency: vec![1, 6] },
                        Dependency { dependency: vec![2, 6] },
                        Dependency { dependency: vec![3, 6] },
                        Dependency { dependency: vec![4, 6] },
                        Dependency { dependency: vec![5, 6] },
                    ],
                    vec![
                        Dependency { dependency: vec![1, 5] },   // L<J
                        Dependency { dependency: vec![2, 0] },   // O<Z
                    ]],
                vec![
                    vec![
                        Dependency { dependency: vec![0, 3] },   // Z<S
                    ]],
                vec![
                    vec![
                        Dependency { dependency: vec![1, 3] },   // L<S
                        Dependency { dependency: vec![4, 2] },   // I<O
                        Dependency { dependency: vec![4, 5] },   // I<J
                        Dependency { dependency: vec![0, 6] },   // T must be last
                        Dependency { dependency: vec![1, 6] },
                        Dependency { dependency: vec![2, 6] },
                        Dependency { dependency: vec![3, 6] },
                        Dependency { dependency: vec![4, 6] },
                        Dependency { dependency: vec![5, 6] },
                    ],
                    vec![
                        Dependency { dependency: vec![1, 3] },   // L<S
                        Dependency { dependency: vec![2, 5] },   // O<J
                        Dependency { dependency: vec![0, 6] },   // T must be last
                        Dependency { dependency: vec![1, 6] },
                        Dependency { dependency: vec![2, 6] },
                        Dependency { dependency: vec![3, 6] },
                        Dependency { dependency: vec![4, 6] },
                        Dependency { dependency: vec![5, 6] },
                    ],
                    vec![
                        Dependency { dependency: vec![1, 3] },   // L<S
                        Dependency { dependency: vec![2, 4] },   // O<I
                        Dependency { dependency: vec![0, 6] },   // T must be last
                        Dependency { dependency: vec![1, 6] },
                        Dependency { dependency: vec![2, 6] },
                        Dependency { dependency: vec![3, 6] },
                        Dependency { dependency: vec![4, 6] },
                        Dependency { dependency: vec![5, 6] },
                    ],
                ],

            ])
    }

    pub fn tki() -> Opener {
        Opener::new(
            vec![vec![[
                Piece { piece_type: 0, rotation_state: 0, center: Point(1, 4), last_kick: 0 },
                Piece { piece_type: 1, rotation_state: 1, center: Point(1, 0), last_kick: 0 },
                Piece { piece_type: 2, rotation_state: 0, center: Point(0, 8), last_kick: 0 },
                Piece { piece_type: 3, rotation_state: 1, center: Point(1, 6), last_kick: 0 },
                Piece { piece_type: 4, rotation_state: 0, center: Point(0, 4), last_kick: 0 },
                Piece { piece_type: 5, rotation_state: 2, center: Point(3, 4), last_kick: 0 },
                Piece { piece_type: 6, rotation_state: 2, center: Point(1, 2), last_kick: 0 }]]],
            vec![vec![vec![
                Dependency{dependency: vec![4, 0, 6]},
                Dependency{dependency: vec![1,6]},
                Dependency{dependency: vec![2,6]}]]]
        )
    }
}
