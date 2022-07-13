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
        H: [i32;N],
    }

    let mut h = H[0];
    for i in 0..N - 1 {
        if H[i] < H[i + 1] {
            h = H[i + 1];
        } else {
            break;
        }
    }
    println!("{}", h);
}
