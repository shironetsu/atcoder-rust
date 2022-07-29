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
        K: i64,
    }

    let mut ans = format!("{:b}", K).chars().map(|c| if c == '0' { '0' } else { '2' }).collect::<String>();
    println!("{}", ans);


    
}
