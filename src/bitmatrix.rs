use std::fmt::*;

use crate::game::{COL, H, W};
use bitvec::prelude::*;

/// I can't figure out how to use num_traits::PrimInt to make Bitmatrix generic :(
// pub const W: usize = 10;
// pub const H: usize = COL::BITS as usize;

/// column major storage of bit-boards

#[derive(Clone)]
pub struct Bitmatrix {
    pub data: BitVec<COL>,
}

impl Display for Bitmatrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let iter = self.data.chunks_exact(H).collect::<Vec<_>>();
        for row in (0..H).rev() {
            for col in 0..W {
                if iter[col][row] {
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
            data: BitVec::repeat(false, W * H),
        }
    }

    pub fn not(&self) -> Self {
        Self {
            data: !self.data.clone(),
        }
    }

    pub fn dshift(&self) -> Self {
        let mut data = self.data[1..].to_bitvec();
        data.push(false);

        for c in 0..W {
            data.set(c * H + 15, false);
        }

        Self { data }
    }

    pub fn ushift(&self) -> Self {
        // let mut data = bitvec![0];
        // data.extend(self.data[..(W *  H - 1)]);
        // Self { data }

        let mut data = self.data.clone();
        data.shift_right(1);

        for c in 0..W {
            data.set(c * H, false);
        }

        Self { data }
    }

    pub fn lshift(&self) -> Self {
        // let mut data = self.data[H..].to_bitvec();
        // data.push(false);
        // Self { data }
        let mut data = self.data.clone();
        data.shift_left(H);
        Self { data }
    }

    pub fn rshift(&self) -> Self {
        let mut data = self.data.clone();
        data.shift_right(H);
        Self { data }
    }

    pub fn or(&self, other: Bitmatrix) -> Self {
        Self {
            data: other.data | self.data.clone(),
        }
    }

    pub fn and(&self, other: Bitmatrix) -> Self {
        Self {
            data: other.data & self.data.clone(),
        }
    }
}

pub mod test {
    use super::*;

    pub fn test_shifting() {
        let mut b = Bitmatrix::new();
        b.data.set(0, true);
        b.data.set(15, true);
        b.data.set(96, true);
        b.data.set(111, true);
        b.data.set(144, true);
        b.data.set(159, true);
        println!("{}", b);
        println!("===================");
        println!("{}", b.lshift());
        println!("===================");
        println!("{}", b.rshift());
        println!("===================");
        println!("{}", b.ushift());
        println!("===================");
        println!("{}", b.dshift());
        println!("===================");
    }
}
