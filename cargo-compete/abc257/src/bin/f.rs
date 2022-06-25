#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::*;
use maplit::*;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
use std::fmt::Write;
use superslice::{Ext, Ext2};

const INFTY: i64 = 1_000_000_000;

#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        edges: [(usize, usize);M],
    }

    let mut ad = vec![vec![]; N];
    let mut p = vec![];
    for &(u, v) in edges.iter() {
        if u != 0 {
            ad[u - 1].push(v - 1);
            ad[v - 1].push(u - 1);
        } else {
            p.push(v - 1);
        }
    }

    let dist0 = {
        let mut dist = vec![INFTY; N];
        let mut mh = MinHeap::<(i64, usize)>::new();
        let mut visited = vec![false; N];

        mh.push((0, 0));
        loop {
            if let Some((d, v)) = mh.pop() {
                if visited[v] {
                    continue;
                } else {
                    visited[v] = true;
                    dist[v].chmin(d);
                    for &w in ad[v].iter() {
                        if !visited[w] {
                            mh.push((d + 1, w));
                        }
                    }
                }
            } else {
                break;
            }
        }
        dist
    };

    let dist1 = {
        let mut dist = vec![INFTY; N];
        let mut mh = MinHeap::<(i64, usize)>::new();
        let mut visited = vec![false; N];

        mh.push((0, N - 1));
        loop {
            if let Some((d, v)) = mh.pop() {
                if visited[v] {
                    continue;
                } else {
                    visited[v] = true;
                    dist[v].chmin(d);
                    for &w in ad[v].iter() {
                        if !visited[w] {
                            mh.push((d + 1, w));
                        }
                    }
                }
            } else {
                break;
            }
        }
        dist
    };

    let mut d0 = INFTY;
    let mut d1 = INFTY;
    for &v in p.iter() {
        d0.chmin(dist0[v]);
        d1.chmin(dist1[v]);
    }

    let mut ans = vec![-1; N];

    for i in 0..N {
        let a = dist0[N - 1];
        let b = d0 + 1 + dist1[i];
        let c = dist0[i] + 1 + d1;
        let d = d0 + 2 + d1;
        let &e = [a, b, c, d].iter().min().unwrap();
        if e < INFTY {
            ans[i] = e;
        }
    }

    let ans = ans.iter().map(|&x| x.to_string()).join(" ");

    println!("{}", ans);
}
//________________________________________________________________________________
//

use std::cmp::Reverse;
use std::collections::BinaryHeap;
impl<T: Ord> MinHeap<T> {
    pub fn new() -> Self {
        let heap = BinaryHeap::new();
        MinHeap { heap }
    }
    pub fn push(&mut self, x: T) {
        self.heap.push(Reverse(x));
    }
    pub fn pop(&mut self) -> Option<T> {
        self.heap.pop().map(|x| x.0)
    }
    pub fn peek(&self) -> Option<&T> {
        self.heap.peek().map(|x| &x.0)
    }
    pub fn len(&self) -> usize {
        self.heap.len()
    }
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }
}
#[derive(Clone, Debug)]
struct MinHeap<T> {
    heap: BinaryHeap<Reverse<T>>,
}
//________________________________________________________________________________
//
pub trait Change<T: PartialOrd> {
    fn chmin(&mut self, rhs: Self) -> bool;
    fn chmax(&mut self, rhs: Self) -> bool;
}
impl<T: PartialOrd> Change<T> for T {
    fn chmax(&mut self, rhs: Self) -> bool {
        if *self < rhs {
            *self = rhs;
            true
        } else {
            false
        }
    }
    fn chmin(&mut self, rhs: Self) -> bool {
        if *self > rhs {
            *self = rhs;
            true
        } else {
            false
        }
    }
}
