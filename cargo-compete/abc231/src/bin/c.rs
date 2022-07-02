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
        Q: usize,
        mut A: [i32;N],
        x: [i32;Q],
    }

    A.sort();
    for q in x.iter(){
        let ans = N-A.lower_bound(&q);
        println!("{}", ans);
    }
}
