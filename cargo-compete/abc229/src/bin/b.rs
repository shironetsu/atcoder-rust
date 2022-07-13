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
        A: Chars,
        B: Chars,
    }

    let c = A.iter().copied().rev().zip(B.iter().copied().rev()).collect_vec();
    for &(a, b) in c.iter(){
        let a = (a as u8 - b'0') as i32;
        let b = (b as u8 - b'0') as i32;
        if a + b >= 10 {
            println!("Hard");
            return;
        }
    }

    println!("Easy");
}
