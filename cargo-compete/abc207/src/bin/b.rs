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
        a: i64,
        b: i64,
        c: i64,
        d: i64,
    }

    let r = c * d - b;
    let ans = if r > 0 {
        num_integer::div_ceil(a, r)
    } else {
        -1
    };

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

