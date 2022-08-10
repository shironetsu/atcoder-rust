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
        K: u32,
        A: String,
        B: String,
    }

    let A = i64::from_str_radix(&A, K).unwrap();
    let B = i64::from_str_radix(&B, K).unwrap();
    let ans = A * B;
    println!("{}", ans);
    
}
