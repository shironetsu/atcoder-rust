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
        M: usize,
        uv: [(Usize1, Usize1);M],
    }

    let mut ad = vec![vec![];N];
    for &(u, v) in uv.iter(){
        ad[u].push(v);
    }

    
    
}
