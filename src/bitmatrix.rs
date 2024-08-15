use std::fmt::*;

use crate::game::{COL, H, W};
use arrayvec::*;
use bitvec::prelude::*;

/// I can't figure out how to use num_traits::PrimInt to make Bitmatrix generic :(
// pub const W: usize = 10;
// pub const H: usize = COL::BITS as usize;

/// column major storage of bit-boards
/// warning: BitVec uses little endian data storage -- very misleading sometimes

// TODO: compare derived equal with (a ^ b == 0)
#[derive(Clone, PartialEq, Eq)]
pub struct Bitmatrix {
    pub data: ArrayVec<COL, W>,
}

impl Display for Bitmatrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // let iter = self.data.chunks_exact(H).collect::<Vec<_>>();
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

impl Bitmatrix {
    pub fn new() -> Self {
        Self {
            data: ArrayVec::new_const(),
        }
    }

    pub fn iter_ones<'a>(&'a self) -> impl Iterator<Item = usize> + 'a {
        self.data.as_bits::<Lsb0>().iter_ones()
    }

    pub fn count_ones(&self) -> usize {
        self.data.as_bits::<Lsb0>().count_ones()
    }

    pub fn not(&self) -> Self {
        Self {
            data: self.data.iter().map(|&c| !c).collect(),
        }
    }

    pub fn dshift(&self) -> Self {
        Self {
            data: self.data.iter().map(|&c| c >> 1).collect(),
        }
    }

    pub fn ushift(&self) -> Self {
        Self {
            data: self.data.iter().map(|&c| c << 1).collect(),
        }
    }

    pub fn lshift(&self) -> Self {
        let mut data = self.data.clone();
        data.rotate_left(1);
        Self { data }
    }

    pub fn rshift(&self) -> Self {
        let mut data = self.data.clone();
        data.rotate_right(1);
        Self { data }
    }

    pub fn or(&self, other: Bitmatrix) -> Self {
        Self {
            data: self
                .data
                .iter()
                .zip(other.data.iter())
                .map(|(x, y)| x | y)
                .collect(),
        }
    }

    pub fn and(&self, other: Bitmatrix) -> Self {
        Self {
            data: self
                .data
                .iter()
                .zip(other.data.iter())
                .map(|(x, y)| x & y)
                .collect(),
        }
    }
}

pub mod test {
    use super::*;

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
