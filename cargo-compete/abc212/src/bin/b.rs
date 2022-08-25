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
        x :Chars,
    }

    let x = x.into_iter().map(|c| (c as u8 - b'0') as i32).collect_vec();

    if (x[0] == x[1] && x[1] == x[2] && x[2] == x[3])
        || ((x[0] + 1) % 10 == x[1] % 10
            && (x[1] + 1) % 10 == x[2] % 10
            && (x[2] + 1) % 10 == x[3] % 10)
    {
        println!("{}", "Weak");
    } else {
        println!("{}", "Strong");
    }
}
