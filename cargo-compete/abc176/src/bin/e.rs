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
        M: usize,
        hw: [(Usize1, Usize1);M],
    }

    let mut row = vec![0;H];
    let mut col = vec![0;W];
    for &(h, w) in hw.iter(){
        row[h] += 1;
        col[w] += 1;
    }
    let &rmax = row.iter().max().unwrap();
    let &cmax = col.iter().max().unwrap();
    let hh = row.iter().copied().enumerate().filter(|&(i, r)|r==rmax).collect_vec();
    let ww = col.iter().copied().enumerate().filter(|&(j, c)|c==cmax).collect_vec();
    
    let cross = hw.iter().filter(|&&(h, w)|(row[h],col[w])==(rmax, cmax)).count();
    let ans = if cross == hh.len() * ww.len() {
        rmax + cmax - 1
    } else {
        rmax + cmax
    };

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

