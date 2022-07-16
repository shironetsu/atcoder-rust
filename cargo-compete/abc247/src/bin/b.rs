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
        st: [(String, String);N],
    }

    let mut d = BTreeMap::<String, i32>::new();
    for (s, t) in st.iter(){
        *d.entry(s.clone()).or_default() += 1;
        *d.entry(t.clone()).or_default() += 1;
    }

    for (s, t) in st.iter(){
        let a = *d.get(s).unwrap();
        let b = *d.get(t).unwrap();
        if s == t && a == 2 {
            continue;
        }
        if a.min(b) > 1 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
