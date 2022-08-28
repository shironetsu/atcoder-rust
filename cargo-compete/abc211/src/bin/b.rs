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
        mut S: [String;4],
    }

    S.sort();
    let mut T = vec!["H", "2B", "3B", "HR"];
    T.sort();
    if S == T {
        println!("Yes");
    } else {
        println!("No");
    }
}
