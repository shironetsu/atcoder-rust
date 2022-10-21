#![allow(unused_imports)]
#![allow(non_snake_case)]
use proconio::{input, fastout};
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use std::fmt::{Write, Display};
use std::collections::*;
use maplit::*;
use itertools::*;
use superslice::{Ext, Ext2};

#[fastout]
fn main() {
    input!{
        mut X: i128,
        K: i128,
        D: i128,
    }

    // x + aD - (K-a)D  0<=a<=K
    // x + (2a-K)D 0<=a<=K
    // x - r * D
    // r = K - 2a
    // a = (K+r)/2
    // 0 <= (K+r)/2 <= K
    // 0 <= K+r<=2*K
    // -K <= r <= K && K+r is even

    if X < 0 {
        X = -X;
    }

    let ans = if X - K * D >= 0 {
            X - K * D
    } else {
        let r = num_integer::div_floor(X, D);
        [r-2, r-1,r,r+1,r+2].iter().filter(|&s| s.abs()<=K && (K+s)&1==0).map(|s|(X - s*D).abs()).min().unwrap()
    };

    println!("{}", ans);
}
//______________________________________________________________________________
//
pub trait ToString {
    fn to_string(&self) -> String;
}

impl ToString for [char] {
    fn to_string(&self) -> String {
        self.iter().collect::<String>()
    }
}
//______________________________________________________________________________
//
pub trait Answer {
    fn fmt(&self)->String;
    fn fmtl(&self)->String;
    fn ans(&self);
    fn ansl(&self);
}

impl<T: Display> Answer for Vec<T> {
    fn fmt(&self)->String {
        self
            .iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn ans(&self) {
        println!("{}", self.fmt());
    }

    fn fmtl(&self)->String {
        self
            .iter()
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

