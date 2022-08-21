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
        ABCDEF: (i64,i64,i64,i64,i64,i64),
        XY: [(i64, i64);M],
    }

    let mut XY = XY.into_iter().collect::<BTreeSet<_>>();

    let (A,B,C,D,E,F)=ABCDEF;

    let mut dp = vec![vec![vec![0;N+1];N+1];N+1];
    dp[0][0][0] = 1;
    for p in 0..=N{
        for q in 0..=N-p{
            for r in 0..=N-p-q{
                let pi = p as i64;
                let qi = q as i64;
                let ri = r as i64;
                let x = pi * A + qi * C + ri * E;
                let y = pi * B + qi * D + ri * F;
                if XY.contains(&(x, y)) {
                    continue;
                }
                if p > 0 {
                    dp[p][q][r] += dp[p-1][q][r];
                    dp[p][q][r] %= MODULO;
                }
                if q > 0 {
                    dp[p][q][r] += dp[p][q-1][r];
                    dp[p][q][r] %= MODULO;
                }
                if r > 0 {
                    dp[p][q][r] += dp[p][q][r-1];
                    dp[p][q][r] %= MODULO;
                }
            }
        }
    }

    let mut ans = 0;
    for p in 0..=N{
        for q in 0..=N-p{
            ans += dp[p][q][N-p-q];
            ans %= MODULO;
        }
    }

    println!("{}", ans);



    
}
