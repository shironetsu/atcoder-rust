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
        X: i64,
        Y: i64,
        N: i64,
    }
    
    let ans = if 3 * X <= Y {
        N * X
    } else {
        let q = N / 3;
        let r = N - 3 * q;
        q * Y + r * X
    };
    
    println!("{}", ans);
}
