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
        mut tlr: [(i32, f64,f64);N],
    }

    let e = 0.1f64;

    let mut lr = vec![(0.0,0.0);N];
    for i in 0..N{
        let (t, mut l, mut r) = tlr[i];
        match t {
            1 => (),
            2 => {r-=e;},
            3 => {l+=e;},
            4 => {r-=e;l+=e;},
            _ => unreachable!(),
        }
        lr[i] = (l, r);
    }

    let mut ans = 0;
    for i in 0..N{
        for j in i+1..N{
            let (l0, r0) = lr[i];
            let (l1, r1) = lr[j];
            if l0 > r0 || l1 > r1 {
                continue;
            }
            if !(r0 < l1 || r1 < l0) {
                ans += 1;
            }
        }
    }

    println!("{}", ans);

    
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

