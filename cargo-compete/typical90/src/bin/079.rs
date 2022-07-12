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
        H: usize,
        W: usize,
        A: [[i64;W];H],
        B: [[i64;W];H],
    }

    let mut x = vec![vec![0i64;W-1];H-1];
    for i in 0..H-1{
        for j in 0..W-1{
            let mut u = B[i][j]-A[i][j];
            if i + !0 < H - 1 {
                u -= x[i+!0][j];
            }
            if j + !0 < W - 1 {
                u -= x[i][j+!0];
            }
            if i + !0 < H - 1 && j + !0 < W - 1{
                u -= x[i+!0][j+!0];
            }
            x[i][j] = u;
        }
    }

    //println!("{:?}", x);

    for i in 0..H-1{
        let j = W-1;
        if B[i][j] != A[i][j] + x[i][j-1] + if i+!0<H-1 { x[i+!0][j-1] } else { 0 } {
            println!("No");
            return;
        }
    }
    for j in 0..W-1{
        let i = H-1;
        if B[i][j] != A[i][j] + x[i-1][j] + if j+!0 < W-1 { x[i-1][j+!0] } else { 0 } {
            println!("No");
            return;
        }
    }
    if B[H-1][W-1] != A[H-1][W-1] + x[H-2][W-2] {
        println!("No");
        return;
    }

    let mut sum = 0;
    for i in 0..H-1{
        for j in 0..W-1{
            sum += x[i][j].abs();
        }
    }
    println!("Yes");
    println!("{}", sum);
}
