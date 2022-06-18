#![allow(unused_imports)]
#![allow(non_snake_case)]
use maplit::*;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
use std::fmt::Write;

#[fastout]
fn main() {
    input! {
        N: usize,
        mut LR: [(u32,u32);N],
    }

    LR.sort();
    let mut it = LR.iter();
    let (mut a, mut b) = it.next().unwrap();
    for &(c, d) in it {
        if b < c {
            println!("{} {}", a, b);
            a = c;
            b = d;
        } else {
            b = b.max(d);
        }
    }
    println!("{} {}", a, b);
}
