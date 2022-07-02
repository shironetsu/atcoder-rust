#![allow(unused_imports)]
#![allow(non_snake_case)]
use proconio::{input, fastout};
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use std::fmt::Write;
use std::collections::*;
use maplit::*;
use itertools::*;

#[fastout]
fn main() {
    input!{
        D: f64,
    }

    let ans = D/100.0;
    println!("{}", ans);

    
}
