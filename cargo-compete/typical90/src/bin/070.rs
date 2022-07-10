#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::*;
use maplit::*;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
use std::fmt::Write;
use superslice::{Ext, Ext2};

#[fastout]
fn main() {
    input! {
        N: usize,
    }

    let mut x = vec![0; N];
    let mut y = vec![0; N];
    for i in 0..N {
        input! {
            xx: i64,
            yy: i64,
        }

        x[i] = xx;
        y[i] = yy;
    }

    if N == 1 {
        println!("0");
        return;
    }

    x.sort();
    y.sort();

    let mut cx = vec![0; N + 1];
    let mut cy = vec![0; N + 1];

    for i in 0..N {
        cx[i + 1] = cx[i] + x[i];
        cy[i + 1] = cy[i] + y[i];
    }

    let mut ans = std::i64::MAX;

    for m in 0..N - 1 {
        let a = cx[m] - cx[0];
        let b = cx[N] - cx[m];
        let c = cy[m] - cy[0];
        let d = cy[N] - cy[m];
        let t = 2 * m as i64 - N as i64;
        let dist = if t < 0 {
            let mx = x[m];
            let my = y[m];
            t * (mx + my) - a + b - c + d
        } else {
            let mx = x[m - 1];
            let my = y[m - 1];
            t * (mx + my) - a + b - c + d
        };
        ans.chmin(dist);
    }

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
