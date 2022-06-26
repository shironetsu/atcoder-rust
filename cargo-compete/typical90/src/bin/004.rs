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

    let mut R = vec![0;H];
    let mut C = vec![0;W];
    for i in 0..H{
        for j in 0..W{
            R[i] += A[i][j];
            C[j] += A[i][j];
        }
    }

    //let mut B =vec![vec![0;W];H];

    for i in 0..H{
        let mut row = vec![];
        for j in 0..W{
            let b = R[i] + C[j] - A[i][j];
            row.push(b);
        }
        let row = row.into_iter().map(|x|{x.to_string()}).join(" ");
        println!("{}", row);
    }

    
}
