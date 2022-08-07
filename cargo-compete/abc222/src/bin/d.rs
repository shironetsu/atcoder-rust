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
        a: [usize;N],
        b: [usize;N],
    }

    let mut dp = vec![vec![0;3005];N];
    for k in a[0]..=b[0]{
        dp[0][k] = 1;
    }
    for i in 0..N{
        
    }

    
}
