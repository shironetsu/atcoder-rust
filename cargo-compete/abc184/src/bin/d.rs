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
        A: usize,
        B: usize,
        C: usize,
    }

    let M = 100;
    let mut dp = vec![vec![vec![0f64;M+1];M+1];M+1];
    for i in 0..=M{
        for j in 0..=M{
            for k in 0..=M{
                if i.max(j).max(k) == M {
                    dp[i][j][k] = 0f64;
                }
            }
        }
    }

    for i in (0..M).rev(){
        for j in (0..M).rev(){
            for k in (0..M).rev(){
                let m = i + j + k;
                dp[i][j][k] = (i as f64/m as f64) * dp[i+1][j][k] 
                            + (j as f64/m as f64) * dp[i][j+1][k]
                            + (k as f64/m as f64) * dp[i][j][k+1] 
                            + 1f64;
            }
        }
    }

    println!("{}", dp[A][B][C]);


    
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

