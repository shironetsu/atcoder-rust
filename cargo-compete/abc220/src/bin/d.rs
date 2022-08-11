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
        A: [usize;N],
    }

    let m = 998244353i64;

    let mut dp = vec![vec![0i64;10];N];
    dp[0][A[0]] = 1;
    for i in 0..N-1{
        for j in 0..10{
            dp[i+1][(j+A[i+1])%10] += dp[i][j];
            dp[i+1][(j+A[i+1])%10] %= m;
            dp[i+1][(j*A[i+1])%10] += dp[i][j];
            dp[i+1][(j*A[i+1])%10] %= m;
        }
    }
    for i in 0..10{
        println!("{}", dp[N-1][i]);
    }



    
}
