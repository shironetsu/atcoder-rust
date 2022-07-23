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
        C: [Usize1;N],
        X: [Usize1;N],
    }

    let mut conj = vec![vec![];N];
    let mut colors = BTreeSet::<usize>::new();
    let mut numbers = vec![btreeset!();N];
    
    for i in 0..N{
        conj[C[i]].push(X[i]);
        colors.insert(C[i]);
        numbers[C[i]].insert(X[i]);
    }

    let mut st = SegmentTree::<usize, usize>::from(vec![0;N]);
    let mut inv = 0;
    for i in 0..N{
        let x = X[i];
        let prev = st.get(x).clone();
        st.set(x, prev+1);
        inv += st.prod(x+1, N);
    }

    for &color in colors.iter(){
        let mut m = BTreeMap::new();
        for (i, &y) in numbers[color].iter().enumerate(){
            m.insert(y, i);
        }

        //println!("{:?}", m);

        let n = m.len();
        let mut stc = SegmentTree::<usize, usize>::from(vec![0;n]);
        let mut invc = 0;
        for  &x in conj[color].iter(){
            let &y = m.get(&x).unwrap();
            let prev = stc.get(y).clone();
            stc.set(y, prev+1);
            invc += stc.prod(y+1, n);
        }
        inv -= invc;
    }

    println!("{}", inv);
    
}

use std::marker::PhantomData;
impl Op<usize> for usize {
    fn op(lhs: &usize, rhs: &usize) -> usize {
        lhs + rhs
    }
    fn e() -> usize {
        0
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