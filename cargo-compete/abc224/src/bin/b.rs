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
        H: usize,
        W: usize,
        A: [[i64;W];H],
    }

    for i0 in 0..H{
        for i1 in i0+1..H{
            for j0 in 0..W{
                for j1 in j0+1..W{
                    if A[i0][j0] + A[i1][j1] > A[i0][j1] + A[i1][j0] {
                        println!("No");
                        return;
                    }
                }
            }
        }
    }
    println!("Yes");
}
