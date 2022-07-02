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
        S: Chars,
    }

    let mut c = 0;
    for _ in 0..Q {
        input! {
            t: i32,
        }
        if t == 1 {
            input! {
                x: usize,
            }
            c += (N - x) % N;
        } else if t == 2 {
            input! {
                x: Usize1,
            }
            let ans = S[(x + c) % N];
            println!("{}", ans);
        }
    }
}
