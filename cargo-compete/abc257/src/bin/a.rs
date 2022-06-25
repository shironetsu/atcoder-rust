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
        N: i32,
        X: i32,
    }

    let n = (X - 1) / N;
    let c = ('A' as u8 + n as u8) as char;
    println!("{}", c);
}
