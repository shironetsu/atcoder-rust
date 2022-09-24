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
        X: f64,
        Y: f64,
        R: f64,
    }

    let m = 10i64.pow(4);
    let X = (X * m as f64 + 0.5).floor() as i64;
    let Y = (Y * m as f64 + 0.5).floor() as i64;
    let R = (R * m as f64 + 0.5).floor() as i64;
    let x_min = X - R;
    let x_max = X + R;
    let xx_min = num_integer::div_ceil(x_min, m);
    let xx_max = num_integer::div_floor(x_max, m);
    let mut ans = 0i64;
    for xx in xx_min..=xx_max {
        let x = xx * m;
        let d = num_integer::sqrt(R.pow(2)-(X-x).pow(2));
        let y_min = Y - d;
        let y_max = Y + d;
        let u = num_integer::div_ceil(y_min, m);
        let v = num_integer::div_floor(y_max, m);
        ans += v - u + 1;
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

