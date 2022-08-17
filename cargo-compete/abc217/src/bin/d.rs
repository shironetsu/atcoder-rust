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
        L: i64,
        Q: usize,
        q: [(i32, i64);Q]
    }

    let mut s = BTreeSet::new();
    s.insert(0);
    s.insert(L);
    for &(c, x) in q.iter(){
        if c == 1 {
            s.insert(x);
        } else {
            let l = s.range(..x).last().unwrap();
            let r = s.range(x..).next().unwrap();
            println!("{}", r - l);
        }
    }

    
}
