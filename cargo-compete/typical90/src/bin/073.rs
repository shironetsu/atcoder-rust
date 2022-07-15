#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::*;
use maplit::*;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
use std::fmt::Write;
use superslice::{Ext, Ext2};

const MODULO: i64 =  1_000_000_007;

fn dfs(
    u: usize,
    seen: &mut Vec<bool>,
    ad: &Vec<Vec<usize>>,
    col: &Vec<char>,
) -> (i64, i64, i64) {
    let mut aa = 1;
    let mut bb = 1;
    let mut cc = 1;
    for &v in ad[u].iter() {
        if seen[v] {
            continue;
        }
        seen[v] = true;
        let (a, b, c) = dfs(v, seen, ad, col);
        aa *= (a + c).rem_euclid(MODULO);
        bb *= (b + c).rem_euclid(MODULO);
        cc *= (a + b + 2 * c).rem_euclid(MODULO);
        aa = aa.rem_euclid(MODULO);
        bb = bb.rem_euclid(MODULO);
        cc = cc.rem_euclid(MODULO);
    }

    if col[u] == 'a' {
        let ab = (cc-aa).rem_euclid(MODULO);
        (aa, 0, ab)
    } else {
        let ab = (cc-bb).rem_euclid(MODULO);
        (0, bb, ab)
    }
}

#[fastout]
fn main() {
    input! {
        N: usize,
        c: [char;N],
        edges: [(Usize1, Usize1);N-1],
    }

    let mut ad = vec![vec![]; N];
    for &(a, b) in edges.iter() {
        ad[a].push(b);
        ad[b].push(a);
    }

    let mut seen = vec![false; N];
    seen[0] = true;
    let (_, _, ans) = dfs(0, &mut seen, &ad, &c);

    println!("{}", ans % MODULO);
}