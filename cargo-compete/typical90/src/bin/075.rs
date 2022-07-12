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

    let mut M = N;
    let mut d = 2;
    let mut c = 0;
    while d * d <= M {
        while M % d == 0 {
            M /= d;
            c += 1;
        }
        d += 1;
    }
    if M > 1 {
        c += 1;
    }

    let ans = ceil_pow2(c);
    println!("{}", ans);
}

fn ceil_pow2(n: usize) -> usize {
    let mut m = 1;
    let mut log = 0;
    while n > m {
        m <<= 1;
        log += 1;
    }
    log
}
