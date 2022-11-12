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
        H: usize,
        W: usize,
        A: [[usize;W];H],
    }

    let mut e = vec![vec![false;W];H];
    let mut r = vec![vec![(0, 0);W];H];
    for i in 0..H{
        for j in 0..W {
            if A[i][j] == 0 {
                e[i][j] = true;
            }
        }
    }

    let mut min = vec![None;H];
    let mut max = vec![None;H];
    for i in 0..H{
        min[i] = A[i].iter().copied().filter(|&a|a!=0).min();
        max[i] = A[i].iter().copied().filter(|&a|a!=0).max();
    }

    //println!("{:?}", min);
    //println!("{:?}", max);

    let mut io = vec![0;H*W+2];
    for i in 0..H{
        if let Some(m) = min[i] {
            io[m] += 1;
        }
        if let Some(m) = max[i] {
            io[m+1] -= 1;
        }
    }
    for i in 1..=H*W{
        io[i] += io[i-1];
    }

    //println!("{:?}", io);

    for &x in io.iter(){
        if x > 1 {
            println!("No");
            return;
        }
    }
    
    println!("Yes");
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

