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

    let mut a = vec![vec![];N];
    let mut s = BTreeSet::new();
    for i in 0..N{
        input!{
            l: usize,
            v: [i64;l],
        }
        a[i] = v.clone();
        s.insert(v);
    }
    println!("{}", s.len());



    
}
