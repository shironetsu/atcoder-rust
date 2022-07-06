#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::*;
use maplit::*;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
use std::fmt::Write;
use superslice::{Ext, Ext2};

#[fastout]
fn main() {
    input! {
        N: i64,
        L: i64,
    }

    let mut binom = ModInt::from(1);
    let mut m = 1;
    let mut ans = ModInt::new();
    let mut k = 0;
    while N >= L * k {
        let m = N - L * k;
        ans += ModInt::binom(k + m, m);
        k += 1;
    }
    println!("{}", ans.val);
}

use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};
impl Sub<i64> for ModInt {
    type Output = Self;
    fn sub(self, rhs: i64) -> Self {
        let val = (self.val - rhs).rem_euclid(Self::MODULO);
        ModInt { val }
    }
}
impl SubAssign for ModInt {
    fn sub_assign(&mut self, rhs: Self) {
        self.val = (self.val - rhs.val).rem_euclid(Self::MODULO);
    }
}
impl Sub for ModInt {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let val = (self.val - rhs.val).rem_euclid(Self::MODULO);
        ModInt { val }
    }
}
impl Mul<i64> for ModInt {
    type Output = Self;
    fn mul(self, rhs: i64) -> Self {
        let val = (self.val * rhs).rem_euclid(Self::MODULO);
        ModInt { val }
    }
}
impl MulAssign for ModInt {
    fn mul_assign(&mut self, rhs: Self) {
        self.val = (self.val * rhs.val).rem_euclid(Self::MODULO);
    }
}
impl Mul for ModInt {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let val = (self.val * rhs.val).rem_euclid(Self::MODULO);
        ModInt { val }
    }
}
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
pub trait Inv {
    fn inv(self) -> Self;
}
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
impl ModInt {
    const MODULO: i64 = 1_000_000_007;
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
}
impl Add<i64> for ModInt {
    type Output = Self;
    fn add(self, rhs: i64) -> Self {
        let val = (self.val + rhs).rem_euclid(Self::MODULO);
        ModInt { val }
    }
}
impl AddAssign for ModInt {
    fn add_assign(&mut self, rhs: Self) {
        self.val = (self.val + rhs.val).rem_euclid(Self::MODULO);
    }
}
impl Add for ModInt {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let val = (self.val + rhs.val).rem_euclid(Self::MODULO);
        ModInt { val }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ModInt {
    val: i64,
}