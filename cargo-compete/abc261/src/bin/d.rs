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
        X: [i64;N],
        CY: [(usize, i64);M],
    }

    let mut dp = vec![vec![0;N+1];N+1];

    let mut bonus = vec![0;N+1];
    for &(c, y) in CY.iter(){
        bonus[c] = y;
    }

    dp[0][0] = 0;

    for i in 1..=N{
        dp[i][0] = dp[i-1][0];
        for j in 1..=i{
            let prev = dp[i-1][j-1];
            dp[i][j] = prev + X[i-1] + bonus[j];
            dp[i][0].chmax(prev);
        }
    }

    // for i in 0..=N{
    //     println!("{:?}", dp[i]);
    // }

    let ans = dp[N].iter().max().unwrap();
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