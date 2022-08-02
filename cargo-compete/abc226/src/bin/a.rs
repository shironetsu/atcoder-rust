#![allow(unused_imports)]
#![allow(non_snake_case)]
use proconio::{input, fastout};
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use std::fmt::Write;
use std::collections::*;
use maplit::*;
use itertools::*;
use superslice::{Ext, Ext2};

#[fastout]
fn main() {
    input!{
        X: f64,
    }

    let a = X.floor();
    let ans = if X - a < 0.5 {
        a
    } else {
        a + 1.0
    };

    println!("{}", ans);

    
}
