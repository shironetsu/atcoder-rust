#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::*;
use maplit::*;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
use std::fmt::Write;
use superslice::{Ext, Ext2};

#[fastout]
fn main() {
    input! {
        N: usize,
        K: usize,
    }
    let mut X = vec![0i64;N];
    let mut Y = vec![0i64;N];

    for i in 0..N{
        input!{
            x: i64,
            y: i64,
        }
        X[i] = x;
        Y[i] = y;
    }


    // let mut dd = vec![vec![0;N];N];
    // for i in 0..N{
    //     for j in i+1..N{
    //         dd[i][j] = (X[i]-X[j]).pow(2) + (Y[i]-Y[j]).pow();
    //         dd[j][i] = dd[i][j];
    //     }
    // }

    // let mut dmax = vec![0;1<<N];
    // for s in 0..(1<<N){
    //     for i in 0..N{
    //         for j in i+1..N{
    //             if (s>>i) & 1 == 1 && (s>>j) & 1 == 1 {
    //                 dmax.chmax(dd[i][j]);
    //             }
    //         }
    //     }
    // }

    // let mut dp = vec![vec![std::i64::MAX;1<<N];K+1];

    // for k in 1..=K{
    //     for s in 0..1<<N{
    //         for t in 
    //     }
    // }

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