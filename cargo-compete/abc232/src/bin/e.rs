#![allow(unused_imports)]
#![allow(non_snake_case)]
use maplit::*;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
use std::fmt::Write;

#[fastout]
fn main() {
    input! {
        H: usize,
        W: usize,
        K: usize,
        x1: Usize1,
        y1: Usize1,
        x2: Usize1,
        y2: Usize1,
    }

    let H = Mint::from(H as i64);
    let W = Mint::from(W as i64);
    let mut a = Mint::from(1);
    let mut b = Mint::new();
    let mut c = Mint::new();
    let mut d = Mint::new();

    for _ in 0..K {
        let (a1, b1, c1, d1) = (
            (H - 1) * c + (W - 1) * b,
            a + (W - 2) * b + (H - 1) * d,
            a + (H - 2) * c + (W - 1) * d,
            b + c + (H + W - 4) * d,
        );
        a = a1;
        b = b1;
        c = c1;
        d = d1;
    }

    let ans = if (x1, y1) == (x2, y2) {
        a
    } else if x1 == x2 {
        b
    } else if y1 == y2 {
        c
    } else {
        d
    };

    println!("{}", ans.val);
}

//________________________________________________________________________________
//

use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};
impl Sub<i64> for ModInt998244353 {
    type Output = Self;
    fn sub(self, rhs: i64) -> Self {
        let val = (self.val - rhs).rem_euclid(Self::MODULO);
        ModInt998244353 { val }
    }
}
impl SubAssign for ModInt998244353 {
    fn sub_assign(&mut self, rhs: Self) {
        self.val = (self.val - rhs.val).rem_euclid(Self::MODULO);
    }
}
impl Sub for ModInt998244353 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let val = (self.val - rhs.val).rem_euclid(Self::MODULO);
        ModInt998244353 { val }
    }
}
impl Mul<i64> for ModInt998244353 {
    type Output = Self;
    fn mul(self, rhs: i64) -> Self {
        let val = (self.val * rhs).rem_euclid(Self::MODULO);
        ModInt998244353 { val }
    }
}
impl MulAssign for ModInt998244353 {
    fn mul_assign(&mut self, rhs: Self) {
        self.val = (self.val * rhs.val).rem_euclid(Self::MODULO);
    }
}
impl Mul for ModInt998244353 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let val = (self.val * rhs.val).rem_euclid(Self::MODULO);
        ModInt998244353 { val }
    }
}
impl ModInt998244353 {
    const MODULO: i64 = 998244353;
    pub fn new() -> ModInt998244353 {
        ModInt998244353 { val: 0 }
    }
    pub fn from(x: i64) -> ModInt998244353 {
        let val = x.rem_euclid(Self::MODULO);
        ModInt998244353 { val }
    }
}
impl Add<i64> for ModInt998244353 {
    type Output = Self;
    fn add(self, rhs: i64) -> Self {
        let val = (self.val + rhs).rem_euclid(Self::MODULO);
        ModInt998244353 { val }
    }
}
impl AddAssign for ModInt998244353 {
    fn add_assign(&mut self, rhs: Self) {
        self.val = (self.val + rhs.val).rem_euclid(Self::MODULO);
    }
}
impl Add for ModInt998244353 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let val = (self.val + rhs.val).rem_euclid(Self::MODULO);
        ModInt998244353 { val }
    }
}
#[derive(Clone, Copy)]
pub struct ModInt998244353 {
    val: i64,
}

type Mint = ModInt998244353;
