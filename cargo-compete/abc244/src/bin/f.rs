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
        edges: [(Usize1, Usize1);M],
    }

    let mut ad = vec![vec![]; N];
    for (u, v) in edges {
        ad[u].push(v);
        ad[v].push(u);
    }

    let inf = 10i64.pow(9);
    let mut minpath = vec![inf; 1 << N];
    minpath[0] = 0;
    for i in 0..N {
        let mut minpath_i = vec![inf; 1 << N];
        let mut todo = MinHeap::<(i64, usize, usize)>::new();
        todo.push((1, i, 1 << i));
        //minpath_i[0] = 0;
        minpath_i[1 << i] = 1;
        loop {
            if let Some((d, u, bits)) = todo.pop() {
                for &v in ad[u].iter() {
                    if minpath_i[bits ^ (1 << v)].chmin(d + 1) {
                        todo.push((d + 1, v, bits ^ (1 << v)));
                    }
                }
            } else {
                break;
            }
        }
        for s in 0..1 << N {
            minpath[s].chmin(minpath_i[s]);
        }
    }
    //println!("{:?}", minpath);
    // for i in 0..N {
    //     let mut dp = vec![vec![inf; N]; 1 << N];
    //     dp[0][i] = 0;
    //     let mut todo = vec![i];
    //     loop {
    //         let mut next = false;
    //         let mut todo_next = btreeset!();
    //         for &u in todo.iter() {
    //             for &v in ad[u].iter() {
    //                 for s in 0..1 << N {
    //                     let d = dp[s][u] + 1;
    //                     if dp[s ^ (1 << v)][v].chmin(d) {
    //                         next = true;
    //                         todo_next.insert(v);
    //                     }
    //                 }
    //             }
    //         }
    //         if !next {
    //             break;
    //         }
    //         todo = todo_next.into_iter().collect_vec();
    //     }
    //     for s in 0..1 << N {
    //         let &min = dp[s].iter().min().unwrap();
    //         minpath[s].chmin(min);
    //     }
    // }
    let ans = minpath.iter().sum::<i64>();
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
