#![allow(unused_imports)]
#![allow(non_snake_case)]
use proconio::{input, fastout};
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use std::fmt::Write;
use std::collections::*;
use maplit::*;
use itertools::*;
use superslice::{Ext, Ext2};

#[fastout]
fn main() {
    input!{
        N: usize,
        M: usize,
    }

    let mut C = BTreeMap::<(usize, usize), i64>::new();
    let mut ad = vec![vec![];N];
    for _ in 0..M{
        input!{a:Usize1, b:Usize1, c:i64}
        C.insert((a,b), c);
        C.insert((b,a), c);
        ad[a].push(b);
        ad[b].push(a);
    }

    let dmin0 = {
        let mut dmin = vec![std::i64::MAX;N];
        let mut todo =MinHeap::new();
        let mut visited = vec![false;N];
        let start = 0;
        todo.push((0, start));
        loop{
            if let Some((d, u)) = todo.pop(){
                if visited[u] {
                    continue;
                } else {
                    visited[u] = true;
                    dmin[u].chmin(d);
                    for &v in ad[u].iter(){
                        let e = C.get(&(u,v)).unwrap();
                        if !visited[v]{
                            todo.push((d+e , v));
                        }
                    }
                }
            }else{
                break;
            }
        }
        dmin
    };

    let dmin1 = {
        let mut dmin = vec![std::i64::MAX;N];
        let mut todo =MinHeap::new();
        let mut visited = vec![false;N];
        let start = N-1;
        todo.push((0, start));
        loop{
            if let Some((d, u)) = todo.pop(){
                if visited[u] {
                    continue;
                } else {
                    visited[u] = true;
                    dmin[u].chmin(d);
                    for &v in ad[u].iter(){
                        let e = C.get(&(u,v)).unwrap();
                        if !visited[v]{
                            todo.push((d+e , v));
                        }
                    }
                }
            }else{
                break;
            }
        }
        dmin
    };

    for k in 0..N{
        let ans = dmin0[k] + dmin1[k];
        println!("{}", ans);
    }


    
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
