#![allow(unused_imports)]
#![allow(non_snake_case)]
use proconio::{input, fastout};
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use std::fmt::{Write, Display};
use std::collections::*;
use maplit::*;
use itertools::*;
use superslice::{Ext, Ext2};

fn name(N: i64)->String{
    let a = (0..26).map(|i|(b'a' + i as u8) as char).collect_vec();
    let mut v = vec![];
    for k in 0.. {
        let d = 26i64.pow(k);
        if N <= d {
            break;
        }
        //let e = if k == 0 { 0 } else { d };
        if k == 0 {
            let r = (N-1).rem_euclid(26);
            v.push(a[r as usize]);
        } else {
            let r = ((N - d).rem_euclid(d*26)/d).rem_euclid(26);
            v.push(a[r as usize]);
        }
    }
    v.reverse();
    v.to_string()
}

#[fastout]
fn main() {
    input!{
        N: i64,
    }

    let ans = name(N);
    println!("{}", ans);

    // for n in 1..100{
    //     println!("{}", name(n));
    // }

    
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

