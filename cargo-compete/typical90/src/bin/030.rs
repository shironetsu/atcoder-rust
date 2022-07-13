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

    let mut f = vec![0; N + 1];
    for i in 2..=N {
        if f[i] == 0 {
            for k in (i..=N).step_by(i) {
                f[k] += 1;
            }
        }
    }
    let ans = (2..=N).map(|i| f[i]).filter(|&x| x >= K).count();
    println!("{}", ans);
}
