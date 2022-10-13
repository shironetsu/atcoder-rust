#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::*;
use maplit::*;
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
use std::fmt::{Display, Write};
use superslice::{Ext, Ext2};

fn f(a: i64, m: i64) -> i64 {
    (a * a).rem_euclid(m)
}

#[fastout]
fn main() {
    input! {
        N: i64,
        X: i64,
        M: i64,
    }

    let mut a = vec![X];
    let mut inv = vec![None::<usize>; M as usize];
    let mut b = vec![];
    let mut c = vec![];
    for i in 0.. {
        if let Some(j) = inv[a[i] as usize] {
            b = a[0..j].iter().copied().collect_vec();
            c = a[j..i].iter().copied().collect_vec();
            break;
        } else {
            inv[a[i] as usize] = Some(i);
        }
        a.push(f(a[i], M));
    }

    let mut cc = vec![0; c.len() + 1];
    for i in 0..c.len() {
        cc[i + 1] = cc[i] + c[i];
    }

    let ans = if N <= b.len() as i64 {
        b[0..N as usize].iter().sum::<i64>()
    } else {
        let n = N - b.len() as i64;
        let (d, r) = num_integer::div_rem(n, c.len() as i64);
        b.iter().sum::<i64>() + d * cc[c.len()] + cc[r as usize]
    };

    println!("{}", ans);
}
//______________________________________________________________________________
//
pub trait Answer {
    fn fmt(&self) -> String;
    fn fmtl(&self) -> String;
    fn ans(&self);
    fn ansl(&self);
}

impl<T: Display> Answer for Vec<T> {
    fn fmt(&self) -> String {
        self.iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn ans(&self) {
        println!("{}", self.fmt());
    }

    fn fmtl(&self) -> String {
        self.iter()
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
