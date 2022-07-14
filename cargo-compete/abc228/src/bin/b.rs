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
        X: Usize1,
        A: [Usize1;N],
    }

    let mut v = vec![false;N];
    let mut h = X;
    let mut ans = 0;
    while !v[h] {
        v[h] = true;
        h = A[h];
        ans += 1;
    }

    println!("{}", ans);
}
