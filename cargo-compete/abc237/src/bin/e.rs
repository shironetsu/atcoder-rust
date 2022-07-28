#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::*;
use maplit::*;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
use std::fmt::Write;
use superslice::{Ext, Ext2};

fn get_delta(x: usize, y: usize, H: &Vec<i64>) -> i64 {
    if H[x] > H[y] {
        H[x] - H[y]
    } else if H[x] < H[y] {
        2 * (H[x] - H[y])
    } else {
        0
    }
}

#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        H: [i64;N],
        UV: [(Usize1, Usize1);M],
    }

    let mut ad = vec![vec![]; N];
    for &(u, v) in UV.iter() {
        ad[u].push(v);
        ad[v].push(u);
    }

    let inf = 10i64.pow(18);
    let mut p = vec![-inf; N];
    let mut todo = BinaryHeap::<(i64, usize, usize)>::new();
    let start = 0;
    p[start] = 0;
    for &u in ad[start].iter() {
        let d = get_delta(start, u, &H);
        todo.push((d, start, u));
    }

    loop {
        if let Some((delta, u, v)) = todo.pop() {
            let pp = p[u] + delta;
            if p[v].chmax(pp) {
                for &w in ad[v].iter() {
                    let delta = get_delta(v, w, &H);
                    todo.push((delta, v, w));
                }
            }
        } else {
            break;
        }
    }

    let ans = p.iter().max().unwrap();
    println!("{}", ans);
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
