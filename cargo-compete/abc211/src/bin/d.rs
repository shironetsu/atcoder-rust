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
    }

    input_edges!(N, M, edges, ad);

    let mut d = vec![2*10i64.pow(5)+5;N];
    let mut seen = vec![false;N];
    let mut ord = vec![];
    let start = 0;
    d[start] = 0;
    let mut todo = VecDeque::new();
    todo.push_back(start);
    loop {
        if let Some(u) = todo.pop_front() {
            if seen[u] {
                continue;
            }
            seen[u] = true;
            ord.push(u);
            for &v in ad[u].iter(){
                let dd = d[u] + 1;
                if d[v].chmin(dd){
                    todo.push_back(v);
                }
            }
        } else {
            break;
        }
    }

    let mut pat = vec![0;N];

    let m = 10i64.pow(9) + 7;

    pat[start] = 1;
    for &u in ord.iter() {
        for &v in ad[u].iter(){
            if d[v] < d[u] {
                pat[u] += pat[v];
                pat[u] %= m;
            }
        }
    }

    println!("{}", pat[N-1]);
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
pub trait Answer {
    fn ans(&self);
    fn ansl(&self);
}

impl<T: Display> Answer for Vec<T> {
    fn ans(&self) {
        let ans = self
            .iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<_>>()
            .join(" ");
        println!("{}", ans);
    }

    fn ansl(&self) {
        let ans = self
            .iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<_>>()
            .join("\n");
        println!("{}", ans);
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
