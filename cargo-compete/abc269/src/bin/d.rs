#![allow(unused_imports)]
#![allow(non_snake_case)]
use proconio::{input, fastout};
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use std::fmt::{Write, Display};
use std::collections::*;
use maplit::*;
use itertools::*;
use superslice::{Ext, Ext2};
use petgraph::unionfind::UnionFind;

fn ad (a: (i64, i64), b: (i64, i64))->bool{
    let dx = a.0 - b.0;
    let dy = a.1 - b.1;
    match (dx, dy) {
        (1,1)|(-1,-1)|(1,0)|(-1,0)|(0,1)|(0,-1) => true,
        _ => false,
    }
}

#[fastout]
fn main() {
    input!{
        N: usize,
        XY: [(i64, i64);N],
    }

    let mut uf = UnionFind::new(N);
    let mut ans = N;
    for i in 0..N{
        for j in i+1..N{
            if ad(XY[i], XY[j]) && !uf.equiv(i, j) {
                uf.union(i, j);
                ans -= 1;
            }
        }
    }

    println!("{}", ans);
}
//______________________________________________________________________________
//
pub trait Answer {
    fn fmt(&self)->String;
    fn fmtl(&self)->String;
    fn ans(&self);
    fn ansl(&self);
}

impl<T: Display> Answer for Vec<T> {
    fn fmt(&self)->String {
        self
            .iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn ans(&self) {
        println!("{}", self.fmt());
    }

    fn fmtl(&self)->String {
        self
            .iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn ansl(&self) {
        println!("{}", self.fmtl());
    }
}
//______________________________________________________________________________
//
#[macro_export]
macro_rules! input_edges {
    ($n: expr, $m: expr, $edges: tt, $ad: tt) => {
        input! {
            $edges: [(Usize1, Usize1); $m],
        }

        let mut $ad = vec![vec![]; $n];
        for &(a, b) in $edges.iter() {
            $ad[a].push(b);
            $ad[b].push(a);
        }
        let $ad = $ad;
    };
}

