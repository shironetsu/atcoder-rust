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
        X: i64,
    }
    let mut A = vec![];
    let mut B = vec![];
    for i in 0..N {
        input! {
            a: i64,
            b: i64,
        }
        A.push(a);
        B.push(b);
    }

    let mut C = vec![0; N + 1];
    for i in 0..N {
        C[i + 1] = C[i] + A[i] + B[i];
    }
    let mut ans = std::i64::MAX;
    for i in 0..N {
        let sum = C[i + 1] + B[i] * (X - (i as i64 + 1));
        ans.chmin(sum);
        if X - (i as i64 + 1) == 0 {
            break;
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
