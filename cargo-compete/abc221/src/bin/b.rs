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
        T: Chars,
    }

    let mut u = vec![];
    let mut v = vec![];
    let mut w = vec![];

    let n = S.len();
    for i in 0..n {
        if S[i] != T[i] {
            u.push(S[i]);
            v.push(T[i]);
            w.push(i);
        }
    }

    if u.len() == 0 {
        println!("Yes");
        return;
    }

    if u.len() != 2 {
        println!("No");
        return;
    }

    if (u[0], u[1]) == (v[1], v[0]) && w[0] + 1 == w[1] {
        println!("Yes");
    } else {
        println!("No");
    }
}
