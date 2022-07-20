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
        mut v: i32,
        a: i32,
        b: i32,
        c: i32,
    }

    let mut period = a + b + c;
    let v = v % period;
    let ans = if v < a {
        'F'
    } else if v - a < b {
        'M'
    } else {
        'T'
    };
    println!("{}", ans);
}
