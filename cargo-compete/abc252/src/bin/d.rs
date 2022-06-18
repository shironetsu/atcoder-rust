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
        A: [u128;N],
    }

    let mut map = btreemap!();
    for &a in A.iter() {
        *map.entry(a).or_insert(0) += 1;
    }

    let mut s1 = 0u128;
    let mut s2 = 0u128;
    let mut s3 = 0u128;
    for (_, &m) in map.iter() {
        s1 += m;
        s2 += m * m;
        s3 += m * m * m;
    }
    let ans = (s1 * s1 * s1 + 2 * s3 - 3 * s1 * s2) / 6;
    println!("{}", ans);
}
