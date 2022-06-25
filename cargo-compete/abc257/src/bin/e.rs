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
        N: i64,
    }

    let mut C = vec![0; 10];
    for i in 1..=9 {
        input! {
            c: i64,
        }
        C[i] = c;
    }

    let &Cmin = C.iter().skip(1).min().unwrap();
    let size = N / Cmin;

    //println!("{} {}", Cmin, size);

    let mut res = N;
    let mut ans = vec![0; size as usize];
    for i in 0..size {
        for j in (1..=9).rev() {
            if res - C[j] >= (size - i - 1) * Cmin {
                ans[i as usize] = j;
                res -= C[j];
                break;
            }
        }
    }

    let ans = ans
        .into_iter()
        .map(|x| ('0' as u8 + x as u8) as char)
        .collect::<String>();

    println!("{}", ans);
}
