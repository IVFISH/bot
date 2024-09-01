use contracts::*;
use std::fmt::*;
use std::ops;

use crate::constants::*;
use bitvec::prelude::*;

#[derive(Clone, PartialEq, Eq, Copy, Debug)]
pub struct Bitmatrix {
    pub data: [COL; W],
}

impl Display for Bitmatrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in (0..H).rev() {
            for col in 0..W {
                if self.get(row, col) {
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

impl ops::BitAndAssign for Bitmatrix {
    /// Implements `self &= rhs`
    ///
    /// * `self`
    /// * `rhs`     - the thing to `and` with
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl ops::BitOrAssign for Bitmatrix {
    /// Implements `self |= rhs`
    ///
    /// * `self`
    /// * `rhs`     - the thing to `or` with
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl ops::BitXorAssign for Bitmatrix {
    /// Implements `self ^= rhs`
    ///
    /// * `self`
    /// * `rhs`     - the thing to `xor` with
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs;
    }
}

impl ops::BitAnd for Bitmatrix {
    type Output = Self;

    /// Implements `self & rhs`
    ///
    /// * `self`
    /// * `rhs`     - the thing to `and` with
    /// * `ret`     - self & rhs
    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            data: self.zip(rhs).map(|(x, y)| x.bitand(y)),
        }
    }
}

impl ops::BitOr for Bitmatrix {
    type Output = Self;

    /// Implements `self | rhs`
    ///
    /// * `self`
    /// * `rhs`     - the thing to `or` with
    /// * `ret`     - self | rhs
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            data: self.zip(rhs).map(|(x, y)| x.bitor(y)),
        }
    }
}

impl ops::BitXor for Bitmatrix {
    type Output = Self;

    /// Implements `self ^ rhs`
    ///
    /// * `self`
    /// * `rhs`     - the thing to `xor` with
    /// * `ret`     - self ^ rhs
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self {
            data: self.zip(rhs).map(|(x, y)| x.bitxor(y)),
        }
    }
}

impl ops::Not for Bitmatrix {
    type Output = Self;

    /// Implements `!self`
    ///
    /// * `self`
    /// * `ret`     - !self
    fn not(self) -> Self::Output {
        Self {
            data: self.data.map(|x| x.not()),
        }
    }
}

impl ops::Index<usize> for Bitmatrix {
    type Output = COL;

    /// Implements indexing into the underlying data.
    /// This is equivalent to `&self.data[index]`
    ///
    /// * `self`
    /// * `index`   - the column to index into
    /// * `ret`     - a inmutable reference to the column
    #[requires(index < W)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl ops::IndexMut<usize> for Bitmatrix {
    /// Implements indexing into the underlying data.
    /// This is equivalent to `&mut self.data[index]`
    ///
    /// * `self`
    /// * `index`   - the column to index into
    /// * `ret`     - a mutable reference to the column
    #[requires(index < W)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl Bitmatrix {
    /// Creates a new Bitmatrix of dimension ([`H`] x [`W`]).
    /// Zeros out all the entries.
    ///
    /// * `ret`     - the new bitmatrix
    #[ensures(ret.data.into_iter().all(|c| c == 0))]
    pub fn new() -> Self {
        Self {
            data: [0; W].into(),
        }
    }

    /// Provides accessing a particular element into the bitmatrix.
    ///
    /// * `r`   - starts at the bottom and goes up
    /// * `c`   - starts at the left and goes right
    /// * `ret` - true if the bit at (r, c) is 1
    #[requires(r < H, c < W)]
    pub fn get(&self, r: usize, c: usize) -> bool {
        (self[c] >> (COL::BITS as usize - r - 1) & 1) == 1
    }

    /// Provides setting a particular element in the bitmatrix to true.
    ///
    /// * `r`   - starts at the bottom and goes up
    /// * `c`   - starts at the left and goes right
    #[requires(r < H, c < W)]
    pub fn set(&mut self, r: usize, c: usize) {
        self[c] |= 1 << (COL::BITS as usize - r - 1)
    }

    /// Provides setting a particular element in the bitmatrix to false.
    ///
    /// * `r`   - starts at the bottom and goes up
    /// * `c`   - starts at the left and goes right
    #[requires(r < H, c < W)]
    pub fn unset(&mut self, r: usize, c: usize) {
        self[c] &= !(1 << (COL::BITS as usize - r - 1))
    }

    /// Checks for an empty board (where everything is zero)
    ///
    /// # Examples
    /// ```
    /// # use bot::bitmatrix::*;
    /// let bitmatrix = Bitmatrix::new();
    /// assert!(bitmatrix.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.data.into_iter().all(|x| x == 0)
    }

    /// Provides iterator for the positions
    /// in the bitmatrix that is a `1`.
    /// That is, `self.get(r, c)` iff `(c * H) + r` is in the iterator.
    ///
    /// * `ret`     - the iterator
    #[ensures(ret.is_sorted())]
    #[ensures(ret.clone().all(|x| self.get(x % H, x / H)))]
    #[ensures((0..W).map(|c| (0..H).filter(|&r| self.get(r, c)).all(|r| ret.clone().collect::<Vec<_>>().contains(&(c * H + r)))).all(|x| x))]
    pub fn iter_ones<'a>(&'a self) -> impl Iterator<Item = usize> + 'a {
        self.data.as_bits::<Msb0>().iter_ones()
    }


    /// Reduces the bitmatrix to find the number of `1` bits.
    ///
    /// * `ret`     - the count of ones
    #[ensures(ret == self.data.map(|x| x.count_ones() as usize).into_iter().sum())]
    pub fn count_ones(&self) -> usize {
        self.data.as_bits::<Msb0>().count_ones()
    }

    #[requires(dr.abs() < H as i32, dc.abs() < W as i32)]
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

    #[requires((c | *self) == c)]
    #[ensures((c | ret) == c)]
    pub fn softdrop(&self, c: Self) -> Self {
        Self {
            data: self.zip(c).map(|(r, c)| {
                // find the first 0 to the left of each target position
                // this is done by finding which bits change from 0 to 1 when r is added to c
                let (c, r) = (c >> 1, r >> 1);
                (c + r) & !c
            }),
        }
    }

    #[requires(re.count_ones() <= 4)]
    pub fn clearrows(&self, re: COL) -> Self {
        if re == 0 {
            return *self;
        }

        let l = re.leading_zeros();
        let fc = (re << l).leading_ones(); // # lines in the First Clear group
        let ones = re.count_ones(); // # lines cleared

        let mbot = !(COL::MAX << l >> l); // no shift
        let mtop = (re - 1) & !re; // full shift
        let mmid = !(mbot | mtop | re); // "half" (fc) shift

        Self {
            data: self
                .data
                .map(|c| (c & mbot) | ((c & mmid) << fc) | ((c & mtop) << ones)),
        }
    }

    #[ensures((ret | *self) == *self)]
    pub fn grounded(&self) -> Self {
        *self & !self.ushift(1)
    }

    #[requires(n < H)]
    #[ensures((0..W).all(|c| (0..n).all(|r| ret.get(H - 1 - r, c) == false)))]
    pub fn dshift(&self, n: usize) -> Self {
        Self {
            data: self.data.map(|c| c << n),
        }
    }

    #[requires(n < H)]
    #[ensures((0..W).all(|c| (0..n).all(|r| ret.get(r, c) == false)))]
    pub fn ushift(&self, n: usize) -> Self {
        Self {
            data: self.data.map(|c| c >> n),
        }
    }

    #[requires(n < W)]
    #[ensures((0..n).all(|c| ret[W - 1 - c] == 0))]
    pub fn lshift(&self, n: usize) -> Self {
        let mut data = self.data;
        for i in 0..n {
            data[i] = 0;
        }
        data.rotate_left(n);
        Self { data }
    }

    #[requires(n < W)]
    #[ensures((0..n).all(|c| ret[c] == 0))]
    pub fn rshift(&self, n: usize) -> Self {
        let mut data = self.data;
        for i in 1..=n {
            data[W - i] = 0;
        }
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
    use crate::constants::COL;
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
