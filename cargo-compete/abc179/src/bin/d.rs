#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::*;
use maplit::*;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
use std::fmt::{Display, Write};
use superslice::{Ext, Ext2};

#[fastout]
fn main() {
    input! {
        N: usize,
        K: usize,
        LR: [(usize, usize);K],
    }

    let m = 998244353i64;
    let mut dp = vec![0; N];
    let mut imos = vec![0i64; N];
    dp[0] = 1;
    for i in 0..N {
        if i > 0 {
            imos[i] += imos[i - 1];
            imos[i] = imos[i].rem_euclid(m);
            dp[i] = imos[i];
        }
        for &(l, r) in LR.iter() {
            if i + l < N {
                imos[i + l] += dp[i];
            }
            if i + r + 1 < N {
                imos[i + r + 1] -= dp[i];
            }
        }
    }
    println!("{}", dp[N - 1]);
}
//______________________________________________________________________________
//
pub trait Answer {
    fn fmt(&self) -> String;
    fn fmtl(&self) -> String;
    fn ans(&self);
    fn ansl(&self);
}

impl<T: Display> Answer for Vec<T> {
    fn fmt(&self) -> String {
        self.iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn ans(&self) {
        println!("{}", self.fmt());
    }

    fn fmtl(&self) -> String {
        self.iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn ansl(&self) {
        println!("{}", self.fmtl());
    }
}
//______________________________________________________________________________
//
#[macro_export]
macro_rules! input_edges {
    ($n: expr, $m: expr, $edges: tt, $ad: tt) => {
        input! {
            $edges: [(Usize1, Usize1); $m],
        }

        let mut $ad = vec![vec![]; $n];
        for &(a, b) in $edges.iter() {
            $ad[a].push(b);
            $ad[b].push(a);
        }
        let $ad = $ad;
    };
}
