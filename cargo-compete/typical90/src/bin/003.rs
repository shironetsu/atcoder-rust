#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::*;
use maplit::*;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
use std::fmt::Write;
use superslice::{Ext, Ext2};

const INF: i32 = 1_000_000;

#[fastout]
fn main() {
    input! {
        N: usize,
        edges: [(Usize1, Usize1);N-1],
    }

    let mut ad = vec![vec![]; N];
    for (a, b) in edges {
        ad[a].push(b);
        ad[b].push(a);
    }

    let r = {
        let mut todo = MinHeap::<(i32, usize)>::new();
        let mut dmin = vec![INF; N];
        let mut visited = vec![false; N];
        todo.push((0, 0));
        loop {
            if let Some((d, u)) = todo.pop() {
                if visited[u] {
                    continue;
                } else {
                    visited[u] = true;
                    dmin[u] = d;
                    for &v in ad[u].iter() {
                        if !visited[v] {
                            todo.push((d + 1, v));
                        }
                    }
                }
            } else {
                break;
            }
        }
        dmin.iter().enumerate().max_by_key(|&(_, d)| d).unwrap().0
    };

    let ans = {
        let mut todo = MinHeap::<(i32, usize)>::new();
        let mut dmin = vec![INF; N];
        let mut visited = vec![false; N];
        todo.push((0, r));
        loop {
            if let Some((d, u)) = todo.pop() {
                if visited[u] {
                    continue;
                } else {
                    visited[u] = true;
                    dmin[u] = d;
                    for &v in ad[u].iter() {
                        if !visited[v] {
                            todo.push((d + 1, v));
                        }
                    }
                }
            } else {
                break;
            }
        }
        dmin.into_iter().enumerate().max_by_key(|&(_, d)| d).unwrap().1
    } + 1;

    println!("{}", ans);
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
    // pub fn peek(&self) -> Option<&T> {
    //     self.heap.peek().map(|x| &x.0)
    // }
    // pub fn len(&self) -> usize {
    //     self.heap.len()
    // }
    // pub fn is_empty(&self) -> bool {
    //     self.heap.is_empty()
    // }
}
#[derive(Clone, Debug)]
struct MinHeap<T> {
    heap: BinaryHeap<Reverse<T>>,
}
