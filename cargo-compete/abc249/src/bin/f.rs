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

    let mut one = vec![];
    let mut two = vec![];
    let mut y1 = vec![];
    let mut y2 = vec![];
    for i in 0..N {
        input! {
            t: i32,
            y: i64,
        }

        if t == 1 {
            one.push(i);
            y1.push(y);
        } else {
            two.push(i);
            y2.push(y);
        }
    }

    let mut cum = vec![0; two.len() + 1];
    for i in 0..y2.len() {
        cum[i + 1] = cum[i] + y2[i];
    }

    for (i, &i1) in one.iter().rev().enumerate() {
        
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
