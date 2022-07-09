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
        N: usize,
        d: [usize;N],
    }
    let mut ad = vec![vec![]; N];
    let mut w = vec![0; N-1];
    let mut p = vec![];
    for i in 0..N - 1 {
        input! {
            u: Usize1,
            v: Usize1,
            ww: i64,
        }
        ad[u].push(v);
        ad[v].push(u);
        w[i] = ww;
        p.push((ww, (u, v)));
    }

    p.sort();

    let mut d = d;
    let mut ans = 0;
    for i in (0..N - 1).rev() {
        let (ww, (u, v)) = p[i];
        if ww <= 0 {
            break;
        }
        if d[u].min(d[v]) >= 1 {
            d[u] -= 1;
            d[v] -= 1;
            ans += ww;
        }
    }

    println!("{}", ans);
}
