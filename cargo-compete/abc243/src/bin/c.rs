#![allow(unused_imports)]
#![allow(non_snake_case)]
use proconio::{input, fastout};
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use std::fmt::Write;
use std::collections::*;
use maplit::*;
use itertools::*;
use superslice::{Ext, Ext2};

#[derive(Clone,Copy,PartialEq,PartialOrd,Eq,Ord,Debug)]
enum Dir{
    Right,
    Left,
}

#[fastout]
fn main() {
    input!{
        n: usize,
        xy: [(i64,i64);n],
        s: Chars,
    }

    let mut l = BTreeMap::<i64, MinHeap::<(i64, Dir)>>::new();
    for (i, &(x, y)) in xy.iter().enumerate(){
        let d = if s[i] == 'L' { Dir::Left } else { Dir::Right };
        l.entry(y).or_insert(MinHeap::new()).push((x, d));
    }

    for (_, y) in l.iter_mut(){
        let mut u = 0;
        //println!("{:?}", y);
        loop {
            if let Some((_, d)) = y.pop(){
                if u == 0 && d == Dir::Right {
                    u = 1;
                } else if u == 1 && d == Dir::Left {
                    println!("Yes");
                    return;
                }
            } else {
                break;
            }
        }
    }
    println!("No");
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
