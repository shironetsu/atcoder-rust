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
        H: usize,
        W: usize,
        P: [[i32;W];H]
    }

    let mut ans = 0;

    for s in 1i32..1 << H {
        let mut m = BTreeMap::<i32, i32>::new();
        for j in 0..W {
            let mut c = vec![];
            for i in 0..H {
                if (s >> i) & 1 == 1 {
                    c.push(P[i][j]);
                }
            }
            let hom = c.iter().fold(true, |acc, &x| acc && c[0] == x);
            if hom {
                *m.entry(c[0]).or_default() += 1;
            }
        }
        let mm = m.iter().max_by_key(|e| e.1).map(|(_, &v)| v).unwrap_or(0);
        let n = s.count_ones() as usize;
        ans.chmax(n as i32 * mm);
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
