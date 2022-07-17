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

    let mut c = vec![];
    for _ in 0..N {
        input! {
            a: i64,
            b: i64,
        }
        c.push(b);
        c.push(a - b);
    }
    c.sort();
    let ans = c.iter().rev().take(K).sum::<i64>();
    println!("{}", ans);
}
