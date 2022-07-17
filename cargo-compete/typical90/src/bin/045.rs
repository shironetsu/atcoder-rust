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
        K: usize,
    }
    let mut X = vec![0i64; N];
    let mut Y = vec![0i64; N];
    for i in 0..N {
        input! {
            x: i64,
            y: i64,
        }
        X[i] = x;
        Y[i] = y;
    }
    let mut edges = MinHeap::<(i64, usize, usize)>::new();
    for i in 0..N {
        for j in i + 1..N {
            let rr = (X[i] - X[j]).pow(2) + (Y[i] - Y[j]).pow(2);
            edges.push((rr, i, j));
        }
    }

    let mut uf = petgraph::unionfind::UnionFind::new(N);
    let mut groups = N;
    let mut d = 0;
    loop {
        if let Some((e, i, j)) = edges.pop() {
            if !uf.equiv(i, j) {
                uf.union(i, j);
                groups -= 1;
            }
            d.chmax(e);
            if groups <= K {
                break;
            }
        } else {
            break;
        }
    }
    println!("{}", d);
}
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