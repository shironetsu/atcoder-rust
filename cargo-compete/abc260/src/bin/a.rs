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
        S: Chars,
    }

    let mut d = BTreeMap::<char, i32>::new();
    for &c in S.iter(){
        *d.entry(c).or_default() += 1;
    }

    for (c, &count) in d.iter(){
        if count == 1 {
            println!("{}", c);
            return;
        }
    }

    println!("-1");
    
}
