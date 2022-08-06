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
    let mut P = vec![0;N];
    for i in 1..N{
        input!{
            x: Usize1,
        }
        P[i] = x;
    }

    let mut ans = 0;
    let mut i = N-1;
    while i != 0 {
        i = P[i];
        ans += 1;
    }
    println!("{}", ans);    
}
