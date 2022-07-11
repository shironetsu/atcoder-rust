#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::*;
use maplit::*;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
use std::fmt::Write;
use superslice::{Ext, Ext2};

const INF: i64 = 2_000_000_000;

#[fastout]
fn main() {
    input! {
        N: usize,
        AB: [(Usize1, Usize1);N],
    }

    let mut roots = vec![];
    let mut ad = vec![vec![]; N];

    for (i, &(a, b)) in AB.iter().enumerate() {
        ad[a].push(i);
        ad[b].push(i);
        if i == a || i == b {
            roots.push(i);
        }
    }

    let mut d = vec![INF; N];
    for &r in roots.iter() {
        let mut todo = MinHeap::<(i64, usize)>::new();
        todo.push((0, r));
        d[r] = 0;
        loop {
            if let Some((dd, u)) = todo.pop() {
                for &v in ad[u].iter() {
                    if d[v].chmin(dd + 1) {
                        todo.push((dd + 1, v));
                    }
                }
            } else {
                break;
            }
        }
    }

    if let Some(_) = d.iter().find(|&&dd| dd == INF) {
        println!("-1");
    } else {
        let mut o = d.iter().enumerate().map(|(i, &dd)| (dd, i)).collect_vec();
        o.sort();
        for (_, i) in o.iter().rev() {
            println!("{}", i + 1);
        }
    }

    //;
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
