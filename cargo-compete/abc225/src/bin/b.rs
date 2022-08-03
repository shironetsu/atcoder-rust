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
        ab: [(Usize1, Usize1);N-1],
    }

    let mut degs = vec![0;N];
    for (a, b) in ab {
        degs[a] += 1;
        degs[b] += 1;
    }

    degs.sort();

    let star = (0..N).map(|i| if i < N -1 { 1 } else { N-1 }).collect_vec();

    if degs == star {
        println!("Yes");
    } else {
        println!("No");
    }
    
}
