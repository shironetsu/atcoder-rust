use cargo_snippet::snippet;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[snippet("modint")]
#[snippet(prefix = "use std::ops::{Add, Sub, Mul, AddAssign, SubAssign, MulAssign};")]
#[snippet(include = "modint-impl")]
#[snippet(include = "modint-add")]
#[snippet(include = "modint-add-i64")]
#[snippet(include = "modint-add-assign")]
#[snippet(include = "modint-sub")]
#[snippet(include = "modint-sub-i64")]
#[snippet(include = "modint-sub-assign")]
#[snippet(include = "modint-mul")]
#[snippet(include = "modint-mul-i64")]
#[snippet(include = "modint-mul-assign")]
#[snippet(include = "modint-inv-impl")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ModInt {
    val: i64,
}

// #[snippet("modint-alias")]
// #[snippet(include="modint")]
// type ModInt = ModInt;

#[snippet("modint-impl")]
impl ModInt {
    //const MODULO: i64 = 1_000_000_007;
    pub const MODULO: i64 = 998_244_353;

    pub fn new() -> ModInt {
        ModInt { val: 0 }
    }

    pub fn from(x: i64) -> ModInt {
        let val = x.rem_euclid(Self::MODULO);
        ModInt { val }
    }

    pub fn binom(n: i64, k: i64) -> ModInt {
        let mut val = ModInt::from(1);
        for i in 1..=k {
            val *= ModInt::from(n - i + 1) * ModInt::from(i).inv()
        }
        val
    }

    pub fn pow(&self, n: i64) -> ModInt {
        if n == 0 {
            return ModInt::from(1);
        }
        let (a, n) = if n > 0 {
            (self.clone(), n)
        } else {
            (self.inv(), -n)
        };
        let mut k = 0;
        while n >= 1 << k {
            k += 1;
        }
        let mut x = ModInt::from(1);
        for i in (0..k).rev() {
            x = if (n >> i) & 1 == 1 { x * x * a } else { x * x };
        }
        x
    }
}

#[snippet("modint-add")]
impl Add for ModInt {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let val = (self.val + rhs.val).rem_euclid(Self::MODULO);
        ModInt { val }
    }
}

#[snippet("modint-add-i64")]
impl Add<i64> for ModInt {
    type Output = Self;
    fn add(self, rhs: i64) -> Self {
        let val = (self.val + rhs).rem_euclid(Self::MODULO);
        ModInt { val }
    }
}

#[snippet("modint-add-assign")]
impl AddAssign for ModInt {
    fn add_assign(&mut self, rhs: Self) {
        self.val = (self.val + rhs.val).rem_euclid(Self::MODULO);
    }
}

#[snippet("modint-sub")]
impl Sub for ModInt {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let val = (self.val - rhs.val).rem_euclid(Self::MODULO);
        ModInt { val }
    }
}

#[snippet("modint-sub-i64")]
impl Sub<i64> for ModInt {
    type Output = Self;
    fn sub(self, rhs: i64) -> Self {
        let val = (self.val - rhs).rem_euclid(Self::MODULO);
        ModInt { val }
    }
}

#[snippet("modint-sub-assign")]
impl SubAssign for ModInt {
    fn sub_assign(&mut self, rhs: Self) {
        self.val = (self.val - rhs.val).rem_euclid(Self::MODULO);
    }
}

#[snippet("modint-mul")]
impl Mul for ModInt {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let val = (self.val * rhs.val).rem_euclid(Self::MODULO);
        ModInt { val }
    }
}

#[snippet("modint-mul-i64")]
impl Mul<i64> for ModInt {
    type Output = Self;
    fn mul(self, rhs: i64) -> Self {
        let val = (self.val * rhs).rem_euclid(Self::MODULO);
        ModInt { val }
    }
}

#[snippet("modint-mul-assign")]
impl MulAssign for ModInt {
    fn mul_assign(&mut self, rhs: Self) {
        self.val = (self.val * rhs.val).rem_euclid(Self::MODULO);
    }
}

#[snippet("modint-inv-trait")]
pub trait Inv {
    fn inv(self) -> Self;
}

#[snippet("modint-inv-impl")]
#[snippet(include = "modint-inv-trait")]
#[snippet(include = "gcd-internal")]
impl Inv for ModInt {
    fn inv(self) -> Self {
        let mut x = 0;
        let mut y = 0;
        gcd(self.val, Self::MODULO, &mut x, &mut y);
        ModInt {
            val: x.rem_euclid(Self::MODULO),
        }
    }
}

#[snippet("gcd-internal")]
pub fn gcd(a: i64, b: i64, x: &mut i64, y: &mut i64) -> i64 {
    if a != 0 {
        let mut d = gcd(b % a, a, x, y);
        *y -= (b / a) * *x;
        std::mem::swap(x, y);
        if d < 0 {
            d = -d;
            *x = -*x;
            *y = -*y;
        }
        d
    } else {
        *x = 0;
        *y = 1;
        b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let x = ModInt { val: 1 };
        assert_eq!(x.inv(), x);
    }

    #[test]
    fn it_works_2() {
        let x = ModInt { val: 4 };
        let y = x.inv();
        assert_eq!(x * y, ModInt { val: 1 });
    }

    #[test]
    fn pow_works() {
        let x = ModInt { val: 2 };
        assert_eq!(x.pow(1), x);
    }

    #[test]
    fn pow_works_2() {
        let x = ModInt { val: 2 };
        assert_eq!(x.pow(2), x * x);
    }

    #[test]
    fn pow_works_5() {
        let x = ModInt { val: 2 };
        assert_eq!(x.pow(5), x * x * x * x * x);
    }

    #[test]
    fn pow_works_6() {
        let x = ModInt { val: 2 };
        assert_eq!(x.pow(ModInt::MODULO), x);
    }
}
