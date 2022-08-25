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
        M: usize,
        A: [i64;N],
        B: [i64;M],
    }

    let mut v = vec![];
    for &a in A.iter() {
        v.push((a, 'a'));
    }
    for &b in B.iter() {
        v.push((b, 'b'));
    }
    v.sort();

    let mut ans = std::i64::MAX;
    for i in 0..(N + M) - 1 {
        let (x, p) = v[i];
        let (y, q) = v[i + 1];
        if p != q {
            ans.chmin((x - y).abs());
        }
    }

    println!("{}", ans);
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
