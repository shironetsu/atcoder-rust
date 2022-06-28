#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::*;
use maplit::*;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
use std::fmt::Write;
use superslice::{Ext, Ext2};

#[fastout]
fn main() {
    input! {
        N: usize,
        mut A: [i64;N],
        mut B: [i64;N],
    }

    A.sort();
    B.sort();

    let mut ans = 0;
    for i in 0..N {
        ans += (A[i] - B[i]).abs();
    }

    println!("{}", ans);
}
