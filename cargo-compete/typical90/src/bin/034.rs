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
        K: usize,
        a: [i64;N],
    }

    let mut c = BTreeMap::<i64, usize>::new();
    let mut l = 0;
    let mut r = 0;
    let mut ans = 0;
    loop {
        while c.len() <= K {
            *c.entry(a[r]).or_default() += 1;
            r += 1;
            if c.len() <= K {
                ans.chmax(r - l);
            }
            if r == N {
                println!("{}", ans);
                return;
            }
        }
        while c.len() > K {
            if let Some(&e) = c.get(&a[l]) {
                if e == 1 {
                    c.remove(&a[l]);
                } else {
                    *c.entry(a[l]).or_default() -= 1;
                }
            }
            l += 1;
            if l <= r {
                break;
            }
        }
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
