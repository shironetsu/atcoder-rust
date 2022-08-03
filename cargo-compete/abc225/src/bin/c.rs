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
        M: usize,
        B: [[i64;M];N],
    }

    let mut ok = true;

    let mut org = B[0][0];
    for i in 0..N {
        for j in 0..M {
            if org + i as i64 * 7 + j as i64 != B[i][j] {
                ok = false;
            }
        }
    }

    let a = B[0].iter().map(|&x| (x - 1) % 7).collect_vec();
    if a != sorted(a.clone()).collect_vec() {
        ok = false;
    }

    if ok {
        println!("Yes");
    } else {
        println!("No");
    }
}
