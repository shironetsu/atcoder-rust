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
        xy: [(i64,i64);4],
    }

    let mut x = vec![0;4];
    let mut y = vec![0;4];
    for i in 0..4{
        x[i] = xy[i].0;
        y[i] = xy[i].1;
    }

    for i in 0..4{
        let j = (i+1) % 4;
        let k = (i+2) % 4;
        let (vx, vy) = (x[i]-x[j], y[i]-y[j]);
        let (wx, wy) = (x[k]-x[j], y[k]-y[j]);
        let t = vx * wy - vy * wx;
        if t > 0 {
            println!("No");
            return;
        }
    }

    println!("Yes");

    
}
