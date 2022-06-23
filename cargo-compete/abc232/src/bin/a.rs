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
        S: Chars,
    }
    let a = (S[0] as i32 - '0' as i32) as i32;
    let b = (S[2] as i32 - '0' as i32) as i32;
    let ans = a * b;
    println!("{}", ans);
}
