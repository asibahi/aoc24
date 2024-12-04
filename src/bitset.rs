use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Shl, ShlAssign, Shr, ShrAssign};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Bitset([u64; 3]);

impl Bitset {
    pub const BITS: u32 = u64::BITS * 3;

    #[inline]
    pub fn new() -> Self {
        Self([0; 3])
    }

    #[inline]
    pub fn count_ones(self) -> u32 {
        self.0[0].count_ones() + self.0[1].count_ones() + self.0[2].count_ones()
    }

    #[inline]
    pub fn count_zeros(self) -> u32 {
        self.0[0].count_zeros() + self.0[1].count_zeros() + self.0[2].count_zeros()
    }

    #[inline]
    pub fn push_bit(self, bit: bool) -> Self {
        self << 1 | bit
    }
}
impl Default for Bitset {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl BitOr<bool> for Bitset {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: bool) -> Self::Output {
        Self([self.0[0], self.0[1], self.0[2] | rhs as u64])
    }
}
impl BitOrAssign<bool> for Bitset {
    fn bitor_assign(&mut self, rhs: bool) {
        *self = *self | rhs
    }
}

impl BitOr<Bitset> for Bitset {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self([
            self.0[0] | rhs.0[0],
            self.0[1] | rhs.0[1],
            self.0[2] | rhs.0[2],
        ])
    }
}
impl BitOrAssign<Bitset> for Bitset {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs
    }
}

impl BitAnd<Bitset> for Bitset {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        Self([
            self.0[0] & rhs.0[0],
            self.0[1] & rhs.0[1],
            self.0[2] & rhs.0[2],
        ])
    }
}
impl BitAndAssign<Bitset> for Bitset {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs
    }
}

impl Shl<u32> for Bitset {
    type Output = Self;

    #[inline]
    fn shl(self, rhs: u32) -> Self::Output {
        // incorrect really but this is unlikely to be used for larger numbers anyway
        debug_assert!(rhs <= u64::BITS);
        Self([
            self.0[0] << rhs | self.0[1] >> (u64::BITS - rhs),
            self.0[1] << rhs | self.0[2] >> (u64::BITS - rhs),
            self.0[2] << rhs,
        ])
    }
}
impl ShlAssign<u32> for Bitset {
    fn shl_assign(&mut self, rhs: u32) {
        *self = *self << rhs;
    }
}

impl Shr<u32> for Bitset {
    type Output = Self;

    #[inline]
    fn shr(self, rhs: u32) -> Self::Output {
        // incorrect really but this is unlikely to be used for larger numbers anyway
        debug_assert!(rhs <= u64::BITS);
        Self([
            self.0[0] >> rhs,
            self.0[1] >> rhs | self.0[0] << (u64::BITS - rhs),
            self.0[2] >> rhs | self.0[1] << (u64::BITS - rhs),
        ])
    }
}
impl ShrAssign<u32> for Bitset {
    fn shr_assign(&mut self, rhs: u32) {
        *self = *self >> rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bitset_test() {
        let bs = Bitset([0, 0, 2]);
        let rs = bs << 1;
        assert_eq!(rs, Bitset([0, 0, 4]));
        assert_eq!(rs >> 1, bs);

        let rs = bs << 63;
        assert_eq!(rs, Bitset([0, 1, 0]));
        assert_eq!(rs >> 63, bs);

        let es = Bitset::default();
        let rs = es.push_bit(true);
        assert_eq!(rs, Bitset([0, 0, 1]));
    }
}
