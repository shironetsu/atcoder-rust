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
        K: usize,
        S: [Chars;H],
    }

    let mut SS = vec![vec![0;W];H];
    for i in 0..H{
        for j in 0..W{
            SS[i][j] = (S[i][j] as u8 - b'0') as usize;
        }
    }

    let mut a = vec![vec![0;W+1];H];
    for i in 0..H{
        for j in 0..W{
            a[i][j+1] = a[i][j] + SS[i][j];
        }
    }

    let mut ans = H*W;
    for s in 0..1<<(H-1){
        let mut cut = vec![];
        cut.push(0);
        for i in 1..H{
            if (s>>(i-1))&1 == 1 {
                cut.push(i);
            }
        }
        cut.push(H);
        let n = cut.len() - 1;

        let mut c = vec![vec![0;W+1];n];
        for i in 0..n{
            for j in 0..=W{
                c[i][j] = (cut[i]..cut[i+1]).map(|k|a[k][j]).sum::<usize>();
            }
        }
        let mut p = 0;
        let mut sum = n - 1;
        loop {
            let q = (0..n).map(|i|c[i].lower_bound(&(c[i][p]+K+1))).min().unwrap();
            p = q - 1;
            if p >= W {
                break;
            } else {
                sum += 1;
            }
        }
        ans.chmin(sum);
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

