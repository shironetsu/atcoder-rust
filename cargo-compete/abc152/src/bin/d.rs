#![allow(unused_imports)]
#![allow(non_snake_case)]
use proconio::{input, fastout};
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use std::fmt::{Write, Display};
use std::collections::*;
use maplit::*;
use itertools::*;
use superslice::{Ext, Ext2};

fn to_vec(n: i64)->Vec<i64>{
    let mut n = n;
    let mut v = vec![];
    while n > 0 {
        let (q, r) = num_integer::div_rem(n, 10);
        v.push(r);
        n = q;
    }
    v.reverse();
    v
}

#[fastout]
fn main() {
    input!{
        N: i64,
    }

    let mut v = vec![vec![vec![];10];10];
    for n in 1..=N{
        let u = to_vec(n);
        let s = u[0] as usize;
        let t = u[u.len()-1] as usize;
        v[s][t].push(n);
    }

    let mut ans = 0i64;
    for n in 1..=N{
        let u = to_vec(n);
        let s = u[0] as usize;
        let t = u[u.len()-1] as usize;
        let r = v[t][s].upper_bound(&N);
        ans += r as i64;
    }

    println!("{}", ans);

}
//______________________________________________________________________________
//
pub trait ToString {
    fn to_string(&self) -> String;
}

impl ToString for [char] {
    fn to_string(&self) -> String {
        self.iter().collect::<String>()
    }
}
//______________________________________________________________________________
//
pub trait Answer {
    fn fmt(&self)->String;
    fn fmtl(&self)->String;
    fn ans(&self);
    fn ansl(&self);
}

impl<T: Display> Answer for Vec<T> {
    fn fmt(&self)->String {
        self
            .iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn ans(&self) {
        println!("{}", self.fmt());
    }

    fn fmtl(&self)->String {
        self
            .iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn ansl(&self) {
        println!("{}", self.fmtl());
    }
}
//______________________________________________________________________________
//
#[macro_export]
macro_rules! input_edges {
    ($n: expr, $m: expr, $edges: tt, $ad: tt) => {
        input! {
            $edges: [(Usize1, Usize1); $m],
        }

        let mut $ad = vec![vec![]; $n];
        for &(a, b) in $edges.iter() {
            $ad[a].push(b);
            $ad[b].push(a);
        }
        let $ad = $ad;
    };
}

