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
        N: i64,
        mut ABC: [i64;3],
    }
    ABC.sort();
    let A = ABC[0];
    let B = ABC[1];
    let C = ABC[2];

    let mut ans = std::i64::MAX;

    for z in (0..=9999).rev() {
        let (mut x, mut y) = (0, 0);
        let g = gcd(A, B, &mut x, &mut y);
        //assert_eq!(A*x+B*y, g);
        let r = N - C * z;
        if r % g == 0 {
            x *= r / g;
            //y *= r / g;
            let x = x.rem_euclid(B / g);
            let y = (r - A * x) / B;
            if y < 0 {
                continue;
            }
            ans.chmin(x + y + z);
            //println!("{} {} {} {}", x, y, z, g);
            //println!("{}", ans);
            //return;
        }
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

pub fn gcd(a: i64, b: i64, x: &mut i64, y: &mut i64) -> i64 {
    if a != 0 {
        let mut d = gcd(b % a, a, x, y);
        *y -= (b / a) * *x;
        std::mem::swap(x, y);
        if d < 0 {
            d = -d;
            *x = -*x;
            *y = -*y;
        }
        d
    } else {
        *x = 0;
        *y = 1;
        b
    }
}
