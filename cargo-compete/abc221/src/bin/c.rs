#![allow(unused_imports)]
#![allow(non_snake_case)]
use proconio::{input, fastout};
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use std::fmt::Write;
use std::collections::*;
use maplit::*;
use itertools::*;
use superslice::{Ext, Ext2};

#[fastout]
fn main() {
    input!{
        N: Chars,
    }

    let n = N.len();
    let mut ans = 0;

    for s in 0..1<<n {
        let mut a = vec![];
        let mut b = vec![];
        for k in 0..n{
            if (s>>k) & 1 == 0 {
                a.push(N[k]);
            } else {
                b.push(N[k]);
            }
        }
        if a.len().min(b.len()) == 0 {
            continue;
        }
        a.sort();
        b.sort();
        a.reverse();
        b.reverse();
        if a[0] == '0' || b[0] == '0' {
            continue;
        }
        let a = a.iter().collect::<String>().parse::<i64>().unwrap();
        let b = b.iter().collect::<String>().parse::<i64>().unwrap();
        ans.chmax(a * b);
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