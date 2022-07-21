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
        N: usize,
    }

    let M = 2 * N;
    println!("{}", M);
    let b = N / 4;
    let a = N % 4;
    let c = (0..b).map(|_|'4').collect::<String>();
    if a == 0 {
        println!("{}", c);
    } else {
        println!("{}{}", a, c);
    }

    
}
