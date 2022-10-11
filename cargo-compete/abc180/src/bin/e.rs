#![allow(unused_imports)]
#![allow(non_snake_case)]
use proconio::{input, fastout};
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use std::fmt::{Write, Display};
use std::collections::*;
use maplit::*;
use itertools::*;
use superslice::{Ext, Ext2};

fn cost(x0: (i64, i64, i64), x1: (i64, i64, i64), )->i64{
    let (a, b, c) = x0;
    let (p, q, r) = x1;
    (p-a).abs() + (q-b).abs() + (r-c).max(0)
}

#[fastout]
fn main() {
    input!{
        N: usize,
        XYZ: [(i64,i64,i64);N],
    }

    let inf = 10i64.pow(18);
    let mut min = vec![vec![inf;N];1<<N];
    min[1][0] = 0;

    for s in 1..1<<N{
        for i in 0..N{
            for j in 0..N{
                if ((s>>i) & 1, (s>>j) & 1) == (1, 0) {
                    let next = min[s][i] + cost(XYZ[i], XYZ[j]);
                    min[s ^ (1<<j)][j].chmin(next);
                }
            }
        }
    }

    let ans = (1..N).map(|i|min[(1<<N)-1][i]+cost(XYZ[i], XYZ[0])).min().unwrap();
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

