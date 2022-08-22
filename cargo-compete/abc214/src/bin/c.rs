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
        N: usize,
        S: [i64;N],
        T: [i64;N],
    }

    let mut ans = vec![0;N];
    let mut SS = vec![0;N+1];
    for i in 0..N{
        SS[i+1] = SS[i] + S[i];
    }

    let mut t0 = T[0];
    for i in 1..=N-1{
        let t = T[i] + SS[N] - SS[i];
        t0.chmin(t);
    }
    
    ans[0] = t0;
    for i in 1..=N-1{
        ans[i] = T[i].min(ans[i-1]+S[i-1]);
    }

    for i in 0..N{
        println!("{}", ans[i]);
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