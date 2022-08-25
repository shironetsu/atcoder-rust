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
        A: i32,
        B: i32,
    }

    let ans = if A > 0 && B == 0 {
        "Gold"
    } else if A == 0 && B > 0 {
        "Silver"
    } else {
        "Alloy"
    };

    println!("{}", ans);
}
