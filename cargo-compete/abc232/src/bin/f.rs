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
        X: i64,
        Y: i64,
        A: [i64;N],
        B: [i64;N],
    }

    let S = 1 << N;
    let mut dp = vec![std::i64::MAX; S];
    dp[0] = 0;
    for s in 0..S {
        for k in 0..N {
            if (s >> k) & 1 == 0 {
                let ones = s.count_ones();
                let left = (s >> k).count_ones();
                let cost = dp[s] + X * (A[k] - B[ones as usize]).abs() + Y * (left as i64);
                dp[s + (1 << k)].chmin(cost);
            }
        }
    }
    println!("{}", dp[S - 1]);
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
