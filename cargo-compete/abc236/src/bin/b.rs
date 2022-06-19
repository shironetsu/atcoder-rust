#![allow(unused_imports)]
#![allow(non_snake_case)]
use maplit::*;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
use std::fmt::Write;

#[fastout]
fn main() {
    input! {
        N: usize,
        A: [i64;4*N-1],
    }
    let sum = A.iter().sum::<i64>();
    let N = N as i64;
    let ans = 4 * N * (N + 1) / 2 - sum;
    println!("{}", ans);
}
