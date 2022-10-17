#![allow(unused_imports)]
#![allow(non_snake_case)]
use proconio::{input, fastout};
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use std::fmt::{Write, Display};
use std::collections::*;
use maplit::*;
use itertools::*;
use superslice::{Ext, Ext2};

#[fastout]
fn main() {
    input!{
        H: usize,
        W: usize,
        Ch: Usize1,
        Cw: Usize1,
        Dh: Usize1,
        Dw: Usize1,
        S: [Chars;H],
    }

    let inf = 10i64.pow(18);

    let mut dist = vec![vec![inf;W];H];
    dist[Ch][Cw] = 0;
    let mut todo = MinHeap::new();
    todo.push((0, Ch, Cw));
    loop {
        if let Some((dd, x, y)) = todo.pop(){
            for &(dx, dy) in [(1, 0), (!0, 0), (0, 1), (0, !0)].iter(){
                if x + dx < H && y + dy < W {
                    let x1 = x + dx;
                    let y1 = y + dy;
                    if S[x1][y1] == '#' {
                        continue;
                    }
                    if S[x1][y1] == '.' {
                        if dist[x1][y1].chmin(dd) {
                            todo.push((dist[x1][y1], x1, y1));
                        }
                    }
                }
            }

            for &(dx, dy) in [
                (2, 2), (1, 2), (0, 2), (!0, 2), (!1, 2),
                (2, 1), (1, 1),          (!0,1),  (!1, 1),
                (2, 0),                          (!1, 0),
                (2,!0), (1, !0),        (!0, !0),(!1,!0),
                (2,!1), (1,!1), (0,!1), (!0,!1), (!1,!1),
            ].iter(){
                if x + dx < H && y + dy < W {
                    let x1 = x + dx;
                    let y1 = y + dy;
                    if S[x1][y1] == '#' {
                        continue;
                    }
                    if S[x1][y1] == '.' {
                        if dist[x1][y1].chmin(dd+1) {
                            todo.push((dist[x1][y1], x1, y1));
                        }
                    }
                }
            }
        } else {
            break;
        }
    }

    let ans = if dist[Dh][Dw] == inf { -1 } else { dist[Dh][Dw] };
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
//______________________________________________________________________________
//
pub trait ToString {
    fn to_string(&self) -> String;
}

impl ToString for [char] {
    fn to_string(&self) -> String {
        self.iter().collect::<String>()
    }
}
//______________________________________________________________________________
//
pub trait Answer {
    fn fmt(&self)->String;
    fn fmtl(&self)->String;
    fn ans(&self);
    fn ansl(&self);
}

impl<T: Display> Answer for Vec<T> {
    fn fmt(&self)->String {
        self
            .iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn ans(&self) {
        println!("{}", self.fmt());
    }

    fn fmtl(&self)->String {
        self
            .iter()
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

