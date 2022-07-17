#![allow(unused_imports)]
#![allow(non_snake_case)]
use proconio::{input, fastout};
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use std::fmt::Write;
use std::collections::*;
use maplit::*;
use itertools::*;
use superslice::{Ext, Ext2};

#[fastout]
fn main() {
    input!{
        N: usize,
        M: usize,
        mut AB: [(Usize1, Usize1);N],
    }

    AB.sort();
    
    let mut bmin = vec![AB[0].1];
    for i in 1..N{
        bmin.push(bmin[i-1].min(AB[i].1));
    }
    let amax = AB[N-1].0;
    let mut lst = LazySegmentTree::<i64, Add, Adder>::from(vec![0; M+1]);
    for l in 0..=M{
        let j = AB.upper_bound(&(l, 0));
        let u = bmin[j].max(amax);
        if u+1 < l {
            break;
        }
        lst.apply_range(u+1-l, M-l+1, &Adder{ x:1 });
    }

    // let mut bmin = vec![0;N+1];
    // for i in 1..=N{
    //     bmin[i] = bmin[i-1].min(AB[i-1].1);
    // }

    // let amin = AB[N-1].0;
    // for l in 0..M{
    //     let j = AB.upper_bound(&(l, 0));
    //     let c = bmin[j].max(amin);
    //     if l > c {
    //         break;
    //     }
    //     lst.apply_range(c-l, M-l+1, &Adder{ x: 1 });
    // }

    for i in 1..=M{
        print!("{}{}", lst.get(i), if i==M{""} else { " " });
    }
    

    
}
pub trait Change<T: PartialOrd> {
    fn chmin(&mut self, rhs: Self) -> bool;
    fn chmax(&mut self, rhs: Self) -> bool;
}
impl<T: PartialOrd> Change<T> for T {
    fn chmax(&mut self, rhs: Self) -> bool {
        if *self < rhs {
            *self = rhs;
            true
        } else {
            false
        }
    }
    fn chmin(&mut self, rhs: Self) -> bool {
        if *self > rhs {
            *self = rhs;
            true
        } else {
            false
        }
    }
}
//________________________________________________________________________________
//
struct Add {}
impl Op<i64> for Add {
    fn op(lhs: &i64, rhs: &i64) -> i64 {
        lhs + rhs
    }

    fn e() -> i64 {
        0
    }
}

#[derive(Clone)]
struct Adder {
    x: i64,
}

impl Action<i64> for Adder {
    fn apply(&self, x: &i64) -> i64 {
        self.x + x
    }

    fn composition(&self, other: &Self) -> Self {
        Adder {
            x: self.x + other.x,
        }
    }

    fn id() -> Self {
        Adder { x: 0 }
    }
}

use std::marker::PhantomData;
impl<S: Clone, T: Op<S>, U: Action<S> + Clone> LazySegmentTree<S, T, U> {
    pub fn from(v: Vec<S>) -> LazySegmentTree<S, T, U> {
        let n = v.len();
        let log = Self::ceil_pow2(n);
        let size = 1 << log;
        let mut d = (0..2 * size).map(|_| T::e()).collect::<Vec<_>>();
        let lz = (0..size).map(|_| U::id()).collect::<Vec<_>>();
        for (i, x) in v.into_iter().enumerate() {
            d[size + i] = x;
        }
        let mut lst = LazySegmentTree {
            n,
            log,
            size,
            d,
            _marker: PhantomData,
            lz,
        };
        for i in (1..=size - 1).rev() {
            lst.update(i);
        }
        lst
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
        let p = p + self.size;
        for i in (1..=self.log).rev() {
            self.push(p >> i);
        }
        &self.d[p]
    }
    pub fn prod(&mut self, l: usize, r: usize) -> S {
        if l == r {
            return T::e();
        }
        let mut l = l + self.size;
        let mut r = r + self.size;
        for i in (1..=self.log).rev() {
            if (l >> i) << i != l {
                self.push(l >> i);
            }
            if ((r >> i) << i != r) {
                self.push((r - 1) >> i);
            }
        }
        let mut sml = T::e();
        let mut smr = T::e();
        while l < r {
            if l & 1 == 1 {
                sml = T::op(&sml, &self.d[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                smr = T::op(&self.d[r], &smr);
            }
            l >>= 1;
            r >>= 1;
        }
        T::op(&sml, &smr)
    }
    pub fn all_prod(&self) -> S {
        self.d[1].clone()
    }
    pub fn apply_at(&mut self, p: usize, f: U) {
        let p = p + self.size;
        for i in (1..=self.log).rev() {
            self.push(p >> i);
        }
        self.d[p] = f.apply(&self.d[p]);
        for i in 1..=self.log {
            self.update(p >> i);
        }
    }
    pub fn apply_range(&mut self, l: usize, r: usize, f: &U) {
        if l == r {
            return;
        }
        let l = l + self.size;
        let r = r + self.size;
        for i in (1..=self.log).rev() {
            if (l >> i) << i != l {
                self.push(l >> i);
            }
            if (r >> i) << i != r {
                self.push((r - 1) >> i);
            }
        }
        {
            let mut l = l;
            let mut r = r;
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
        }
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
        self.d[k] = T::op(&self.d[2 * k], &self.d[2 * k + 1]);
    }
    fn all_apply(&mut self, k: usize, f: &U) {
        self.d[k] = f.apply(&self.d[k]);
        if k < self.size {
            self.lz[k] = f.composition(&self.lz[k]);
        }
    }
    fn push(&mut self, k: usize) {
        let f = self.lz[k].clone();
        self.all_apply(2 * k, &f);
        self.all_apply(2 * k + 1, &f);
        self.lz[k] = U::id();
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
pub trait Op<S> {
    fn op(lhs: &S, rhs: &S) -> S;
    fn e() -> S;
}
pub trait Action<S> {
    fn apply(&self, x: &S) -> S;
    fn composition(&self, rhs: &Self) -> Self;
    fn id() -> Self;
}
struct LazySegmentTree<S, T: Op<S>, U: Action<S>> {
    n: usize,
    log: usize,
    size: usize,
    d: Vec<S>,
    _marker: PhantomData<T>,
    lz: Vec<U>,
}