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
        S: i64,
        T: i64,
    }

    let mut ans = 0;
    for a in 0..=S{
        for b in 0..=S-a{
            for c in 0..=S-a-b{
                if a * b * c <= T{
                    ans += 1;
                }
            }
        }
    }

    println!("{}", ans);

    
}
