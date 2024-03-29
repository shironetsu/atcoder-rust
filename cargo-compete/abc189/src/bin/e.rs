#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::*;
use maplit::*;
use nalgebra::{Matrix3, Vector3};
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use proconio::{fastout, input};
use std::collections::*;
use std::fmt::{Display, Write};
use superslice::{Ext, Ext2};

#[fastout]
fn main() {
    input! {
        N: usize,
        XY: [(i64, i64);N],
        M: usize,
    }
    let mut m = Matrix3::identity();
    let mut mm = Vec::with_capacity(M + 1);
    mm.push(m);
    let r = Matrix3::new(
        0, 1, 0, 
        -1, 0, 0, 
        0, 0, 1
    );
    let s = Matrix3::new(
        0, -1, 0, 
        1, 0, 0, 
        0, 0, 1
    );

    for _ in 0..M {
        input! {
            t: i32,
        }
        match t {
            1 => {
                m = r * m;
                mm.push(m);
            }
            2 => {
                m = s * m;
                mm.push(m);
            }
            3 => {
                input! {
                    p: i64,
                }
                let a = Matrix3::new(
                    -1, 0, 2 * p, 
                    0, 1, 0, 
                    0, 0, 1
                );
                m = a * m;
                mm.push(m);
            }
            4 => {
                input! {
                    p: i64,
                }
                let a = Matrix3::new(
                    1, 0, 0, 
                    0, -1, 2 * p, 
                    0, 0, 1
                );
                m = a * m;
                mm.push(m);
            }
            _ => unreachable!(),
        }
    }

    input! {
        Q: usize,
        AB: [(usize, Usize1);Q],
    }

    for &(a, b) in AB.iter() {
        let (x, y) = XY[b];
        let v = Vector3::new(x, y, 1);
        let w = mm[a] * v;
        println!("{} {}", w[0], w[1]);
    }
}
//______________________________________________________________________________
//
pub trait Answer {
    fn fmt(&self) -> String;
    fn fmtl(&self) -> String;
    fn ans(&self);
    fn ansl(&self);
}

impl<T: Display> Answer for Vec<T> {
    fn fmt(&self) -> String {
        self.iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn ans(&self) {
        println!("{}", self.fmt());
    }

    fn fmtl(&self) -> String {
        self.iter()
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
