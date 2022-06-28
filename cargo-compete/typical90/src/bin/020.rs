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
        a: i64,
        b: i64,
        c: i64,
    }

    if a < c.pow(b as u32) {
        println!("Yes");
    } else {
        println!("No");
    }
}
