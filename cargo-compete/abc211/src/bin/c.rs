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
        S: Chars,
    }

    let mut m = 10i64.pow(9) + 7;
    let mut c = vec![ 'c' , 'h' , 'o' , 'k' , 'u' , 'd' , 'a' , 'i' ];
    let mut dp = vec![vec![0;c.len()+1];S.len()+1];
    dp[0][0] = 1;

    for i in 0..S.len(){
        dp[i+1] = dp[i].clone();
        for j in 0..c.len() {
            if S[i] == c[j] {
                dp[i+1][j+1] += dp[i][j];
                dp[i+1][j+1] %= m;
            }
        }
    }

    let mut ans = dp[S.len()][c.len()];
    println!("{}", ans);

    
}
