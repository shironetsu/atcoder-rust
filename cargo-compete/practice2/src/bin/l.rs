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
        A: [i32;N],
    }
    let mut ls = LazySegmentTree::<i32, F>::from(A);
    for _ in 0..Q {
        input! {
            t: i32,
            l: Usize1,
            r: Usize1,
        }
        if t == 1 {
            ls.apply_range(l, r + 1, F { val: 1 });
        } else {
            let a = inv(&mut ls, l, r + 1);
            println!("{}", a);
        }
    }
}

fn inv(ls: &mut LazySegmentTree<i32, F>, l: usize, r: usize) -> i32 {
    if l == r || l + 1 == r {
        return 0;
    }
    let m = (l + r) / 2;
    let suml = ls.prod(l, m);
    let invl = inv(ls, l, m);
    let sumr = ls.prod(m, r);
    let invr = inv(ls, m, r);
    invl + invr + suml * (r as i32 - m as i32 - sumr)
}

impl Monoid for i32 {
    fn op(&self, rhs: &Self) -> Self {
        *self + *rhs
    }
    fn e() -> Self {
        0
    }
}

#[derive(Clone, Copy)]
struct F {
    val: i32,
}

impl Action<i32> for F {
    fn apply(&self, x: i32) -> i32 {
        self.val ^ x
    }
    fn composition(&self, other: &Self) -> Self {
        F {
            val: self.val ^ other.val,
        }
    }
    fn id() -> Self {
        F { val: 0 }
    }
}

pub trait Monoid {
    fn op(&self, rhs: &Self) -> Self;
    fn e() -> Self;
}

pub trait Action<S: Monoid> {
    fn apply(&self, x: S) -> S; //mapping
    fn composition(&self, other: &Self) -> Self;
    fn id() -> Self;
}

pub struct LazySegmentTree<S: Monoid + Clone, F: Action<S> + Clone> {
    log: usize,
    size: usize,
    d: Vec<S>,
    lz: Vec<F>,
}

impl<S: Monoid + Clone, F: Action<S> + Clone> LazySegmentTree<S, F> {
    pub fn from(v: Vec<S>) -> LazySegmentTree<S, F> {
        let n = v.len();
        let log = ceil_pow2(n);
        let size = 1 << log;
        let mut d = vec![S::e(); 2 * size];
        for i in 0..n {
            d[size + i] = v[i].clone();
        }
        let lz = (0..size).map(|_| F::id()).collect::<Vec<_>>();
        let mut ls = LazySegmentTree { log, size, d, lz };
        for i in (1..=size - 1).rev() {
            ls.update(i);
        }
        ls
    }

    pub fn set(&mut self, p: usize, x: S) {
        let p = p + self.size;
        for i in (1..=self.log).rev() {
            self.push(p >> i);
        }
        self.d[p] = x;
        for i in 1..=self.log {
            self.update(p >> i);
        }
    }

    pub fn get(&mut self, p: usize) -> &S {
        //reference?
        let p = p + self.size;
        for i in (1..=self.log).rev() {
            self.push(p >> i);
        }
        &self.d[p]
    }

    pub fn prod(&mut self, l: usize, r: usize) -> S {
        if l == r {
            return S::e();
        }

        let l = l + self.size;
        let r = r + self.size;

        for i in (1..=self.log).rev() {
            if ((l >> i) << i) != l {
                self.push(l >> i);
            }
            if ((r >> i) << i) != r {
                self.push((r - 1) >> i);
            }
        }

        let mut sml = S::e();
        let mut smr = S::e();
        let mut l = l;
        let mut r = r;
        while l < r {
            if l & 1 == 1 {
                sml = sml.op(&self.d[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                smr = smr.op(&self.d[r]);
            }
            l >>= 1;
            r >>= 1;
        }

        sml.op(&smr)
    }

    fn all_prod(&self) -> &S {
        &self.d[1]
    }

    fn apply_at(&mut self, p: usize, f: F) {
        let p = p + self.size;
        for i in (1..=self.log).rev() {
            self.push(p >> i);
        }
        self.d[p] = f.apply(self.d[p].clone()); //in-place
        for i in 1..=self.log {
            self.update(p >> i);
        }
    }

    fn apply_range(&mut self, l: usize, r: usize, f: F) {
        if l == r {
            return;
        }

        let l = l + self.size;
        let r = r + self.size;

        for i in (1..=self.log).rev() {
            if ((l >> i) << i) != l {
                self.push(l >> i);
            }
            if ((r >> i) << i) != r {
                self.push((r - 1) >> i);
            }
        }

        let (l, r) = {
            let mut l = l;
            let mut r = r;
            let l2 = l;
            let r2 = r;
            while l < r {
                if l & 1 == 1 {
                    self.all_apply(l, &f);
                    l += 1;
                }
                if r & 1 == 1 {
                    r -= 1;
                    self.all_apply(r, &f);
                }
                l >>= 1;
                r >>= 1;
            }
            (l2, r2)
        };

        for i in 1..=self.log {
            if ((l >> i) << i) != l {
                self.update(l >> i);
            }
            if ((r >> i) << i) != r {
                self.update((r - 1) >> i);
            }
        }
    }

    fn update(&mut self, k: usize) {
        self.d[k] = self.d[2 * k].op(&self.d[2 * k + 1]);
    }

    fn all_apply(&mut self, k: usize, f: &F) {
        self.d[k] = f.apply(self.d[k].clone());
        if k < self.size {
            self.lz[k] = f.composition(&self.lz[k]);
        }
    }

    fn push(&mut self, k: usize) {
        let f = self.lz[k].clone();
        self.all_apply(2 * k, &f);
        self.all_apply(2 * k + 1, &f);
        self.lz[k] = F::id();
    }
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
