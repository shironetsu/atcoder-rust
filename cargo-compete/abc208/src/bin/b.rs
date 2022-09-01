#![allow(unused_imports)]
#![allow(non_snake_case)]
use proconio::{input, fastout};
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use std::fmt::{Write, Display};
use std::collections::*;
use maplit::*;
use itertools::*;
use superslice::{Ext, Ext2};

fn fac(n: i64)->i64{
    if n <= 1 {
        1
    } else {
        n * fac(n-1)
    }
}

#[fastout]
fn main() {
    input!{
        mut P: i64,
    }

    let mut ans = 0;

    for n in (1..=10).rev(){
        let f = fac(n);
        let q = P / f;
        ans += q;
        P -= q.min(100) * f;
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

