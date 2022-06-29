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
        K: i64,
        A: [i64;N],
        B: [i64;N],
    }

    let diffsum = A
        .iter()
        .zip(B.iter())
        .map(|(a, b)| (a - b).abs())
        .sum::<i64>();

    if diffsum > K || (K - diffsum) % 2 == 1 {
        println!("No");
    } else {
        println!("Yes");
    }
}
