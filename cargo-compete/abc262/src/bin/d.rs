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
        a: [i64;N],
    }

    let mut ans = 0i64;
    for m in 1..=N{
        let mut dp = vec![vec![vec![0;m+1];m];N+1];
        dp[0][0][0] = 1;
        for i in 0..N{
            for j in 0..m{
                for k in 0..=m{
                    dp[i+1][j][k] = dp[i][j][k];
                }
            }
            let aa = a[i] as usize;
            for j in 0..m{
                for k in 0..m{
                    dp[i+1][(j+aa) % m][k+1] += dp[i][j][k];
                    dp[i+1][(j+aa) % m][k+1] %= MODULO;
                }
            }
        }

        //println!("{} {:?}",m, dp[N][0][m]);

        ans += dp[N][0][m];
        ans %= MODULO;
    }

    println!("{}", ans);
}
