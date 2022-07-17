#![allow(unused_imports)]
#![allow(non_snake_case)]
use proconio::{input, fastout};
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use std::fmt::Write;
use std::collections::*;
use maplit::*;
use itertools::*;
use superslice::{Ext, Ext2};

#[fastout]
fn main() {
    input!{
        N: usize,
        X: i64,
        Y: i64,
    }

    let mut r = vec![0i64;N+1];
    let mut b = vec![0i64;N+1];
    r[1] = 0;
    b[1] = 1;
    for i in 1..N{
        b[i+1] = r[i] + Y * b[i];
        r[i+1] = r[i] + X * b[i+1]; 
    }
    println!("{}", r[N]);
}
//________________________________________________________________________________
//
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
