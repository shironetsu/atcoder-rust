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
        A: [i32;N],
    }

    let mut sum = A.iter().sum::<i32>();
    let mut P = 0;
    if sum >= 4 {
        P += 1;
    }
    for a in A.iter() {
        sum -= a;
        if sum >= 4 {
            P += 1;
        }
    }

    println!("{}", P);
}
