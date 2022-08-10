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
        A: [i64;N],
        X: i64,
    }

    let mut cum = vec![0;N+1];
    for i in 0..N{
        cum[i+1] = cum[i] + A[i];
    }

    let (q, r) = num_integer::div_rem(X, cum[N]);
    let i = cum.upper_bound(&r);
    let ans = q * N as i64 + i as i64;
    println!("{}", ans);
}
