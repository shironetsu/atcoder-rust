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
        a: i64,
        b: i64,
        mut A: [i64;N],
    }
    A.sort();

    let mut B = vec![0;N+1];
    for i in 0..N{
        B[i+1] = B[i] + A[i];
    }

    

    
}
