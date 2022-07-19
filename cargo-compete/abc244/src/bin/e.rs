#![allow(unused_imports)]
#![allow(non_snake_case)]
use proconio::{input, fastout};
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use std::fmt::Write;
use std::collections::*;
use maplit::*;
use itertools::*;
use superslice::{Ext, Ext2};

const MODULO: i64 = 998244353;

#[fastout]
fn main() {
    input!{
        N: usize,
        M: usize,
        K: usize,
        S: Usize1,
        T: Usize1,
        X: Usize1,
        edges: [(Usize1, Usize1);M],
    }

    let mut ad = vec![vec![];N];
    for (u, v) in edges{
        ad[u].push(v);
        ad[v].push(u);
    }
    let mut dp = vec![vec![vec![0i64;2];N];K+1];
    dp[0][S][0] = 1;
    for i in 1..=K{
        for u in 0..N{
            for &v in ad[u].iter() {
                let t = if v == X { 1 } else { 0 };
                for s in 0..2{
                    dp[i][v][s^t] += dp[i-1][u][s];
                    dp[i][v][s^t] %= MODULO;
                }
            }
        }
    }
    println!("{}", dp[K][T][0].rem_euclid(MODULO));
    
}
