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
        edges: [(Usize1, Usize1);M],
    }

    let mut uf = UnionFind::<usize>::new(N);
    let mut degrees = vec![0;N];

    for &(a, b) in edges.iter(){
        if uf.equiv(a, b) {
            println!("No");
            return;
        }else if degrees[a] >= 2 || degrees[b] >= 2 {
            println!("No");
            return;
        }else{
            degrees[a] += 1;
            degrees[b] += 1;
            uf.union(a, b);
        }
    }

    println!("Yes");
}