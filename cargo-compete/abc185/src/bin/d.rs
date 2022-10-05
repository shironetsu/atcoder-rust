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
        N: i64,
        M: usize,
        mut A: [i64;M], //A in [1,N]
    }

    if M == 0 {
        println!("{}", 1);
        return;
    }

    A.sort();
    let A = [&[0], &A[..], &[N+1]].concat();
    let mut B = vec![0;A.len()-1];
    for i in 0..A.len()-1{
        B[i] = A[i+1]-A[i]-1;
    }

    if let Some(&min) = B.iter().filter(|&&b|b!=0).min(){
        let ans = B.iter().map(|&b|num_integer::div_ceil(b, min)).sum::<i64>();
        println!("{}", ans);
    } else {
        println!("{}", 0);
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

