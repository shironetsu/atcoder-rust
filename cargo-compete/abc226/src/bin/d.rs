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
        xy: [(i64, i64);N],
    }

    let mut s = BTreeSet::<(i64, i64)>::new();
    for (i, &(x0, y0)) in xy.iter().enumerate(){
        for (j, &(x1, y1)) in xy.iter().enumerate(){
            if i == j {
                continue;
            }
            let mut dx = x0-x1;
            let mut dy = y0-y1;
            let g = num_integer::gcd(dx.abs(), dy.abs());
            dx /= g;
            dy /= g;
            s.insert((dx, dy));
        }
    }

    println!("{}", s.len());
}
