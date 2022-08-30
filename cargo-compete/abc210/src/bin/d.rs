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
        H: usize,
        W: usize,
        C: i64,
        mut A: [[i64;W];H],
    }

    let inf = 3 * 10i64.pow(18);
    let mut ans = inf;

    
    for i in 0..2 {
        let mut b = vec![vec![0; W]; H];
        for i in 0..H {
            for j in 0..W {
                b[i][j] = A[i][j] - C * (i as i64 + j as i64);
            }
        }

        let mut min = vec![vec![inf; W]; H];

        for i in 0..H {
            for j in 0..W {
                if i > 0 {
                    let x = b[i - 1][j].min(min[i - 1][j]);
                    min[i][j].chmin(x);
                }
                if j > 0 {
                    let x = b[i][j - 1].min(min[i][j - 1]);
                    min[i][j].chmin(x);
                }
            }
        }

        for i in 0..H {
            for j in 0..W {
                let cost = A[i][j] + C * (i as i64 + j as i64) + min[i][j];
                ans.chmin(cost);
            }
        }

        for i in 0..H {
            A[i].reverse();
        }
    }

    println!("{}", ans);
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
pub trait Answer {
    fn ans(&self);
    fn ansl(&self);
}

impl<T: Display> Answer for Vec<T> {
    fn ans(&self) {
        let ans = self
            .iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<_>>()
            .join(" ");
        println!("{}", ans);
    }

    fn ansl(&self) {
        let ans = self
            .iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<_>>()
            .join("\n");
        println!("{}", ans);
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
