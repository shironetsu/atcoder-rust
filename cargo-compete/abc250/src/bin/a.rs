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
        R: Usize1,
        C: Usize1,
    }

    let mut ans = 0;
    for i in 0..H{
        for j in 0..W{
            if i == R && j == W {
                continue;
            }
            let d = (R as i32-i as i32).abs() + (C as i32-j as i32).abs();
            if d == 1 {
                ans += 1;
            }
        }
    }

    println!("{}", ans);

    
}
