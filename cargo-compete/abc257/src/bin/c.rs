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
        S: Chars,
        W: [i64;N],
    }

    let mut CC = vec![];
    let mut AA = vec![];
    for i in 0..N {
        if S[i] == '0' {
            CC.push(2 * W[i]);
        } else {
            AA.push(2 * W[i]);
        }
    }

    CC.sort();
    AA.sort();

    let mut WW = BTreeSet::<i64>::new();
    for &w in W.iter() {
        WW.insert(2 * w);
        WW.insert(2 * w - 1);
        WW.insert(2 * w + 1);
    }

    let mut ans = 0;
    for &ww in WW.iter() {
        let mut cok = CC.lower_bound(&ww);
        let mut aok = AA.len() - AA.upper_bound(&ww);
        ans.chmax(cok + aok);
    }

    println!("{}", ans);
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
