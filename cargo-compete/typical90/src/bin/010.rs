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
        CP: [(i64,i64);N],
        Q: usize,
        LR: [(Usize1, usize);Q],
    }

    let s1 = CP
        .iter()
        //.filter(|(c, _)| *c == 1)
        .map(|&(c, s)| if c == 1 { s } else { 0 })
        .collect_vec();
    let s2 = CP
        .iter()
        //.filter(|(c, _)| *c == 2)
        .map(|&(c, s)| if c == 2 { s } else { 0 })
        .collect_vec();

    let mut cum1 = vec![0; s1.len() + 1];
    let mut cum2 = vec![0; s2.len() + 1];

    for i in 0..s1.len() {
        cum1[i + 1] = cum1[i] + s1[i];
    }

    for i in 0..s2.len() {
        cum2[i + 1] = cum2[i] + s2[i];
    }

    for &(l, r) in LR.iter() {
        let a1 = cum1[r.min(cum1.len() - 1)] - cum1[l.min(cum1.len() - 1)];
        let a2 = cum2[r.min(cum2.len() - 1)] - cum2[l.min(cum2.len() - 1)];
        println!("{} {}", a1, a2);
    }
}
