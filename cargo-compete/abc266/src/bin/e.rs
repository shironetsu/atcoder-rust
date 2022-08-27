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
        N: i32,
    }

    let mut e = 3.5f64;
    for i in 0..N-1{
        let mut sum = 0f64;
        let h = e.floor() as i32;
        for x in 1..=6{
            if x <= h {
                sum += e;
                //println!("{}", e);
            } else {
                sum += x as f64;
                //println!("{}", x);
            }
        }
        e = sum / 6.0;
    }

    println!("{}", e);
}
