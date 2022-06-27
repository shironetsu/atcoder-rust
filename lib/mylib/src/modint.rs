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
#[derive(Clone, Copy)]
pub struct ModInt {
    val: i64,
}

// #[snippet("modint-alias")]
// #[snippet(include="modint")]
// type ModInt = ModInt;

#[snippet("modint-impl")]
impl ModInt {
    //const MODULO: i64 = 1_000_000_007;
    const MODULO: i64 = 998_244_353;

    pub fn new() -> ModInt {
        ModInt { val: 0 }
    }

    pub fn from(x: i64) -> ModInt {
        let val = x.rem_euclid(Self::MODULO);
        ModInt { val }
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
