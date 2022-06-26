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
    }

    let mut X = vec![0; N];
    let mut Y = vec![0; N];
    for i in 0..N {
        input! {x: i64, y: i64}
        X[i] = x;
        Y[i] = y;
    }

    let x0 = X[0];
    let y0 = Y[0];
    for x in X.iter_mut() {
        *x -= x0;
    }
    for y in Y.iter_mut() {
        *y -= y0;
    }

    let mut SS = 0; //*2
    let mut tt = vec![0;N];
    for i in 0..=N-1{
        tt[i] = X[i]*Y[(i+1) % N] - X[(i+1) % N]*Y[i];
        SS += tt[i];
    }

    println!("{:?}", tt);
    println!("{}", SS);


}
