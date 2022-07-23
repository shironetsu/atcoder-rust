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
        a: i32,
        b: i32,
        c: i32,
        d: i32,
    }

    let ans = 0.max(b.min(d)-a.max(c));
    println!("{}", ans);

    
}
