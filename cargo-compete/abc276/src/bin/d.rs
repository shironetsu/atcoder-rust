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
        a: [i64;N],
    }

    let mut b = vec![0;N];
    let mut c = vec![0;N];
    let mut r = vec![0;N];
    for i in 0..N{
        let mut m = a[i];
        while m % 2 == 0 {
            b[i] += 1;
            m /= 2;
        }
        while m % 3 == 0 {
            c[i] += 1;
            m /= 3;
        }
        r[i] = m;
    }

    for i in 1..N{
        if r[0] != r[i]{
            println!("{}", -1);
            return;
        }
    }

    let b_min = b.iter().min().unwrap();
    let c_min = c.iter().min().unwrap();
    
    let ans = b.iter().map(|&k|k-b_min).sum::<i32>()
                + c.iter().map(|&k|k-c_min).sum::<i32>();
    println!("{}", ans);
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

