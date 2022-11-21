#![allow(unused_imports)]
#![allow(non_snake_case)]
use proconio::{input, fastout};
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use std::fmt::{Write, Display};
use std::collections::*;
use maplit::*;
use itertools::*;
use superslice::{Ext, Ext2};

#[fastout]
fn main() {
    input!{
        n: i64,
        k: i64,
    }

    let mut c = ModInt::from(1);
    let mut ans = ModInt::from(0);

    for d in 0..=k{
        if c == ModInt::from(0) {
            break;
        }
        ans += c;
        //println!("{} {}", d, c.val);
        c = c * ModInt::from(n-d) * ModInt::from(n-d-1) * (ModInt::from(d+1).inv().pow(2));
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
    pub const MODULO: i64 = 1_000_000_007;
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
//______________________________________________________________________________
//
pub trait ToString {
    fn to_string(&self) -> String;
}

impl ToString for [char] {
    fn to_string(&self) -> String {
        self.iter().collect::<String>()
    }
}
//______________________________________________________________________________
//
pub trait Answer {
    fn fmt(&self)->String;
    fn fmtl(&self)->String;
    fn ans(&self);
    fn ansl(&self);
}

impl<T: Display> Answer for Vec<T> {
    fn fmt(&self)->String {
        self
            .iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn ans(&self) {
        println!("{}", self.fmt());
    }

    fn fmtl(&self)->String {
        self
            .iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn ansl(&self) {
        println!("{}", self.fmtl());
    }
}
//______________________________________________________________________________
//
#[macro_export]
macro_rules! input_edges {
    ($n: expr, $m: expr, $edges: tt, $ad: tt) => {
        input! {
            $edges: [(Usize1, Usize1); $m],
        }

        let mut $ad = vec![vec![]; $n];
        for &(a, b) in $edges.iter() {
            $ad[a].push(b);
            $ad[b].push(a);
        }
        let $ad = $ad;
    };
}

