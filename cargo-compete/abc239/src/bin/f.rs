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
        D: [usize;N],
        AB: [(Usize1, Usize1);M],
    }

    let mut rem = D.clone();
    let mut uf = UnionFind::new(N);
    for &(a, b) in AB.iter(){
        if D[a] == 0 || D[b] == 0 {
            println!("-1");
            return;
        }
        uf.union(a, b);
        D[a] -= 1;
        D[b] -= 1;
    }

    let mut ll = uf.clone().into_labeling();
}