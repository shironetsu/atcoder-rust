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
        mut A: [i64;N],
        Q: usize,
        B: [i64;Q],
    }

    A.sort();

    for b in B {
        let l = A.lower_bound(&b);
        //let u = A.upper_bound(&b);
        let mut ans = std::i64::MAX;
        if l < N {
            ans.chmin((A[l] - b).abs());
        }
        if 1 <= l {
            ans.chmin((A[l - 1] - b).abs());
        }
        println!("{}", ans);
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
