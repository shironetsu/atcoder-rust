#![allow(unused_imports)]
#![allow(non_snake_case)]
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
use std::fmt::Write;

#[fastout]
fn main() {
    input! {
        N: usize,
        L: i64,
        A: [i64;N],
    }

    let mut mh = MinHeap::<i64>::new();
    for &a in A.iter() {
        mh.push(a);
    }
    let mut b = L - A.iter().sum::<i64>();
    if b > 0 {
        mh.push(b);
    }
    let mut ans = 0;
    while mh.len() > 1 {
        let a = mh.pop().unwrap();
        let b = mh.pop().unwrap();
        let c = a + b;
        ans += c;
        mh.push(c);
    }

    println!("{}", ans);
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
