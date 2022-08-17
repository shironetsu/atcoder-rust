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
        s: [String;3]
    }

    let a = vec!["ABC", "ARC", "AGC", "AHC"];
    let ans = a
        .into_iter()
        .map(|x| x.to_string())
        .find(|x| !s.contains(&x))
        .unwrap();
    println!("{}", ans);
}
