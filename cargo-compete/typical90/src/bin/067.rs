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
        mut N: String,
        K: usize,
    }

    if N == "0" {
        println!("{}", N);
        return;
    }

    for _ in 0..K{
        let N10 = i64::from_str_radix(&N, 8).unwrap();
        N = num_bigint::BigInt::from(N10).to_str_radix(9).chars().map(|c| if c == '8' { '5' } else { c }).collect::<String>();
    }

    println!("{}", N);

    // for _ in 0..K{
    //     let mut N10 = i64::from_str_radix(&N, 8).unwrap();
    //     let mut c =  vec![];
    //     while N10 > 0 {
    //         let r = N10 % 9;
    //         N10 /= 9;
    //         c.push((b'0' + r as u8) as char);
    //     }
    //     let N9 = c.iter().rev().collect::<String>();
    //     N = N9.chars().map(|c| if c == '8' { '5' } else { c }).collect::<String>();
    // }

    // println!("{}", N);
}
