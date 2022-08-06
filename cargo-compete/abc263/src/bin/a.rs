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
        A: [i32;5],
    }

    let mut c = BTreeMap::new();
    for a in A{
        *c.entry(a).or_default() += 1;
    }

    let c = c.into_iter().map(|(_, x)|x).collect::<BTreeSet<_>>();
    if c == btreeset!(2, 3){
        println!("Yes");
    } else {
        println!("No");
    }
}
