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
        H: [u64;N],
    }
    let mut ans = H[0];
    for i in 0..N - 1 {
        if H[i] < H[i + 1] {
            ans = H[i + 1];
        } else {
            break;
        }
    }
    println!("{}", ans);
}
