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
        H: i64,
        W: i64,
        N: usize,
        AB: [(i64, i64);N],
    }

    let mut h = BTreeSet::new();
    let mut w = BTreeSet::new();
    for (i, &(a, b)) in AB.iter().enumerate(){
        h.insert(a);
        w.insert(b);
    }

    let h = h.into_iter().collect_vec();
    let w = w.into_iter().collect_vec();
    for &(a, b) in AB.iter() {
        let i = h.lower_bound(&a);
        let j = w.lower_bound(&b);
        println!("{} {}", i+1, j+1);
    }



    
}
