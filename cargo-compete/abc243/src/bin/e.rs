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
        M: usize,
        ABC: [(Usize1, Usize1, i64);M],
    }

    let mut edges = vec![];
    for &(a,b,c) in ABC.iter(){
        edges.push((c, a, b));
    }
    edges.sort();

    let inf = 10i64.pow(18);
    let mut dist = vec![vec![inf; N]; N];
    for i in 0..N{
        dist[i][i] = 0;
    }

    for i in 0..M{
        
    }



    let mut ad = vec![vec![]; N];
    for &(a, b, c) in ABC.iter() {
        ad[a].push((b, c));
        ad[b].push((a, c));
    }

    let mut edges = BTreeSet::<(usize, usize)>::new();

    let inf = 10i64.pow(18);
    let mut dist = vec![vec![inf; N]; N];
    for s in 0..N {
        dist[s][s] = 0;
        let mut mh = MinHeap::<(i64, usize, usize)>::new();
        for &(t, c) in ad[s].iter() {
            mh.push((c, s, t));
        }
        loop {
            if let Some((d, u, v)) = mh.pop() {
                if dist[s][v].chmin(d) {
                    edges.insert((u.min(v), v.max(u)));
                    for &(w, c) in ad[v].iter() {
                        mh.push((d + c, v, w));
                    }
                }
            } else {
                break;
            }
        }
        for i in s + 1..N {
            for j in i + 1..N {
                let d = dist[s][i] + dist[s][j];
                dist[i][j].chmin(d);
                dist[j][i].chmin(d);
            }
        }
    }
    let ans = M - edges.len();
    println!("{}", ans);
}
//______________________________________________________________________________
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
//______________________________________________________________________________
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
