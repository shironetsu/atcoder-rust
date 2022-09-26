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
        a: [usize;N],
    }

    let mut v = a.clone();
    let mut inv = v.merge_sort();
    for &aa in a.iter(){
        println!("{}", inv);
        inv += N as i64 - 1 - 2 * aa as i64;
    }
}

use std::cmp::Ord;

pub trait MergeSort {
    fn merge_sort(&mut self) -> i64;
}

impl<T: Ord + Clone> MergeSort for [T] {
    fn merge_sort(&mut self) -> i64 {
        let mut inv = 0;
        let n = self.len();
        if n > 1 {
            let m = n / 2;
            inv += self[0..m].merge_sort();
            inv += self[m..n].merge_sort();
            let mut w = Vec::with_capacity(n);
            let mut c = m;
            for i in 0..m {
                while c < n && self[c] < self[i] {
                    inv += (m - i) as i64;
                    w.push(self[c].clone());
                    c += 1;
                }
                w.push(self[i].clone());
            }
            for i in c..n {
                w.push(self[i].clone());
            }

            // https://stackoverflow.com/questions/66417950/how-to-assign-to-a-slice-range
            (*self).clone_from_slice(&w);
        }
        inv
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

