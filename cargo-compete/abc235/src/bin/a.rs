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
        abc: Chars,
    }

    let s = abc
        .iter()
        .map(|&x| (x as u8 - '0' as u8) as i32)
        .sum::<i32>();
    let ans = s * 111;
    println!("{}", ans);
}
