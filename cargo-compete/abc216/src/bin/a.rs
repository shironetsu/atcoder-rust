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
        x: String,
    }

    let mut it = x.split('.');
    let x = it.next().unwrap();
    let y = it.next().unwrap().parse().unwrap();
    let s = match y {
        0 | 1 | 2 => "-",
        3 | 4 | 5 | 6 => "",
        _ => "+",
    };
    println!("{}{}", x, s);
}
