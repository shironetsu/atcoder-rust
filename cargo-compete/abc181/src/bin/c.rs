#![allow(unused_imports)]
#![allow(non_snake_case)]
use proconio::{input, fastout};
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use std::fmt::{Write, Display};
use std::collections::*;
use maplit::*;
use itertools::*;
use superslice::{Ext, Ext2};
use nalgebra::{Matrix3};


#[fastout]
fn main() {
    input!{
        N: usize,
        xy: [(f64,f64);N],
    }

    for i in 0..N{
        for j in i+1..N{
            for k in j+1..N{
                let (x0, y0) = xy[i];
                let (x1, y1) = xy[j];
                let (x2, y2) = xy[k];
                let m = Matrix3::new(
                    x0, y0, 1.0,
                    x1, y1, 1.0,
                    x2, y2, 1.0
                );
                if m.determinant().abs() < 10.0f64.powi(-5) {
                    println!("Yes");
                    return;
                }
            }
        }
    }

    println!("No");


    
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

