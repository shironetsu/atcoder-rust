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
        N: usize,
        h: usize,
        w: usize,
        A: [[Usize1;W];H],
    }

    let mut u = vec![btreeset![];N]; // [0,H)
    let mut v = vec![btreeset![];N]; // [0,W)
    let mut num = btreeset![];
    for i in 0..H{
        for j in 0..W{
            u[A[i][j]].insert(i);
            v[A[i][j]].insert(j);
            num.insert(A[i][j]);
        }
    }

    let mut umin = vec![None;N];
    let mut umax = vec![None;N];
    for &n in num.iter(){
        umin[n] = Some(*u[n].iter().min().unwrap());
        umax[n] = Some(*u[n].iter().max().unwrap());
    }


    let mut vmin = vec![None;N];
    let mut vmax = vec![None;N];
    for &n in num.iter(){
        vmin[n] = Some(*v[n].iter().min().unwrap());
        vmax[n] = Some(*v[n].iter().max().unwrap());
    }

    let mut ans = vec![vec![0;W-w+1];H-h+1];
    for k in 0..=H-h{
        for l in 0..=W-w{
            let hmin = k;
            let hmax = k+h-1;
            let wmin = l;
            let wmax = l+w-1;
            let mut x = num.len();
            for &n in num.iter(){
                let a = umin[n].unwrap();
                let b = umax[n].unwrap();
                let c = vmin[n].unwrap();
                let d = vmax[n].unwrap();
                if k <= a && b <= hmax && wmin <= c && d <= wmax {
                    x -= 1;
                }
            }
            ans[k][l] = x;
        }
    }

    for k in 0..=H-h{
        let row = ans[k].iter().map(|x|x.to_string()).join(" ");
        println!("{}", row);
    }



    
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

