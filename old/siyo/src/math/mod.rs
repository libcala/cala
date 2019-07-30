//! # Getting Started
//! ```
//! // TODO
//! ```

use std::fmt::*;
use std::ops::*;

/// A fraction 32-bit.
pub struct Fr32(pub i16, pub u16);

impl Fr32 {
    /// Create a new fraction.
    pub fn new(num: i16, den: u16) -> Fr32 {
        Fr32(num, den)
    }

    //    /// TODO Simplify the fraction.
    //    pub fn simplify() {
    //    }
}

impl Div<i16> for Fr32 {
    type Output = Fr32;

    fn div(mut self, mut other: i16) -> Self::Output {
        if other.is_negative() {
            self.0 = -self.0;
            other = -other;
        }
        Fr32(self.0, self.1 * (other as u16))
    }
}

impl Mul<i16> for Fr32 {
    type Output = Fr32;

    fn mul(mut self, mut other: i16) -> Self::Output {
        if other.is_negative() {
            self.0 = -self.0;
            other = -other;
        }
        Fr32(self.0 * (other as i16), self.1)
    }
}

/// A fraction 64-bit.
pub struct Fr64(pub i32, pub u32);

impl Fr64 {
    /// Create a new fraction.
    pub fn new(num: i32, den: u32) -> Fr64 {
        Fr64(num, den)
    }

    //    /// TODO Simplify the fraction.
    //    pub fn simplify() {
    //    }
}

impl Div<i32> for Fr64 {
    type Output = Fr64;

    fn div(mut self, mut other: i32) -> Self::Output {
        if other.is_negative() {
            self.0 = -self.0;
            other = -other;
        }
        Fr64(self.0, self.1 * (other as u32))
    }
}

impl Mul<i32> for Fr64 {
    type Output = Fr64;

    fn mul(mut self, mut other: i32) -> Self::Output {
        if other.is_negative() {
            self.0 = -self.0;
            other = -other;
        }
        Fr64(self.0 * (other as i32), self.1)
    }
}

impl Display for Fr64 {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}/{}", self.0, self.1)
    }
}

/// Fixed-Point 32-Bit.
pub struct Fp32(i32);

impl Fp32 {
    /// Create a new fixed point number.
    pub fn new(int: i16, frac: (u16, u16)) -> Self {
        let num = (frac.0 as u32) << 16;
        let dec = num / (frac.1 as u32);

        Fp32(if int.is_negative() {
            ((int as i32) << 16) - (dec as i32)
        } else {
            ((int as i32) << 16) + (dec as i32)
        })
    }

    /// Get the integer portion of the fixed point number.
    pub fn int(&self) -> u16 {
        (self.0 >> 16) as u16
    }

    /// Get the fractional portion of the fixed point number.
    pub fn frac(&self) -> u16 {
        self.0 as u16
    }
}

/// Fixed-Point 64-Bit.
pub struct Fp64(i64);

impl Fp64 {
    /// Create a new fixed point number.
    pub fn new(int: i32, frac: (u32, u32)) -> Self {
        let num = (frac.0 as u64) << 32;
        let dec = num / (frac.1 as u64);

        Fp64(if int.is_negative() {
            ((int as i64) << 32) - (dec as i64)
        } else {
            ((int as i64) << 32) + (dec as i64)
        })
    }

    /// Get the integer portion of the fixed point number.
    pub fn int(&self) -> u32 {
        (self.0 >> 32) as u32
    }

    /// Get the fractional portion of the fixed point number.
    pub fn frac(&self) -> u32 {
        self.0 as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_fp32() {
        let one_and_half = Fp32::new(1, (1, 2));
        assert_eq!(one_and_half.int(), 1);
        assert_eq!(one_and_half.frac(), 1 << 15);
    }

    #[test]
    fn build_fp64() {
        let one_and_half = Fp64::new(1, (1, 2));
        assert_eq!(one_and_half.int(), 1);
        assert_eq!(one_and_half.frac(), 1 << 31);
    }
}
