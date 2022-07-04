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
        S: Chars,
    }

    let mut large = 0;
    let mut small = 0;
    let mut count = btreeset!();
    for &c in S.iter() {
        if 'A' <= c && c <= 'Z' {
            large += 1;
        } else {
            small += 1;
        }
        if count.contains(&c) {
            println!("No");
            return;
        } else {
            count.insert(c);
        }
    }
    if large.min(small) > 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
