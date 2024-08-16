use std::fmt::*;
use std::ops;

use crate::game::{COL, H, W};
use bitvec::prelude::*;
use tinyvec::ArrayVec;

#[derive(Clone, PartialEq, Eq, Copy, Debug)]
pub struct Bitmatrix {
    pub data: ArrayVec<[COL; W]>,
}

impl Display for Bitmatrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in (0..H).rev() {
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
            data: self
                .data
                .iter()
                .zip(rhs.data.iter())
                .map(|(x, y)| x.bitand(y))
                .collect(),
        }
    }
}

impl ops::BitXor for Bitmatrix {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self {
            data: self
                .data
                .into_iter()
                .zip(rhs.data.into_iter())
                .map(|(x, y)| x.bitxor(y))
                .collect(),
        }
    }
}

impl ops::BitOr for Bitmatrix {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            data: self
                .data
                .into_iter()
                .zip(rhs.data.into_iter())
                .map(|(x, y)| x.bitor(y))
                .collect(),
        }
    }
}

impl ops::Not for Bitmatrix {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self {
            data: self.data.into_iter().map(|x| x.not()).collect(),
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

    pub fn softdrop(&self) -> Self {
        // TODO
        self.dshift(1)
    }

    pub fn dshift(&self, n: usize) -> Self {
        Self {
            data: self.data.into_iter().map(|c| c >> n).collect(),
        }
    }

    pub fn ushift(&self, n: usize) -> Self {
        Self {
            data: self.data.into_iter().map(|c| c << n).collect(),
        }
    }

    pub fn lshift(&self, n: usize) -> Self {
        let mut data = self.data;
        data.rotate_left(n);
        Self { data }
    }

    pub fn rshift(&self, n: usize) -> Self {
        let mut data = self.data;
        data.rotate_right(n);
        Self { data }
    }
}

pub mod test {
    // use super::*;

    // pub fn test_shifting() {
    //     let mut b = Bitmatrix::new();
    //     b.data.set(0, true);
    //     b.data.set(15, true);
    //     b.data.set(96, true);
    //     b.data.set(111, true);
    //     b.data.set(144, true);
    //     b.data.set(159, true);
    //     println!("{}", b);
    //     println!("===================");
    //     println!("{}", b.lshift());
    //     println!("===================");
    //     println!("{}", b.rshift());
    //     println!("===================");
    //     println!("{}", b.ushift());
    //     println!("===================");
    //     println!("{}", b.dshift());
    //     println!("===================");
    // }
}
