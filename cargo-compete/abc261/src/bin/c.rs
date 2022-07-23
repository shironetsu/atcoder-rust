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

    let mut d = BTreeMap::<String, usize>::new();
    for _ in 0..N{
        input!{
            s: String,
        }

        if let Some(x) = d.get(&s) {
            println!("{}({})", s, x);
        } else {
            println!("{}", s);
        }

        *d.entry(s).or_default() += 1;
    }

    
}
