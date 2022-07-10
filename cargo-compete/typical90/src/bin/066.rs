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
        LR: [(i32, i32);N],
    }

    let mut v = vec![0f64; 101];
    let mut ans = 0f64;
    for (l, r) in LR {
        let a = (1..=100)
            .map(|i| {
                let c = (i - l).max(0).min(r - l + 1) as f64;
                c * v[i as usize]
            })
            .sum::<f64>();
        let p = 1.0 / (r - l + 1) as f64;
        ans += a * p;
        for i in l..=r {
            v[i as usize] += p;
        }
    }
    println!("{}", ans);
}
