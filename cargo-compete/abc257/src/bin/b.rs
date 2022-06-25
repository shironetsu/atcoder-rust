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
        K: usize,
        Q: usize,
        mut A: [Usize1;K],
        L: [Usize1;Q],
    }

    // let mut occupied = vec![false;N];
    // for &a in A.iter(){
    //     occupied[a] = true;
    // }

    for &l in L.iter() {
        if A[l] == N - 1 {
            continue;
        } else {
            if l + 1 < K && A[l + 1] == A[l] + 1 {
                continue;
            } else {
                A[l] += 1;
            }
        }
    }

    let ans = A.iter().map(|&a| (a + 1).to_string()).join(" ");
    println!("{}", ans);
}
