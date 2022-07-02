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
        K: i32,
    }

    if K < 60 {
        println!("{:02}:{:02}", 21, K);
    } else {
        println!("{:02}:{:02}", 22, K - 60);
    }
}
