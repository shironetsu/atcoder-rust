#![allow(unused_imports)]
#![allow(non_snake_case)]
use maplit::*;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
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
        N: usize,
        X: [Usize1;N],
        C: [u64;N],
    }

    let mut visited = vec![false; N];

    let mut ans = 0;
    for i in 0..N {
        if visited[i] {
            continue;
        } else {
            visited[i] = true;
        }

        let mut a = i;
        let mut b = i;
        loop {
            a = X[a];
            b = X[X[b]];
            if a == b {
                break;
            }
        }

        if a != i && visited[a] {
            continue;
        }

        let mut hmin = std::u64::MAX;
        let mut c = a;
        loop {
            visited[c] = true;
            hmin.chmin(C[c]);
            c = X[c];
            if c == a {
                break;
            }
        }
        //println!("{} {}", c, hmin);
        ans += hmin;
    }

    println!("{}", ans);
}
