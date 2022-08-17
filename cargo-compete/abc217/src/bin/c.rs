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
        p: [Usize1; N],
    }

    let mut q = vec![0;N];
    for i in 0..N{
        q[p[i]] = i;
    }

    let ans = q.iter().map(|x|(x+1).to_string()).join(" ");
    println!("{}", ans);

    
}
