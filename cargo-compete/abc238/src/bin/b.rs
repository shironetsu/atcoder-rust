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
        A: [i64;N],
    }

    let mut t = 0;
    let mut cuts = vec![t];
    for a in A {
        t += a;
        cuts.push(t % 360);
    }
    cuts.push(360);

    cuts.sort();

    let mut tt = (0..=N).map(|i| cuts[i + 1] - cuts[i]).collect_vec();
    let ans = tt.iter().max().unwrap();
    println!("{}", ans);
}
