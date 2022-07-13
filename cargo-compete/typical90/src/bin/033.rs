#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::*;
use maplit::*;
use num_integer::div_ceil;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
use std::fmt::Write;
use superslice::{Ext, Ext2};

#[fastout]
fn main() {
    input! {
        H: usize,
        W: usize,
    }

    if H.min(W) == 1 {
        println!("{}", H.max(W));
        return;
    }

    let H = H + (H & 1);
    let W = W + (W & 1);
    let ans = (H * W) / 4;
    println!("{}", ans);
}
