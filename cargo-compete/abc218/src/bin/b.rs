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
        P: [Usize1;26],
    }

    let ans = P.into_iter().map(|i| ('a' as u8 + i as u8) as char).collect::<String>();
    println!("{}", ans);

    
}
