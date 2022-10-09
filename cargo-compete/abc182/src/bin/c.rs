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
        N: Chars,
    }

    let N = N.iter().map(|&c|(c as u8 - b'0') as i64).collect_vec();
    let sum = N.iter().sum::<i64>();
    let one = N.iter().filter(|&&x|x%3==1).count();
    let two = N.iter().filter(|&&x|x%3==2).count();
    if sum % 3 == 0 {
        println!("{}", 0);
        return;
    } else if sum % 3 == 1 {
        if one >= 1 && N.len() > 1  {
            println!("{}", 1);
        } else if two >= 2  && N.len() > 2 {
            println!("{}", 2);
        } else {
            println!("{}", -1);
        }
    } else {
        if two >= 1 && N.len() > 1 {
            println!("{}", 1);
        } else if one >= 2 && N.len() > 2 {
            println!("{}", 2);
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

