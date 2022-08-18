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
        names: [(String, String);N],
    }

    let mut m = BTreeSet::new();
    for st in names.iter(){
        if !m.insert(st) {
            println!("Yes");
            return;
        }
    }

    println!("No");

    
}
