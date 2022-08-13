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
        H1: usize,
        W1: usize,
        A: [[i64;W1];H1],
        H2: usize,
        W2: usize,
        B: [[i64;W2];H2],
    }

    for s in 0..1 << H1 {
        for t in 0..1 << W1 {
            let mut C = vec![];
            for i in 0..H1 {
                if (s >> i) & 1 == 1 {
                    let mut v = vec![];
                    for j in 0..W1 {
                        if (t >> j) & 1 == 1 {
                            v.push(A[i][j]);
                        }
                    }
                    C.push(v);
                }
            }
            if C == B {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
