#![allow(unused_imports)]
#![allow(non_snake_case)]
use maplit::*;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::cmp::Reverse;
use std::collections::*;
use std::fmt::Write;

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

#[fastout]
fn main() {
    input! {
        H: usize,
        W: usize,
        C: [Chars;H]
    }

    let mut dmax = 0;
    let mut visited = vec![vec![false; W]; H];
    let mut pq = BinaryHeap::new();
    pq.push((Reverse(0), (0, 0)));
    loop {
        if let Some((Reverse(d), (i, j))) = pq.pop() {
            if visited[i][j] {
                continue;
            }
            dmax.chmax(d);
            visited[i][j] = true;
            if i + 1 < H && C[i + 1][j] == '.' {
                pq.push((Reverse(d + 1), (i + 1, j)));
            }
            if j + 1 < W && C[i][j + 1] == '.' {
                pq.push((Reverse(d + 1), (i, j + 1)));
            }
        } else {
            break;
        }
    }
    println!("{}", dmax + 1);
}
