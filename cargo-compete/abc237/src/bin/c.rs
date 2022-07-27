#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::*;
use maplit::*;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
use std::fmt::Write;
use superslice::{Ext, Ext2};

#[fastout]
fn main() {
    input! {
        S: Chars,
    }

    let a = {
        let mut i = 0;
        while i < S.len() && S[i] == 'a' {
            i += 1;
        }
        i
    };

    if a == S.len() {
        println!("Yes");
        return;
    }

    let S = S[a..].iter().copied().rev().collect_vec();

    let b = {
        let mut i = 0;
        while i < S.len() && S[i] == 'a' {
            i += 1;
        }
        i
    };

    let S = S[b..].into_iter().collect_vec();

    let n = S.len();
    for i in 0..n{
        if S[i] != S[n-i-1] {
            println!("No");
            return;
        }
    }

    if a <= b {
        println!("Yes");
    } else {
        println!("No");
    }

}
