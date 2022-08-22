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
        N: i64,
    }

    let ans = if 1 <= N && N <= 125 {
        4
    } else if N <= 211 {
        6
    } else {
        8
    };
    println!("{}", ans);

    
}
