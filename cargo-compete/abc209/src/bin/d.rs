#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::*;
use maplit::*;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
use std::fmt::{Display, Write};
use superslice::{Ext, Ext2};

#[fastout]
fn main() {
    input! {
        N: usize,
        Q: usize,
    }

    input_edges!(N, N - 1, edges, ad);

    input! {
        q: [(Usize1, Usize1);Q],
    }

    let mut dist = vec![0; N];
    let start = 0;
    let mut ord = vec![start];
    let mut ad = ad;
    for i in 0..N {
        let u = ord[i];
        for &v in ad[u].clone().iter() {
            ord.push(v);
            dist[v] = dist[u] + 1;
            ad[v].retain(|&x| x != u);
        }
    }

    for &(c, d) in q.iter() {
        if (dist[c] + dist[d]) % 2 == 0 {
            println!("Town");
        } else {
            println!("Road");
        }
    }
}
//______________________________________________________________________________
//
pub trait Answer {
    fn ans(&self);
    fn ansl(&self);
}

impl<T: Display> Answer for Vec<T> {
    fn ans(&self) {
        let ans = self
            .iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<_>>()
            .join(" ");
        println!("{}", ans);
    }

    fn ansl(&self) {
        let ans = self
            .iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<_>>()
            .join("\n");
        println!("{}", ans);
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
