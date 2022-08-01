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
        S: [i64;N],
    }

    let mut s = BTreeSet::<i64>::new();
    for a in 1..=142{
        for b in 1..=142{
            let ss = 4 * a * b + 3 * a + 3 * b;
            s.insert(ss);
        }
    }

    let mut ans = 0;
    for x in S {
        if !s.contains(&x) {
            ans += 1;
        }
    }
    println!("{}", ans);
}
