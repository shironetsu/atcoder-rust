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

    let n = S.into_iter().collect::<BTreeSet<_>>().len();

    let ans = match n {
        1 => 1,
        2 => 3,
        3 => 6,
        _ => 0,
    };

    println!("{}", ans);
}
