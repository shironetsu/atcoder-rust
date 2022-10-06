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
        T: i64,
        A: [i64;N],
    }

    let a = &A[0..N/2];
    let b = &A[N/2..N];

    let mut p = Vec::with_capacity(1<<a.len());
    let mut q = Vec::with_capacity(1<<b.len());

    for s in 0..1<<a.len(){
        let sum = (0..a.len()).filter(|&i|(s>>i)&1==1).map(|i|a[i]).sum::<i64>();
        p.push(sum);
    }
    for t in 0..1<<b.len(){
        let sum = (0..b.len()).filter(|&i|(t>>i)&1==1).map(|i|b[i]).sum::<i64>();
        q.push(sum);
    }

    p.sort();
    q.sort();

    let mut ans = 0;
    for &x in p.iter(){
        let y = T - x;
        let i = q.upper_bound(&y);
        if 0 < i {
            ans.chmax(x + q[i-1]);
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

