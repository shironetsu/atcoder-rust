#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::*;
use maplit::*;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
use std::fmt::Write;
use superslice::{Ext, Ext2};

const INF: i64 = 200000;

#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    let mut p = vec![vec![]; N];
    let mut r = vec![vec![]; M];
    for i in 0..M {
        input! {
            k: usize,
            rr: [Usize1;k],
        }
        r[i] = rr;
        for &x in r[i].iter() {
            p[x].push(i);
        }
    }

    let mut todo = VecDeque::<(i64, usize)>::new();
    let mut used = vec![false;M];
    let mut d = vec![INF; N];
    todo.push_back((0i64, 0usize));
    d[0] = 0;
    loop {
        if let Some((dd, i)) = todo.pop_front() {
            for &j in p[i].iter() {
                if used[j] {
                    continue;
                }
                used[j] = true;
                for &k in r[j].iter() {
                    if d[k].chmin(dd + 1) {
                        todo.push_back((dd + 1, k));
                    }
                }
            }
        } else {
            break;
        }
    }
    let mut ans = d.into_iter().map(|x| if x == INF { "-1".to_string() } else {x.to_string()}).join("\n");
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