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
        M: i32,
        X: i32,
        T: i32,
        D: i32,
    }

    let ans = if M <= X {
        T - D * (X - M)
    } else {
        T
    };

    println!("{}", ans);
}
