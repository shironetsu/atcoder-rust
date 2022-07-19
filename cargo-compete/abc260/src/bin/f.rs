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
        S: usize,
        T: usize,
        M: usize,
        edges: [(Usize1, Usize1);M],
    }

    let mut ad = vec![vec![];S];
    for (u, v) in edges {
        ad[u].push(v);
    }

    let mut t = vec![vec![None;T];T];
    for u0 in 0..S{
        let n = ad[u0].len();
        for i in 0..n{
            let v0 = ad[u0][i];
            for j in i+1..n{
                let v1 = ad[u0][j];
                if let Some(u1) = t[v0-S][v1-S] {
                    println!("{} {} {} {}", u0 + 1, u1 + 1, v0 + 1, v1 + 1);
                    return;
                } else {
                    t[v0-S][v1-S] = Some(u0);
                    t[v1-S][v0-S] = Some(u0);
                }
            }
        }
    }
    println!("-1");
}
