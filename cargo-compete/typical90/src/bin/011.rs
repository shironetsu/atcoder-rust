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
    }

    let mut D = vec![0; N];
    let mut C = vec![0; N];
    let mut S = vec![0; N];

    for i in 0..N {
        input! {
            d: i64,
            c: i64,
            s: i64,
        }
        D[i] = d;
        C[i] = c;
        S[i] = s;
    }
    let dmax = 5002;
    let mut dp = vec![vec![0; N + 1]; dmax];

    for i in 0..dmax {
        for j in 0..=N {}
    }
}
//________________________________________________________________________________
//
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
