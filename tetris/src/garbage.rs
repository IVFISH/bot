#![allow(dead_code)]

use crate::constants::board_constants::*;
use rand::Rng;

#[derive(PartialEq, Clone)]
pub struct GarbageItem {
    pub amt: usize,
    pub col: usize,
}

impl GarbageItem {
    pub fn new(amt: usize) -> Self {
        Self {
            amt,
            col: rand::thread_rng().gen_range(0..BOARD_WIDTH),
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
