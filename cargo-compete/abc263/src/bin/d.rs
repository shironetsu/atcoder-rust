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
        L: i64,
        R: i64,
        A: [i64;N],
    }

    let mut B = vec![0;N+1];
    for i in 0..N{
        B[i+1] = B[i] + A[i];
    }

    let mut l = vec![0;N+1];
    for i in 0..=N{
        l[i] = B[i] - L * i as i64;
    }
    let mut r = vec![0;N+1];
    for i in 0..=N{
        r[i] = B[N]-B[i] - R * (N-i) as i64;
    }

    let mut a = vec![0;N+1];
    let mut u = -10i64.pow(18);
    let mut k = 0;
    for i in 0..=N{
        if u.chmax(l[i]){
            k = i;
        }
        a[i] = k;
    }
    let mut b = vec![0;N+1];
    let mut u = -10i64.pow(18);
    let mut k = N;
    for i in (0..=N).rev(){
        if u.chmax(r[i]){
            k = i;
        }
        b[i] = k;
    }

    let mut ans = B[N];
    // println!("{:?}", a);
    // println!("{:?}", b);
    for &x in a.iter(){
        let bns = B[N] - l[x] - r[b[x]];
        ans.chmin(bns);
    }
    ans.chmin(R * N as i64);
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