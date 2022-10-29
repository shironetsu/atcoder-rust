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
        S: [Chars;9],
    }

    let mut ans = 0;
    for a in 0..9{
        for b in 0..9{
            for c in 0..9{
                for d in 0..9{
                    if (a,b) == (c,d) {
                        continue;
                    }
                    let s = a + b + c + d;
                    if s % 2 != 0 {
                        continue;
                    }

                    if a.max(b).max(c).max(d) > s/2 {
                        continue;
                    }

                    let (u, v) = (s/2-d, s/2-a);
                    let (p, q) = (s/2-b, s/2-c);

                    if u.max(v).max(p).max(q) >= 9 {
                        continue;
                    }

                    if (S[a][b],S[c][d],S[u][v],S[p][q]) == ('#','#','#','#') {
                        ans += 1;
                    }
                }
            }
        }
    }

    println!("{}", ans/4);

    
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

