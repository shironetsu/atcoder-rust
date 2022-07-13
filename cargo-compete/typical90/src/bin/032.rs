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
        A: [[i32;N];N],
        M: usize,
    }
    let mut exc = btreeset!();
    for i in 0..M {
        input! {
            X: Usize1,
            Y: Usize1,
        }

        exc.insert((X, Y));
    }

    let mut ans = std::i32::MAX;

    let mut p = (0..N).collect_vec();
    loop {
        let mut ok = true;
        for i in 0..N - 1 {
            let mut x = p[i];
            let mut y = p[i + 1];
            if x > y {
                std::mem::swap(&mut x, &mut y);
            }
            if exc.contains(&(x, y)) {
                ok = false;
            }
        }

        if ok {
            let time = (0..N).map(|i| A[p[i]][i]).sum::<i32>();
            ans.chmin(time);
        }

        if !p.next_permutation() {
            break;
        }
    }

    if ans == std::i32::MAX {
        println!("-1");
    } else {
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
