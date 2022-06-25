#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::*;
use maplit::*;
use num_integer;
use petgraph::unionfind::UnionFind;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
use std::fmt::Write;
use superslice::{Ext, Ext2};

#[fastout]
fn main() {
    input! {
        N: usize, //200
    }

    let mut x = vec![0; N];
    let mut y = vec![0; N];
    let mut P = vec![0; N];
    for i in 0..N {
        input! {
            x0: i64,
            y0: i64,
            P0: i64,
        }

        x[i] = x0;
        y[i] = y0;
        P[i] = P0;
    }

    let mut Smin = vvec!(0; N, N);
    //let mut edges = BTreeMap::<i64, BTreeSet::<(usize, usize)>>::new();
    for i in 0..N {
        for j in 0..N {
            let d = (x[i] - x[j]).abs() + (y[i] - y[j]).abs();
            Smin[i][j] = num_integer::div_ceil(d, P[i]);
            //edges.entry(Smin[i][j]).or
        }
    }

    let mut ans = std::i64::MAX;

    for i in 0..N {
        let mut s = 0;
        let mut todo = MinHeap::<(i64, usize)>::new();
        let mut visited = vec![false; N];
        let mut count = 0;
        todo.push((s, i));
        loop {
            if let Some((t, j)) = todo.pop() {
                s.chmax(t);
                if visited[j] {
                    continue;
                } else {
                    visited[j] = true;
                    count += 1;
                    if count == N {
                        break;
                    }
                    for k in 0..N {
                        if !visited[k] {
                            todo.push((Smin[j][k], k));
                        }
                    }
                }
            } else {
                break;
            }
        }

        ans.chmin(s);
    }

    println!("{}", ans);
}

#[macro_export]
macro_rules! vvec {
    ($ val : expr ; $ a : expr , $ b : expr ) => {
        vec![vec![$val; $b]; $a]
    };
}
//________________________________________________________________________________
//
use std::cmp::Reverse;
//use std::collections::BinaryHeap;
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
