#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::*;
use maplit::*;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
use std::fmt::Write;
use superslice::{Ext, Ext2};

fn dist(a: i32, b: i32, c: i32, x: i32) -> i32 {
    let mut d = 0;
    let p = x / (a + c);
    d += p * a * b;
    let q = x - p * (a + c);
    if q <= a {
        d += q * b;
    } else {
        d += a * b;
    }
    d
}

#[fastout]
fn main() {
    input! {
        A: i32,
        B: i32,
        C: i32,
        D: i32,
        E: i32,
        F: i32,
        X: i32,
    }

    let t = dist(A, B, C, X);
    let a = dist(D, E, F, X);

    if t < a {
        println!("Aoki");
    } else if t > a {
        println!("Takahashi");
    } else {
        println!("Draw");
    }
}
