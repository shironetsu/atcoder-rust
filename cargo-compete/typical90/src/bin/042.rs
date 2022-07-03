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
        K :usize,
    }

    if K % 9 != 0 {
        println!("0");
        return;
    }
    let mut dp = vec![0;K+1];
    dp[0] = 1;
    for i in 0..K{
        dp[i] %= MODULO;
        for j in 1..=9{
            if i + j <= K {
                dp[i+j] += dp[i];
            }
        }
    }
    println!("{}", dp[K] % MODULO);
}
