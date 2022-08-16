#![allow(unused_imports)]
#![allow(non_snake_case)]
use proconio::{input, fastout};
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use std::fmt::Write;
use std::collections::*;
use maplit::*;
use itertools::*;
use superslice::{Ext, Ext2};
use petgraph::unionfind::UnionFind;

#[fastout]
fn main() {
    input!{
        N: usize,
        M: usize,
        ABC: [(Usize1, Usize1, i64);M],
    }

    let mut edges = ABC.into_iter().map(|(a, b, c)| (c, a.min(b), a.max(b))).collect_vec();

    edges.sort();
    let mut uf = UnionFind::new(N);
    let mut ans = edges.iter().map(|&(c, _, _)|c).sum::<i64>();
    for &(c, a, b) in edges.iter(){
        if !uf.equiv(a, b) {
            uf.union(a, b);
            ans -= c;
        } else {
            if c <= 0 {
                uf.union(a, b);
                ans -= c;
            }
        }
    }
    println!("{}", ans);
}
