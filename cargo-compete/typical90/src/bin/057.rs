#![allow(unused_imports)]
#![allow(non_snake_case)]
use bitset_fixed::BitSet;
use itertools::*;
use maplit::*;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
use std::fmt::Write;
use superslice::{Ext, Ext2};

fn rswap(vv: &mut Vec<BitSet>, i: usize, j: usize) {
    let tmp = vv[i].clone();
    vv[i] = vv[j].clone();
    vv[j] = tmp;
}

fn cswap(vv: &mut Vec<BitSet>, i: usize, j: usize) {
    for k in 0..vv.len() {
        let a = vv[k][i];
        let b = vv[k][j];
        vv[k].set(i, b);
        vv[k].set(j, a);
    }
}

#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    let mut m = vec![BitSet::new(N + 1); M];
    for i in 0..N {
        input! {
            t: usize,
            a: [Usize1;t],
        }
        for j in a {
            m[j].set(i, true);
        }
    }
    for i in 0..M {
        input! {
            s: i32,
        }
        if s == 1 {
            m[i].set(N, true);
        }
    }

    let mut r = 0;
    loop {
        if !(r < M.min(N)) {
            break;
        }

        if m[r][r] {
            let v = m[r].clone();
            for i in 0..M {
                if i == r {
                    continue;
                }
                if m[i][r] {
                    m[i] ^= &v;
                }
            }
            r += 1;
        } else {
            let mut found = false;
            for i in r..M {
                for j in r..N {
                    if (i, j) == (r, r) {
                        continue;
                    }
                    if m[i][j] {
                        rswap(&mut m, r, i);
                        cswap(&mut m, r, j);
                        found = true;
                        break;
                    }
                }
                if found {
                    break;
                }
            }
            if !found {
                break;
            }
        }
    }

    for i in r..M {
        if m[i][N] {
            println!("0");
            return;
        }
    }

    // for i in 0..M{
    //     for j in 0..N+1{
    //         print!("{}", if m[i][j] {1} else {0} );
    //     }
    //     println!();
    // }

    //println!("{}", r);

    let ans = ModInt::from(2).pow((N - r) as i64);
    println!("{}", ans.val);
}
//______________________________________________________________________________
//
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
