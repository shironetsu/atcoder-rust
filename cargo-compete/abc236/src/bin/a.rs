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
        mut S: Chars,
        a: Usize1,
        b: Usize1,
    }

    let tmp = S[a];
    S[a] = S[b];
    S[b] = tmp;
    let ans = S.iter().collect::<String>();
    println!("{}", ans);
}
