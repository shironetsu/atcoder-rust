#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::*;
use maplit::*;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
use std::fmt::Write;
use superslice::{Ext, Ext2};
use num_integer;

#[fastout]
fn main() {
    input! {
        A: i64,
        B: i64,
        C: i64,
    }

    let g1 = num_integer::gcd(A, B);
    let g = num_integer::gcd(g1, C);
    let ans = (A+B+C)/g - 3;
    println!("{}", ans);
}
