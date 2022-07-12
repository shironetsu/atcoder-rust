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
        M: usize,
        edges: [(Usize1, Usize1);M],
    }

    let mut inn = vec![0;N];
    for (a, b) in edges {
        if a < b {
            inn[b] += 1;
        } else if a > b {
            inn[a] += 1;
        }
    }

    let mut ans = inn.iter().filter(|c| **c == 1).count();
    println!("{}", ans);
}
