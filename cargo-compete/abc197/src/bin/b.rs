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
        H: usize,
        W: usize,
        X: Usize1,
        Y: Usize1,
        S: [Chars;H],
    }

    let mut ans = 1;
    let mut a = X - 1;
    while 0 <= a && a < H && S[a][Y] == '.' {
        ans += 1;
        a -= 1;
    }
    let mut b = X + 1;
    while 0 <= b && b < H && S[b][Y] == '.' {
        ans += 1;
        b += 1;
    }
    let mut c = Y - 1;
    while 0 <= c && c < W && S[X][c] == '.' {
        ans += 1;
        c -= 1;
    }
    let mut d = Y + 1;
    while 0 <= d && d < W && S[X][d] == '.' {
        ans += 1;
        d += 1;
    }
    println!("{}", ans);
    
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

