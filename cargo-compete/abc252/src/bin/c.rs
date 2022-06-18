#![allow(unused_imports)]
#![allow(non_snake_case)]
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
        S: [String;N],
    }

    let mut tmin = std::i32::MAX;
    for i in 0..=9 {
        let mut count = vec![0; 10];
        for j in 0..N {
            let (k, _) = S[j]
                .chars()
                .enumerate()
                .find(|&(_, c)| c as usize - '0' as usize == i)
                .unwrap();
            count[k] += 1;
        }
        let mut t = 0;
        for (i, &c) in count.iter().enumerate() {
            if c == 0 {
                continue;
            }
            let t0 = i + 10 * (c - 1);
            t.chmax(t0);
        }
        tmin.chmin(t as i32);
    }
    println!("{}", tmin);
}
