#![allow(unused_imports)]
#![allow(non_snake_case)]
use proconio::{input, fastout};
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use std::fmt::Write;
use std::collections::*;
use maplit::*;
use itertools::*;
use superslice::{Ext, Ext2};
use std::marker::PhantomData;

#[fastout]
fn main() {
    input!{
        N: usize,
        K: usize,
        p: [Usize1;N],
    }

    let mut q = vec![0;N];
    for (i, &pp) in p.iter().enumerate(){
        q[pp] = i;
    }

    let mut st = SegmentTree::<usize, usize>::from(p.clone());

    let a = {
        let mut ans = vec![];
        // xxxxxhxxxoxxooooxooo
        let mut K = K;
        let mut h = 0;
        let mut l = 0;
        for p in 0..N{
            if q[p] <= K {
                h = p;
                K -= q[p];
                l = q[p];
                ans.push(h);
                break;
            }
        }

        loop {
            let u = st.prod(l, l+K);
            ans.push(u);
            K -= q[u]-l;
            l = q[u];
            if K == 0 {
                break;
            }
        }
        for i in l..N{
            ans.push(p[i]);
        }
        ans
    };

    let b = {
        let mut ans = vec![];
        let mut K = K;
        let mut h = 0;
        let mut l = 0;
        for p in 0..N{
            if (N-q[p]) % N <= K {
                h = p;
                K -= (N-q[p]) % N;
                l = q[p];
                ans.push(h);
                break;
            }
        }

        loop {
            let u = st.prod(l, l+K);
            ans.push(u);
            K -= q[u]-l;
            l = q[u];
            if K == 0 {
                break;
            }
        }
        for i in l..N{
            ans.push(p[i]);
        }
        ans
    };

    let ans = a.min(b);
    println!("{:?}", ans);    
}

impl Op<usize> for usize{
    fn op(lhs: &usize, rhs: &usize)->usize{
        *lhs.min(rhs)
    }

    fn e()->usize{
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