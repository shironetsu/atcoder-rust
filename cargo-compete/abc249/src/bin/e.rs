#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::*;
use maplit::*;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
use std::fmt::Write;
use superslice::{Ext, Ext2};

#[fastout]
fn main() {
    input! {
        N: usize,
        P: i64,
    }

    let mut dp = vvec![0;N+1,2*N+1];

    dp[0][0] = 1;
    for i in 1..=N {
        for j in 0..=2 * N {
            dp[i][j] = dp[i - 1][j] 
                + c(i, 1, j, 2) * dp.sget(i, 1, j, 2)
                + c(i, 10, j, 3) * dp.sget(i, 10, j, 3)
                - c(i, 10, j, 2) * dp.sget(i, 10, j, 2)
                + c(i, 100, j, 4) * dp.sget(i, 100, j, 4)
                - c(i, 100, j, 3) * dp.sget(i, 100, j, 3)
                + c(i, 1000, j, 5) * dp.sget(i, 1000, j, 5)
                - c(i, 1000, j, 4) * dp.sget(i, 1000, j, 4)
                - if i == 1 && j == 0 { 1 } else { 0 };
            dp[i][j] = dp[i][j].rem_euclid(P);
        }
    }

    let ans = (0..N).map(|i| dp[N][i]).sum::<i64>().rem_euclid(P);

    // for i in 0..=N{
    //     println!("{:?}", dp[i]);
    // }

    println!("{}", ans);
}

fn c(i: usize, a: usize, j: usize, b: usize) -> i64 {
    if i > a {
        25
    } else if j == b {
        26
    } else {
        0
    }
}

pub trait SafeGet<T> {
    fn sget(&self, i: usize, a: usize, j: usize, b: usize) -> T;
}

impl<T: Default + Copy> SafeGet<T> for Vec<Vec<T>> {
    fn sget(&self, i: usize, a: usize, j: usize, b: usize) -> T {
        if i >= a && j >= b {
            self[i - a][j - b]
        } else {
            T::default()
        }
    }
}

#[macro_export]
macro_rules! vvec {
    ($ val : expr ; $ a : expr , $ b : expr ) => {
        vec![vec![$val; $b]; $a]
    };
}
