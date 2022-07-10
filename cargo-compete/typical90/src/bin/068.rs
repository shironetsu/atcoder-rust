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
        N: usize,
        Q: usize,
        TXYV: [(i32, Usize1, Usize1, i64);Q],
    }

    let mut sti = SegmentTree::<i64, Add>::from(vec![0; N - 1]);
    let mut stb = SegmentTree::<bool, And>::from(vec![false; N - 1]);
    let mut ans = String::new();
    for (t, x, y, v) in TXYV {
        if t == 0 {
            sti.set(x, if x & 1 == 1 { -v } else { v });
            stb.set(x, true);
        } else {
            if x == y {
                writeln!(&mut ans, "{}", v);
            } else {
                let l = x.min(y);
                let r = x.max(y);
                if !stb.prod(l, r) {
                    writeln!(&mut ans, "Ambiguous");
                } else {
                    let c = sti.prod(l, r);
                    let a = if (x, y) == (l, r) {
                        let sgn1 = if (y - 1) & 1 == 0 { 1 } else { -1 };
                        let sgn2 = if (x ^ y) & 1 == 0 { 1 } else { -1 };
                        sgn1 * c + sgn2 * v
                    } else {
                        let sgn1 = if y & 1 == 0 { 1 } else { -1 };
                        let sgn2 = if (x ^ y) & 1 == 0 { 1 } else { -1 };
                        sgn1 * c + sgn2 * v
                    };
                    writeln!(&mut ans, "{}", a);
                }
            }
        }
    }

    print!("{}", ans);
}

use std::marker::PhantomData;

struct Add {}

impl Op<i64> for Add {
    fn op(lhs: &i64, rhs: &i64) -> i64 {
        *lhs + *rhs
    }

    fn e() -> i64 {
        0
    }
}

struct And {}

impl Op<bool> for And {
    fn op(lhs: &bool, rhs: &bool) -> bool {
        *lhs && *rhs
    }
    fn e() -> bool {
        true
    }
}

pub trait Op<S> {
    fn op(lhs: &S, rhs: &S) -> S;
    fn e() -> S;
}
impl<S: Clone, F: Op<S>> SegmentTree<S, F> {
    pub fn from(v: Vec<S>) -> SegmentTree<S, F> {
        let n = v.len();
        let log = Self::ceil_pow2(n);
        let size = 1 << log;
        let mut d = (0..2 * size).map(|_| F::e()).collect::<Vec<_>>();
        for (i, x) in v.into_iter().enumerate() {
            d[size + i] = x
        }
        let mut st = SegmentTree {
            n,
            log,
            size,
            d,
            _marker: PhantomData,
        };
        for i in (1..=(size - 1)).rev() {
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
    pub fn get(&self, p: usize) -> &S {
        &self.d[p + self.size]
    }
    pub fn prod(&mut self, l: usize, r: usize) -> S {
        let mut sml = F::e();
        let mut smr = F::e();
        let mut l = l + self.size;
        let mut r = r + self.size;
        while l < r {
            if l & 1 == 1 {
                sml = F::op(&sml, &self.d[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                smr = F::op(&self.d[r], &smr);
            }
            l >>= 1;
            r >>= 1;
        }
        F::op(&sml, &smr)
    }
    pub fn all_prod(&self) -> S {
        self.d[1].clone()
    }
    fn update(&mut self, k: usize) {
        self.d[k] = F::op(&self.d[2 * k], &self.d[2 * k + 1]);
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
}
pub struct SegmentTree<S: Clone, F: Op<S>> {
    n: usize,
    log: usize,
    size: usize,
    d: Vec<S>,
    _marker: PhantomData<F>,
}
