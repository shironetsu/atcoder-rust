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
        N: usize,
        M: usize,
        a: [usize;N],
    }
    let mut aa = vec![0;N+1];
    for i in 0..N{
        aa[i+1] = aa[i] + a[i]; 
    }

    let inf = 1_000_000_000;
    let mut dp = vec![vec![inf;M+1];N+1];
    let mut ep = vec![vec![inf;M+1];N+1];
    dp[1][0] = 0;
    if a[0] <= M{
        ep[1][a[0]] = 0;
    }
    for i in 1..N{
        for m in 0..=M{
            let pd = dp[i][m];
            let pe = ep[i][m];
            dp[i+1][m].chmin(pd);
            dp[i+1][m].chmin(pe+1);
            if m + a[i] <= M {
                ep[i+1][m+a[i]].chmin(pe);
                ep[i+1][m+a[i]].chmin(pd+1);
            }
        }
    }

    // for i in 0..=N{
    //     for j in 0..=M{
    //         if dp[i][j] == inf {
    //             print!("∞ ");
    //         } else {
    //             print!("{} ", dp[i][j]);
    //         }
    //     }
    //     println!();
    // }
    // println!();
    // for i in 0..=N{
    //     for j in 0..=M{
    //         if ep[i][j] == inf {
    //             print!("∞ ");
    //         } else {
    //             print!("{} ", ep[i][j]);
    //         }
    //     }
    //     println!();
    // }



    for x in 1..=M{
        let ans = dp[N][x].min(ep[N][x]);
        if ans == inf {
            println!("{}", -1);
        } else {
            println!("{}", ans);
        }
    }
}
pub trait Change<T: PartialOrd> {
    fn chmin(&mut self, rhs: Self) -> bool;
    fn chmax(&mut self, rhs: Self) -> bool;
}
impl<T: PartialOrd> Change<T> for T {
    fn chmax(&mut self, rhs: Self) -> bool {
        if *self < rhs {
            *self = rhs;
            true
        } else {
            false
        }
    }
    fn chmin(&mut self, rhs: Self) -> bool {
        if *self > rhs {
            *self = rhs;
            true
        } else {
            false
        }
    }
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

