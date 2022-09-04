pub struct Piece {

}

#[cfg(test)]
mod piece_tests {
    use super::*;

    #[test]
    fn test_abs_locations() {
        // TODO: FIX TESTS WITH OFFSETS LATER

        let mut piece = create_preset_t();
        if let Ok(locations) = piece.abs_locations() {
            assert!(locations.contains(&Point { row: 2, col: 2 }));
            assert!(locations.contains(&Point { row: 3, col: 2 }));
            assert!(locations.contains(&Point { row: 2, col: 3 }));

            assert!(!locations.contains(&Point { row: 2, col: 4 }));
        } else {
            assert!(false)
        }

        let mut piece = create_preset_i();
        if let Ok(locations) = piece.abs_locations() {
            assert!(locations.contains(&Point { row: 4, col: 6 }));
            assert!(!locations.contains(&Point { row: 2, col: 4 }));
        } else {
            assert!(false)
        }

        let mut piece = Placement {
            piece_type: 0,
            rotation_state: 0,
            center: Point { row: 0, col: 0 },
            last_kick: 0,
        };

        assert!(piece.abs_locations().is_err());
    }

    #[test]
    fn test_rotate() {
        let mut piece = create_preset_i();
        piece.rotate(1);

        let locations = piece.abs_locations();

        assert_eq!(piece.rotation_state, 3);

        piece.rotate(2);

        assert_eq!(piece.rotation_state, 1);
    }

    #[test]
    fn test_move() {
        let mut piece = create_preset_s();
    }

    #[test]
    fn test_negative_center() {
        let piece = Placement {
            piece_type: 2,
            rotation_state: 3,
            center: Point { row: 0, col: 0 },
            last_kick: 0,
        };

        assert!(piece.abs_locations().is_err());
    }

    fn create_preset_i() -> Placement {
        Placement {
            piece_type: 4,
            rotation_state: 2,
            center: Point { row: 4, col: 6 },
            last_kick: 0,
        }
    }

    fn create_preset_s() -> Placement {
        Placement {
            piece_type: 3,
            rotation_state: 1,
            center: Point { row: 15, col: 5 },
            last_kick: 0,
        }
    }

    fn create_preset_t() -> Placement {
        Placement {
            piece_type: 6,
            rotation_state: 0,
            center: Point { row: 2, col: 2 },
            last_kick: 0,
        }
    }
}
