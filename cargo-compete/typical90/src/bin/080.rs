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
        D: usize,
        A: [i64;N],
    }

    let mut p = 0i64;
    for s in 0..1i64<<N{
        if s == 0 {
            continue;
        }
        let mut a = 0i64;
        let mut sgn = -1;
        for i in 0..N{
            if (s>>i) & 1 == 1 {
                sgn *= -1;
                a |= A[i];
            }
        }
        let e = a.count_ones();
        p += sgn * (1<<(D-e as usize));
    }

    let ans = (1i64<<D) - p;
    println!("{}", ans);
}
