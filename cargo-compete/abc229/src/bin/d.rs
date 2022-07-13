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
        S: Chars,
        K: i64,
    }

    let mut x = vec![0;S.len()+1];
    for i in 0..S.len(){
        if S[i] == '.' {
            x[i+1] = x[i] + 1;
        } else {
            x[i+1] = x[i];
        }
    }

    //println!("{:?}", x);

    let mut l = 0;
    let mut r = 0;
    let mut ans = 0;
    loop {
        while r <= S.len() && x[r]-x[l] <= K {
            ans.chmax(r-l);
            r += 1;
        }
        if r == S.len() + 1{
            break;
        }
        while x[r]-x[l] > K {
            l += 1;
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