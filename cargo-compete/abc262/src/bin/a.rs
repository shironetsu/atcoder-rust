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
        Y: i32,
    }

    let mut d = 0;
    loop {
        if (Y+d) % 4 == 2 {
            println!("{}", Y+d);
            return;
        }
        d+=1;
    }

    
}
