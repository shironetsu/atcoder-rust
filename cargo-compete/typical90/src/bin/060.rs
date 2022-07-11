#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::*;
use maplit::*;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
use std::fmt::Write;
use superslice::{Ext, Ext2};

const INF: i64 = 1_000_000_000;

#[fastout]
fn main() {
    input! {
        N: usize,
        A: [i64;N],
    }

    let mut p = vec![0; N];
    let mut dp = vec![INF; N];
    for (i, &a) in A.iter().enumerate() {
        let j = dp.lower_bound(&a);
        dp[j] = a;
        p[i] = j + 1;
    }

    let mut q = vec![0; N];
    let mut dp = vec![INF; N];
    let B = A.iter().copied().rev().collect_vec();
    for (i, &b) in B.iter().enumerate() {
        let j = dp.lower_bound(&b);
        dp[j] = b;
        q[N - i - 1] = j + 1;
    }

    let mut ans = 0;
    for i in 0..N {
        ans.chmax(p[i] + q[i] - 1);
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
