use std::{
    fmt::Debug,
    ops::{BitAnd, BitOr},
};

use rand::seq::IndexedRandom as _;

#[derive(Debug, Clone, Copy)]
pub enum Number {
    N1 = 1,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
}

impl From<Number> for u8 {
    fn from(value: Number) -> Self {
        value as u8
    }
}

impl Number {
    pub fn to_bits(self) -> u16 {
        1 << u8::from(self)
    }
}

#[derive(Clone, Copy, Default)]
pub struct BitSet(u16);

impl Debug for BitSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Seen")
            .field(&format!("{:b}", self.0))
            .finish()
    }
}

impl BitSet {
    const ALL_NUMBERS: [Number; 9] = {
        use Number::*;
        [N1, N2, N3, N4, N5, N6, N7, N8, N9]
    };

    pub fn new() -> Self {
        Self(0)
    }

    pub fn clear(&mut self) {
        self.0 = 0;
    }

    pub fn get(&self, n: Number) -> bool {
        self.0 & n.to_bits() > 0
    }

    pub fn set(&mut self, n: Number) {
        self.0 |= n.to_bits();
    }

    pub fn unset(&mut self, n: Number) {
        self.0 &= !n.to_bits();
    }

    pub fn get_count(&self) -> u8 {
        let mut n = self.0;
        let mut sum = 0;
        for _ in 0..9 {
            sum += n & 0x1;
            n <<= 0x1;
        }
        sum as u8
    }

    pub fn all_seen(&self) -> bool {
        use Number::*;
        self.0
            == (N1.to_bits()
                | N2.to_bits()
                | N3.to_bits()
                | N4.to_bits()
                | N5.to_bits()
                | N6.to_bits()
                | N7.to_bits()
                | N8.to_bits()
                | N9.to_bits())
    }

    pub fn choose_new<R>(&self, rng: &mut R) -> Option<Number>
    where
        R: rand::Rng + ?Sized,
    {
        use Number::*;
        let mut arr = [N1; 9];
        let mut i = 0;
        for number in Self::ALL_NUMBERS {
            if !self.get(number) {
                arr[i] = number;
                i += 1;
            }
        }
        arr[0..i].choose(rng).cloned()
    }

    pub fn get_next(&self) -> Option<Number> {
        for n in Self::ALL_NUMBERS {
            if !self.get(n) {
                return Some(n);
            }
        }
        None
    }
}

impl BitAnd for BitSet {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitOr for BitSet {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}
