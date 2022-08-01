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
        K: i64,
        mut A: [i64;N],
    }

    A.sort();

    let mut l = 0;
    let mut r = A.iter().sum::<i64>()/K + 1;
    while (l - r).abs() > 1 {
        let m = (l + r)/2;
        let mut u = 0;
        for &a in A.iter(){
            u += a.min(m);
        }
        if u >= m * K {
            l = m;
        } else {
            r = m;
        }
    }
    println!("{}", l);
}
