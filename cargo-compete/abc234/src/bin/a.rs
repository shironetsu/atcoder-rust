#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::*;
use maplit::*;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
use std::fmt::Write;
use superslice::{Ext, Ext2};

fn f(x: i64) -> i64 {
    x * x + 2 * x + 3
}

fn g(t: i64) -> i64 {
    f(f(f(t) + t) + f(f(t)))
}

#[fastout]
fn main() {
    input! {
        t: i64,
    }

    println!("{}", g(t));
}
