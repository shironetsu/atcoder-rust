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
        xy: [(i64, i64);N],
    }

    let s = xy.iter().copied().collect::<BTreeSet<_>>();
    let mut ans = 0;
    for i in 0..N{
        for j in 0..N{
            let (a, b) = xy[i];
            let (c, d) = xy[j];
            if a > c && b > d && s.contains(&(a, d)) && s.contains(&(c, b)) {
                ans += 1;
            }
        }
    }
    
    println!("{}", ans);

    
}
