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
        A: [i64;2*N],
    }

    let mut dp = vec![vec![std::i64::MAX; 2 * N + 1]; 2 * N + 1];
    for i in 0..=2 * N {
        dp[i][i] = 0;
    }
    for l in (2..=2 * N).step_by(2) {
        for i in 0..=2 * N {
            if i + l > 2 * N {
                break;
            }
            for j in (i + 1..=2 * N).step_by(2) {
                if j + 1 > i + l {
                    break;
                }
                let a = (A[i] - A[j]).abs() + dp[i + 1][j] + dp[j + 1][i + l];
                dp[i][i + l].chmin(a);
            }
        }
    }

    //println!("{:?}", dp);

    println!("{}", dp[0][2 * N]);
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
