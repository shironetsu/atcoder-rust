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
        A: [i64;N],
        B: [i64;N],
        C: [i64;N],
    }

    let mut a = vec![0i64; 46];
    let mut b = vec![0i64; 46];
    let mut c = vec![0i64; 46];
    for i in 0..N {
        a[A[i].rem_euclid(46) as usize] += 1;
        b[B[i].rem_euclid(46) as usize] += 1;
        c[C[i].rem_euclid(46) as usize] += 1;
    }

    let mut d = vec![0i64; 46];
    for i in 0..46 {
        for j in 0..46 {
            for k in 0..46 {
                let n = (i + j + k) % 46;
                d[n] += a[i] * b[j] * c[k];
            }
        }
    }
    println!("{}", d[0]);
}
