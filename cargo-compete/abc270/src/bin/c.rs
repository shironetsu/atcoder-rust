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
        X: Usize1,
        Y: Usize1,
        UV: [(Usize1, Usize1);N-1],
    }

    let mut ad = vec![vec![];N];
    for &(u, v) in UV.iter(){
        ad[u].push(v);
        ad[v].push(u);
    }

    let mut ord = vec![0];
    let mut par = vec![None;N];
    for i in 0..N{
        let u = ord[i];
        for &v in ad[u].clone().iter(){
            ad[v].retain(|&x| x != u);
            ord.push(v);
            par[v] = Some(u);
        }
    }

    let mut p = vec![X];
    let mut q = vec![Y];
    for i in 0..{
        if let Some(u) = par[p[i]] {
            p.push(u);
        } else {
            break;
        }
    }
    for i in 0..{
        if let Some(u) = par[q[i]] {
            q.push(u);
        } else {
            break;
        }
    }

    p.reverse();
    q.reverse();
    let mut c = 0;
    loop{
        if c == p.len().min(q.len()) || p[c] != q[c] {
            break;
        }
        c += 1;
    }
    let mut path = vec![];
    for i in (c..p.len()).rev(){
        path.push(p[i]);
    }
    path.push(p[c-1]);
    for i in (c..q.len()) {
        path.push(q[i]);
    }

    //let mut path = vec![(&p[0..=i]), (&q[0..i]).clone()].concat();
    let ans = path.into_iter().map(|i|i+1).collect_vec();
    ans.ans();
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

