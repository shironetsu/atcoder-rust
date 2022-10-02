#![allow(unused_imports)]
#![allow(non_snake_case)]
use proconio::{input, fastout};
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use std::fmt::{Write, Display};
use std::collections::*;
use maplit::*;
use itertools::*;
use superslice::{Ext, Ext2};

pub trait Between {
    fn between(&self, x: i64, y: i64) -> bool;
}


impl Between for i64 {
    fn between(&self, x: i64, y: i64) -> bool {
        x <= *self && *self <= y || y <= *self && *self <= x
    }
}


#[fastout]
fn main() {
    input!{
        X: i64,
        Y: i64,
        Z: i64,
    }

    if !Y.between(0, X) {
        println!("{}", X.abs());
    } else {
        if !Y.between(0, Z) {
            let ans = Z.abs() + (Y-Z).abs() + (X-Y).abs();
            println!("{}", ans);
        } else {
            println!("{}", -1);
        }
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

