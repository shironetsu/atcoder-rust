#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::*;
use maplit::*;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
use std::fmt::{Display, Write};
use superslice::{Ext, Ext2};

#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        AB: [(Usize1, Usize1);M],
        K: usize,
        C: [Usize1;K],
    }

    let mut ad = vec![vec![]; N];
    for &(a, b) in AB.iter() {
        ad[a].push(b);
        ad[b].push(a);
    }

    let inf = 10i64.pow(18);
    let mut dist = vec![vec![inf; K]; K];
    for i in 0..K {
        dist[i][i] = 0;
    }
    for i in 0..K {
        for j in i + 1..K {
            let a = C[i];
            let b = C[j];
            let mut todo = VecDeque::new();
            let mut visited = vec![false; N];
            todo.push_front((0, a));
            visited[a] = true;
            'outer: loop {
                if let Some((d, u)) = todo.pop_front() {
                    for &v in ad[u].iter() {
                        if v == b {
                            dist[i][j] = d + 1;
                            dist[j][i] = d + 1;
                            break 'outer;
                        }
                        if !visited[v] {
                            visited[v] = true;
                            todo.push_back((d + 1, v));
                        }
                    }
                } else {
                    break;
                }
            }
        }
    }

    let mut dist2 = vec![vec![inf; K]; 1 << K];
    for i in 0..K {
        dist2[1 << i][i] = 0;
    }
    for s in 1..1 << K {
        for i in 0..K {
            let b = (0..K).filter(|&i| (s >> i) & 1 == 0).collect_vec();
            if (s >> i) & 1 == 1 {
                for &j in b.iter() {
                    let d = dist2[s][i] + dist[i][j];
                    dist2[s + (1 << j)][j].chmin(d);
                }
            }
        }
    }

    let &ans = dist2[(1 << K) - 1].iter().min().unwrap();
    if ans < inf {
        println!("{}", ans + 1);
    } else {
        println!("{}", -1);
    }
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
//______________________________________________________________________________
//
pub trait Answer {
    fn fmt(&self) -> String;
    fn fmtl(&self) -> String;
    fn ans(&self);
    fn ansl(&self);
}

impl<T: Display> Answer for Vec<T> {
    fn fmt(&self) -> String {
        self.iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn ans(&self) {
        println!("{}", self.fmt());
    }

    fn fmtl(&self) -> String {
        self.iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn ansl(&self) {
        println!("{}", self.fmtl());
    }
}
//______________________________________________________________________________
//
#[macro_export]
macro_rules! input_edges {
    ($n: expr, $m: expr, $edges: tt, $ad: tt) => {
        input! {
            $edges: [(Usize1, Usize1); $m],
        }

        let mut $ad = vec![vec![]; $n];
        for &(a, b) in $edges.iter() {
            $ad[a].push(b);
            $ad[b].push(a);
        }
        let $ad = $ad;
    };
}
