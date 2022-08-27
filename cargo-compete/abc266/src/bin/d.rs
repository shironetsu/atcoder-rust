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
        TXA: [(usize,usize,i64);N],
    }

    let tmax = 200005;

    let mut s = vec![vec![0i64;5];tmax];

    for i in 0..N{
        let (t, x, a) = TXA[i];
        s[t][x] = a;
    }

    
    let inf = 10i64.pow(15);
    let mut dp = vec![vec![-inf;5];tmax];
    dp[0][0] = 0;
    for t in 1..tmax {
        for x in 0..5 {
            if x > 0 {
                let score = dp[t-1][x-1] + s[t][x];
                dp[t][x].chmax(score);
            }
            let score = dp[t-1][x] + s[t][x];
            dp[t][x].chmax(score);
            if x < 4 {
                let score = dp[t-1][x+1] + s[t][x];
                dp[t][x].chmax(score);
            }
        }
    }

    // for i in 0..5 {
    //     println!("{:?}", dp[i]);
    // }

    let ans = dp[tmax-1].iter().max().unwrap();
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