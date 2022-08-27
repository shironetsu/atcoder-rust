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
        uv: [(Usize1, Usize1);N],
        Q: usize,
        xy: [(Usize1, Usize1);Q],
    }

    let mut deg = vec![0;N];
    let mut ad = vec![vec![];N];
    for &(u, v) in uv.iter(){
        deg[u] += 1;
        deg[v] += 1;
        ad[u].push(v);
        ad[v].push(u);
    }

    let mut e = vec![];
    for u in 0..N{
        if deg[u] == 1 {
            e.push(u);
        }
    }

    let mut todo = e.into_iter().collect::<VecDeque<_>>();
    let mut circ = vec![true;N];
    loop {
        if let Some(u) = todo.pop_front(){
            circ[u] = false;
            for &v in ad[u].iter(){
                deg[v] -= 1;
                if deg[v] == 1 {
                    todo.push_back(v);
                }
            }
        } else {
            break;
        }
    }

    let mut uf = UnionFind::new(N);
    for &(u, v) in uv.iter(){
        if !(circ[u] && circ[v]) {
            uf.union(u, v);
        }
    }

    for &(x, y) in xy.iter(){
        if uf.equiv(x, y) {
            println!("Yes");
        } else {
            println!("No");
        }
    }

    
}
