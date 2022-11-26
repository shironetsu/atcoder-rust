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
        M: usize,
        A: [Usize1;M],
    }

    let mut u = (0..N).collect_vec();
    let mut r = (0..N).collect_vec();
    for &a in A.iter(){
        let tmp = u[a];
        u[a] = u[a+1];
        u[a+1] = tmp;
        r[u[a]] = a;
        r[u[a+1]] = a+1;
        println!("{:?}", r);
    }

    let mut ans = vec![0;M];
    let mut v = (0..N).collect_vec();
    let mut s = (0..N).collect_vec();
    for (i,&a) in A.iter().enumerate(){
        r[u[a+1]] = a;
        r[u[a]] = a + 1;
        let tmp = u[a];
        u[a] = u[a+1];
        u[a+1] = tmp;
        println!("s {:?}", s);
        println!("r {:?}", r);

        ans[i] = r[u[s[0]]];

        let tmp = v[a];
        v[a] = v[a+1];
        v[a+1] = tmp;
        s[v[a]] = a;
        s[v[a+1]] = a+1;
    }

    for i in 0..M{
        println!("{}", ans[i] + 1);
    }

    
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

