#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::*;
use maplit::*;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
use std::fmt::Write;
use superslice::{Ext, Ext2};

struct UnionFind {
    par: Vec<i64>,
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        UnionFind { par: vec![-1; n] }
    }

    fn find(&self, x: usize) -> usize {
        let y = self.par[x];
        if y < 0 {
            x
        } else {
            self.find(y as usize)
        }
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let x = self.find(x);
        let y = self.find(y);
        if x == y {
            false
        } else {
            let (x, y) = if self.par[x] > self.par[y] {
                (y, x)
            } else {
                (x, y)
            };
            self.par[x] += self.par[y];
            self.par[y] = x as i64;
            true
        }
    }

    fn equiv(&self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    fn size(&self, x: usize) -> usize {
        (-self.par[self.find(x)]) as usize
    }
}

#[fastout]
fn main() {
    input! {
        N: usize,
        mut uvw: [(Usize1, Usize1, i64);N-1],
    }

    uvw.sort_by_key(|t| t.2);

    let mut uf = UnionFind::new(N);
    let mut ans = 0;
    for &(u, v, w) in uvw.iter() {
        ans += w * uf.size(u) as i64 * uf.size(v) as i64;
        uf.union(u, v);
    }

    println!("{}", ans);
}
