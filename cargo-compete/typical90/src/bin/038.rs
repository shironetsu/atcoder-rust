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
        A: i128,
        B: i128,
    }
    let (_, ans) = num_integer::gcd_lcm(A, B);
    if ans > 1_000_000_000_000_000_000 {
        println!("Large");
    } else {
        println!("{}", ans);
    }
}
