#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::*;
use maplit::*;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
use std::fmt::Write;
use superslice::{Ext, Ext2};

const MODULO: i64 = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        N: usize,
        S: Chars,
    }

    let ac = "atcoder".chars().collect_vec();
    let mut dp = vec![vec![0; N + 1]; ac.len() + 1];
    for i in 0..=N {
        dp[0][i] = 1;
    }
    for i in 0..N {
        for j in 0..=ac.len() {
            dp[j][i + 1] = dp[j][i];
        }
        for j in 0..ac.len() {
            if S[i] == ac[j] {
                dp[j + 1][i + 1] += dp[j][i];
                dp[j + 1][i + 1] %= MODULO;
            }
        }
    }

    println!("{}", dp[ac.len()][N]);
}
