#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::*;
use maplit::*;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
use std::fmt::Write;
use superslice::{Ext, Ext2};

const MODULO: i64 = 998244353;

macro_rules! dp {
    ($c: expr, $N: expr, $M: expr) => {
        {
            let mut dp = vec![vec![vec![0; $c]; $M]; $N + 1];
            for j in 0..$M {
                dp[1][j][0] = 1;
            }
            for i in 1..$N {
                for j0 in 0..$M {
                    for j1 in 0..$M {
                        for k in 0..$c {
                            if j1 > j0 {
                                if k == $c - 1 {
                                    continue;
                                }
                                dp[i + 1][j1][k + 1] += dp[i][j0][k];
                                dp[i + 1][j1][k + 1] %= MODULO;
                            } else if j1 <= j0 {
                                dp[i + 1][j1][0] += dp[i][j0][k];
                                dp[i + 1][j1][0] %= MODULO;
                            }
                        }
                    }
                }
            }
            // for i in 0..=$N {
            //     for j in 0..$M {
            //         print!("{:?} ", dp[i][j]);
            //     }
            //     println!();
            // }
            let mut sum = 0;
            for j in 0..$M{
                for k in 0..$c{
                    sum += dp[$N][j][k];
                    sum %= MODULO;
                } 
            }
            sum
        };
    };
}

#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
    }

    let sum3 = dp!(3, N, M);
    let sum2 = dp!(2, N, M);
    let sum1 = dp!(1, N, M);

    let ans = (sum3 - sum2).rem_euclid(MODULO);
    // println!("{}", sum3);
    // println!("{}", sum2);
    // println!("{}", sum1);
    println!("{}",  ans);
}
