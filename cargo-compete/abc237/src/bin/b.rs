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
        H: usize,
        W: usize,
        A: [[i32;W];H],
    }

    for j in 0..W{
        let line = (0..H).map(|i|A[i][j].to_string()).join(" ");
        println!("{}", line);
    }

    
}
