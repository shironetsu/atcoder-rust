#![allow(unused_imports)]
#![allow(non_snake_case)]
use proconio::{input, fastout};
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use std::fmt::Write;
use std::collections::*;
use maplit::*;
use petgraph::unionfind::UnionFind;

#[derive(PartialOrd, PartialEq, Ord, Eq, Clone, Copy)]
pub enum Edge{
    Real,
    Query(usize),
}

#[fastout]
fn main() {
    input!{
        N: usize,
        M: usize,
        Q: usize,
        abc: [(Usize1, Usize1, i64);M],
        uvw: [(Usize1, Usize1, i64);Q],
    }

    let mut ans = vec![false;Q];
    let mut edges = Vec::<(i64, usize, usize, Edge)>::new();
    for (a, b, c) in abc {
        edges.push((c, a, b, Edge::Real));
    }
    for (i, (u, v, w)) in uvw.into_iter().enumerate(){
        edges.push((w, u, v, Edge::Query(i)));
    }
    edges.sort();
    let mut uf = UnionFind::new(N);
    for &(_w, i, j, t) in edges.iter(){
        if uf.equiv(i, j) {
            match t {
                Edge::Real => (),
                Edge::Query(k) => ans[k] = false,
            }
        } else {
            match t {
                Edge::Real => {uf.union(i, j); ()},
                Edge::Query(k) => ans[k] = true,
            }
        }
    }

    for a in ans {
        if a {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
