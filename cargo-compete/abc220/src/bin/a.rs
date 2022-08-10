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
        A: i32,
        B: i32,
        C: i32,
    }

    if let Some(x) = (A..=B).find(|&x|x%C==0) {
        println!("{}", x);
    } else {
        println!("{}", -1);
    }

    
}
