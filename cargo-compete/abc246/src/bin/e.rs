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
        a: Usize1,
        b: Usize1,
        c: Usize1,
        d: Usize1,
        S: [Chars;N],
    }

    let inf = 1_000_000_000usize;
    let mut dist = vec![vec![[inf; 4]; N]; N];
    dist[a][b] = [0, 0, 0, 0];
    let dirs = [(1, 1), (!0, !0), (1, !0), (!0, 1)];
    let mut mh = MinHeap::<(usize, usize, usize, usize)>::new();
    for i in 0..4 {
        dist[a][b][i] = 0;
        mh.push((1, i, a, b));
    }
    loop {
        if let Some((d, prev, x, y)) = mh.pop() {
            for (next, &(dx, dy)) in dirs.iter().enumerate() {
                let x = x + dx;
                let y = y + dy;
                if x >= N || y >= N || S[x][y] == '#' {
                    continue;
                }
                if next == prev && dist[x][y][next].chmin(d) {
                    mh.push((d, next, x, y));
                } else if dist[x][y][next].chmin(d + 1) {
                    mh.push((d + 1, next, x, y));
                }
            }
        } else {
            break;
        }
    }
    let &ans = dist[c][d].iter().min().unwrap();
    if ans != inf {
        println!("{}", ans);
    } else {
        println!("-1");
    }
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
