

#[cfg(test)]
mod piece_queue_tests {
    use super::*;

    #[test]
    fn test_seven_bag() {
        let mut queue = PieceQueue::default();
        let bag = queue.shuffle_seven([0, 1, 2, 3, 4, 5, 6]);

        for piece in 0..7 {
            assert!(bag.contains(&piece));
        }

        for _ in 0..10 {
            let new_bag = queue.shuffle_seven([0, 1, 2, 3, 4, 5, 6]);
            assert_ne!(bag, new_bag);
        }
    }

    #[test]
    fn test_match_with_osk() {
        let mut queue = PieceQueue::new(Some(15));
        // ITOSLJZS JOTZLIL
        let osk_queue = [4, 6, 2, 3, 1, 5, 0, 3, 5, 2, 6, 0, 1, 4];

        for piece in osk_queue {
            assert_eq!(queue.next(), piece);
        }

        let mut queue = PieceQueue::new(Some(7000));
        // TSJOLIZ ITLSJZO
        let osk_queue = [6, 3, 5, 2, 1, 4, 0, 4, 6, 1, 3, 5, 0, 2];

        for piece in osk_queue {
            assert_eq!(queue.next(), piece);
        }
    }
}

#[cfg(test)]
mod garbage_item_test {
    use super::*;

    #[test]
    fn test_random() {
        const N: usize = 10;
        let mut arr = vec![];
        for _ in 0..N {
            arr.push(GarbageItem::new(1));
        }

        // checks that not everything is the same
        assert!(!arr.windows(2).all(|w| w[0] == w[1]));
    }
}
