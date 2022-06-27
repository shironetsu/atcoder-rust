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
        mut A: [i64;N],
        mut B: [i64;N],
    }

    A.sort();
    B.sort();

    let mut ans = 0;
    for i in 0..N{
        ans += (A[i]-B[i]).abs();
    }

    println!("{}", ans);

    
}
