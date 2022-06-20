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
        N: usize,
        Q: usize,
        A: [i64;N],
    }

    let A0 = A
        .iter()
        .map(|&x| ModInt998244353::from(x))
        .collect::<Vec<_>>();
    let mut st0 = SegmentTree::from(A0);

    let A1 = A
        .iter()
        .enumerate()
        .map(|(i, &x)| {
            let i = ModInt998244353::from(i as i64);
            let x = ModInt998244353::from(x as i64);
            i * x
        })
        .collect();
    let mut st1 = SegmentTree::from(A1);
    let A2 = A
        .iter()
        .enumerate()
        .map(|(i, &x)| {
            let i = ModInt998244353::from(i as i64);
            let x = ModInt998244353::from(x as i64);
            i * i * x
        })
        .collect();

    let mut st2 = SegmentTree::from(A2);

    let mut ans = String::new();
    for _ in 0..Q {
        input! {
            t: i32,
        }
        if t == 1 {
            input! {
                x: Usize1,
                v: i64,
            }
            let i = ModInt998244353::from(x as i64);
            let v = ModInt998244353::from(v);
            st0.set(x, v);
            st1.set(x, i * v);
            st2.set(x, i * i * v);
        } else {
            input! {
                x: Usize1,
            }
            //let i = ModInt998244353::from(x as i64);
            let p = ModInt998244353::from(x as i64 + 1)
                * ModInt998244353::from(x as i64 + 2)
                * st0.prod(0, x + 1)
                - ModInt998244353::from(2 * x as i64 + 3) * st1.prod(0, x + 1)
                + st2.prod(0, x + 1);
            let p = p.val;
            let a = if p % 2 == 0 {
                p / 2
            } else {
                ((p + MODULO) / 2).rem_euclid(MODULO)
            };
            writeln!(ans, "{}", a);
        }
    }

    print!("{}", ans);
}
//______________________________________________________________________________
//
impl Monoid for ModInt998244353 {
    fn op(&self, rhs: &Self) -> Self {
        *self + *rhs
    }

    fn e() -> Self {
        ModInt998244353::from(0)
    }
}

//______________________________________________________________________________
//
use std::ops::{Add, Mul, Sub};

const MODULO: i64 = 998244353;

#[derive(Clone, Copy)]
pub struct ModInt998244353 {
    val: i64,
}

impl ModInt998244353 {
    pub fn from(x: i64) -> ModInt998244353 {
        let val = x.rem_euclid(MODULO);
        ModInt998244353 { val }
    }
}

impl Add for ModInt998244353 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let val = (self.val + rhs.val).rem_euclid(MODULO);
        ModInt998244353 { val }
    }
}

impl Sub for ModInt998244353 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let val = (self.val - rhs.val).rem_euclid(MODULO);
        ModInt998244353 { val }
    }
}

impl Mul for ModInt998244353 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let val = (self.val * rhs.val).rem_euclid(MODULO);
        ModInt998244353 { val }
    }
}
//______________________________________________________________________________
//
pub trait Monoid {
    fn op(&self, rhs: &Self) -> Self;
    fn e() -> Self;
}

#[derive(Debug)]
pub struct SegmentTree<T: Monoid> {
    log: usize,
    size: usize,
    d: Vec<T>,
}

fn ceil_pow2(n: usize) -> usize {
    let mut m = 1;
    let mut log = 0;
    while n > m {
        m <<= 1;
        log += 1;
    }
    log
}

impl<S: Monoid> SegmentTree<S> {
    pub fn from(v: Vec<S>) -> SegmentTree<S> {
        let n = v.len();
        let log = ceil_pow2(n);
        let size = 1 << log;
        let mut d = vec![];
        for _ in 0..2 * size {
            d.push(S::e());
        }
        for (i, x) in v.into_iter().enumerate() {
            d[size + i] = x;
        }

        let mut st = SegmentTree { d, log, size };
        for i in (1..=size - 1).rev() {
            st.update(i);
        }

        st
    }

    pub fn set(&mut self, p: usize, x: S) {
        let p = p + self.size;
        self.d[p] = x;
        for i in 1..=self.log {
            self.update(p >> i);
        }
    }

    pub fn get(&self, p: usize) -> Option<&S> {
        self.d.get(p + self.size)
    }

    pub fn prod(&self, l: usize, r: usize) -> S {
        let mut sml = S::e();
        let mut smr = S::e();
        let mut l = l + self.size;
        let mut r = r + self.size;

        while l < r {
            if l & 1 == 1 {
                sml = sml.op(&self.d[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                smr = self.d[r].op(&smr);
            }

            l >>= 1;
            r >>= 1;
        }
        sml.op(&smr)
    }

    pub fn all_prod(&self) -> &S {
        self.d.get(1).unwrap()
    }

    fn update(&mut self, k: usize) {
        self.d[k] = self.d[2 * k].op(&self.d[2 * k + 1]);
    }
}
