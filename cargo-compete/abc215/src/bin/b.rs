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
        N: i64,
    }

    let mut k = 0;
    while 1 << (k + 1) <= N {
        k += 1;
    }
    println!("{}", k);
}
