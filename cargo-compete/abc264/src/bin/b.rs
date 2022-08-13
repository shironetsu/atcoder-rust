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
        R: Isize1,
        C: Isize1,
    }

    let d = (R-7).abs().max((C-7).abs());
    let ans = if d % 2 == 0 {
        "white"
    } else{
        "black"
    };
    println!("{}", ans);

    
}
