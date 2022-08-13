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
        H: usize,
        W: usize,
        R: [i64;H],
        C: [i64;W],
        A: [[i32;W];H],
    }

    for c in [0,1].iter(){
        let mut dp = vec![vec![vec![10i64.pow(18);4];W];H];
        let mut todo = VecDeque::new();
        todo.push_back((0,0));
        loop {
            if let Some((i, j)) = todo.pop_front(){
                for k in 0..4{
                    dp[i][j][k] = 
                }
            }
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
