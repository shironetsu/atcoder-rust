#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::*;
use maplit::*;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
use std::fmt::Write;
use superslice::{Ext, Ext2};

const MODULO: i64 = 998_244_353;

#[fastout]
fn main() {
    input! {
        N: usize,
        a: [usize;N],
        b: [usize;N],
    }

    let m = 3005;
    let mut dp = vec![vec![0i64; m]; N];
    let mut ep = vec![vec![0i64; m + 1]; N];
    for k in a[0]..=b[0] {
        dp[0][k] = 1;
    }
    for k in 0..m {
        ep[0][k + 1] = (ep[0][k] + dp[0][k]).rem_euclid(MODULO);
    }
    for i in 1..N {
        for k in a[i]..=b[i] {
            let c = k.min(b[i - 1]) + 1;
            dp[i][k] = if c >= a[i - 1] {
                ep[i - 1][c] - ep[i - 1][a[i - 1]]
            } else {
                0
            };
            dp[i][k] = dp[i][k].rem_euclid(MODULO);
        }
        for k in 0..m {
            ep[i][k + 1] = (ep[i][k] + dp[i][k]).rem_euclid(MODULO);
        }
    }

    // for i in 0..N{
    //     println!("{:?}", &dp[i][..10]);
    //     println!("{:?}", &ep[i][..10]);
    // }

    let ans = ep[N - 1][m];
    println!("{}", ans);
}
