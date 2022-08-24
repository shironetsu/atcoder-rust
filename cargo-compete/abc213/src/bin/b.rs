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
        N: usize,
        A: [i32;N]
    }

    let mut A = A.into_iter().enumerate().collect_vec();
    A.sort_by_key(|t|t.1);
    let i = A[N-2].0 + 1;
    println!("{}", i);

    
}
