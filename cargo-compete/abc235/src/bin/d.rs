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

fn rotate(x: u64) -> Option<u64> {
    if x >= 10 && x % 10 != 0 {
        let u = x.to_string().len();
        let a = x / 10;
        let b = x % 10;
        let y = b * 10u64.pow(u as u32 - 1) + a;
        Some(y)
    } else {
        None
    }
}

#[fastout]
fn main() {
    input! {
        a: u64,
        N: u64,
    }
    let mut pq = BinaryHeap::<(Reverse<u32>, u64)>::new();
    pq.push((Reverse(0), 1));
    let mut smin = vec![std::u32::MAX; 1_000_000];
    loop {
        if let Some((d, x)) = pq.pop() {
            //println!("{} {}", d.0, x);
            if x == N {
                println!("{}", d.0);
                return;
            }
            if a * x < 1_000_000 {
                if smin[a as usize * x as usize].chmin(d.0 + 1) {
                    pq.push((Reverse(d.0 + 1), a * x));
                }
            }
            let mut y = x;
            for i in 1..x.to_string().len() {
                if let Some(z) = rotate(y) {
                    if smin[z as usize].chmin(d.0 + i as u32) {
                        pq.push((Reverse(d.0 + i as u32), z));
                    }
                    y = z;
                } else {
                    break;
                }
            }
        } else {
            println!("{}", -1);
            break;
        }
    }
}
