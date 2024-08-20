use std::fmt::*;
use std::ops;

use crate::game::{COL, H, W};
use bitvec::prelude::*;

#[derive(Clone, PartialEq, Eq, Copy, Debug)]
pub struct Bitmatrix {
    pub data: [COL; W],
}

impl Display for Bitmatrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in 0..H {
            for col in 0..W {
                if self.data[col] >> row & 1 == 1 {
                    write!(f, "■ ")?
                } else {
                    write!(f, "□ ")?
                }
            }
            write!(f, "\n")?
        }
        Ok(())
    }
}

impl ops::BitOrAssign for Bitmatrix {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl ops::BitAndAssign for Bitmatrix {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl ops::BitXorAssign for Bitmatrix {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs;
    }
}

impl ops::BitAnd for Bitmatrix {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            data: self.zip(rhs).map(|(x, y)| x.bitand(y)),
        }
    }
}

impl ops::BitXor for Bitmatrix {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self {
            data: self.zip(rhs).map(|(x, y)| x.bitxor(y)),
        }
    }
}

impl ops::BitOr for Bitmatrix {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            data: self.zip(rhs).map(|(x, y)| x.bitor(y)),
        }
    }
}

impl ops::Not for Bitmatrix {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self {
            data: self.data.map(|x| x.not()),
        }
    }
}

impl ops::Index<usize> for Bitmatrix {
    type Output = COL;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl ops::IndexMut<usize> for Bitmatrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl Bitmatrix {
    pub fn new() -> Self {
        Self {
            data: [0; W].into(),
        }
    }

    pub fn iter_ones<'a>(&'a self) -> impl Iterator<Item = usize> + 'a {
        self.data.as_bits::<Lsb0>().iter_ones()
    }

    pub fn count_ones(&self) -> usize {
        self.data.as_bits::<Lsb0>().count_ones()
    }

    pub fn shift(&self, dr: i32, dc: i32) -> Self {
        let mut ret = *self;

        if dr > 0 {
            ret = ret.ushift(dr as usize);
        }
        if dr < 0 {
            ret = ret.dshift(-dr as usize)
        }
        if dc > 0 {
            ret = ret.rshift(dc as usize)
        }
        if dc < 0 {
            ret = ret.lshift(-dc as usize)
        }

        ret
    }

    pub fn softdrop(&self, c: Self) -> Self {
        // MUST have c|r == c
        Self {
            data: self.zip(c).map(|(r, c)| {
                // find the first 0 to the left of each target position
                // this is done by finding which bits change from 0 to 1 when r is added to c
                let (c, r) = (c >> 1, r >> 1);
                (c + r) & !c
            }),
        }
    }

    pub fn clearrows(&self, re: COL) -> Self {
        // PRECONDITION: re can have no more than 2 groups of 1s
        // re: a COL where 1s represent rows to clear
        if re == 0 {
            return *self;
        }

        let l = re.leading_zeros();
        let fc = (re << l).leading_ones(); // # lines in the First Clear group
        let ones = re.count_ones(); // num lines cleared

        let mbot = !(COL::MAX << l >> l); // no shift
        let mtop = (re - 1) & !re; // full shift
        let mmid = !(mbot | mtop | re); // "half" (fc) shift

        Self {
            data: self
                .data
                .map(|c| (c & mbot) | ((c & mmid) << fc) | ((c & mtop) << ones)),
        }
    }

    pub fn grounded(&self) -> Self {
        Self {
            data: self.data.map(|c| c & !(c >> 1)),
        }
    }

    pub fn dshift(&self, n: usize) -> Self {
        Self {
            data: self.data.map(|c| c << n),
        }
    }

    pub fn ushift(&self, n: usize) -> Self {
        Self {
            data: self.data.map(|c| c >> n),
        }
    }

    pub fn lshift(&self, n: usize) -> Self {
        let mut data = self.data;
        data[0] = 0;
        data.rotate_left(n);
        Self { data }
    }

    pub fn rshift(&self, n: usize) -> Self {
        let mut data = self.data;
        data[W - 1] = 0;
        data.rotate_right(n);
        Self { data }
    }

    // CODE TAKEN FROM [#79451]
    pub fn zip(self, rhs: Self) -> [(COL, COL); W] {
        use core::mem::MaybeUninit;

        let mut dst = MaybeUninit::uninit_array::<W>();
        for (i, (lhs, rhs)) in self.data.into_iter().zip(rhs.data.into_iter()).enumerate() {
            dst[i].write((lhs, rhs));
        }

        // SAFETY: At this point we've properly initialized the whole array
        // and we just need to cast it to the correct type.
        unsafe { core::mem::transmute::<[MaybeUninit<(COL, COL)>; W], [(COL, COL); W]>(dst) }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::game::COL;
    use crate::test_api::test_api::*;

    #[test]
    fn lineclear() {
        // NOTE while these test cases only clear full lines,
        // the function clearrows does not require full lines :)
        // it shouldn't make a difference, but feel free to add more tests
        #[rustfmt::skip]
        let board_str = [
            "oooo.ooooo",
            "ooooxooooo", // clear
            "ooo.xooooo",
            "ooooxooooo", // clear
            "ooooxooooo", // clear
            "ooo.oooooo"
        ];

        #[rustfmt::skip]
        let sol_str = [
            "oooo.ooooo",
            "ooo.xooooo",
            "ooo.oooooo",
        ];

        let board = bitmatrix_from_string(&board_str);
        let sol = bitmatrix_from_string(&sol_str);
        // TODO this will become a function somewhere, use it!
        let to_clear: COL = board.data.iter().fold(COL::MAX, |a, n| a & n);

        assert_eq!(board.clearrows(to_clear), sol);
        assert_eq!(board.clearrows(0), board);

        #[rustfmt::skip]
        let board_str = [
            "o..ooooooo",
            "o...oooooo",
            "oooxoooooo", // clear
            "ooxx.ooooo",
            "oooxoooooo", // clear
        ];

        #[rustfmt::skip]
        let sol_str = [
            "o..ooooooo",
            "o...oooooo",
            "ooxx.ooooo",
        ];

        let board = bitmatrix_from_string(&board_str);
        let sol = bitmatrix_from_string(&sol_str);
        // TODO this will become a function somewhere, use it!
        let to_clear: COL = board.data.iter().fold(!0, |a, n| a & n);

        assert_eq!(board.clearrows(to_clear), sol);

        #[rustfmt::skip]
        let board_str = [
            "ooo..ooooo",
            "oooxxooooo",
            "oooxx.oooo",
        ];

        #[rustfmt::skip]
        let sol_str = [
            "ooo..ooooo",
            "oooxx.oooo",
        ];

        let board = bitmatrix_from_string(&board_str);
        let sol = bitmatrix_from_string(&sol_str);
        // TODO this will become a function somewhere, use it!
        let to_clear: COL = board.data.iter().fold(!0, |a, n| a & n);

        assert_eq!(board.clearrows(to_clear), sol);
    }

    #[test]
    fn grounded() {
        #[rustfmt::skip]
        let collision_str = [
            "xx.xxx....",
            "x.x..x.xxx",
            "xxxxxxx...",
            ".xxx.x..x.",
            "..xxx...x.",
        ];

        #[rustfmt::skip]
        let sol_str = [
            ".x.xx.....",
            ".......xxx",
            "x...x.x...",
            ".x...x....",
            "..xxx...x.",
        ];

        let collision = bitmatrix_from_string(&collision_str);
        let sol = bitmatrix_from_string(&sol_str);

        assert_eq!(collision.grounded(), sol);
    }
}
