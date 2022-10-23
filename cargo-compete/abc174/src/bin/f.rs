#![allow(unused_imports)]
#![allow(non_snake_case)]
use proconio::{input, fastout};
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use std::fmt::{Write, Display};
use std::collections::*;
use maplit::*;
use itertools::*;
use superslice::{Ext, Ext2};

#[fastout]
fn main() {
    input!{
        N: usize,
        Q: usize,
        c: [Usize1;N],
        lr: [(Usize1, usize);Q],
    }

    let mut a = vec![0;N+1];
    let mut e = vec![false;N];
    for i in 0..N{
        a[i+1] = if !e[c[i]] {
            e[c[i]] = true;
            a[i]+1
        } else {
            a[i]
        };
    }

    let mut b = vec![0;N+1];
    let mut e = vec![false;N];
    for i in 0..N{
        b[i+1] = if !e[c[N-i-1]] {
            e[c[N-i-1]] = true;
            b[i]+1
        } else {
            b[i]
        };
    }

    let mut ans = vec![0;Q];
    for i in 0..Q{
        let (l, r) = lr[i];
        ans[i] = a[r]-a[l]+b[N-r]-b[N-l];
    }

    ans.ansl();



    
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

