#![allow(unused_imports)]
#![allow(non_snake_case)]
use proconio::{input, fastout};
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use std::fmt::{Write, Display};
use std::collections::*;
use maplit::*;
use itertools::*;
use superslice::{Ext, Ext2};

fn f(A: f64, B: f64, t: f64)->f64{
    let g = 1.0;
    t + A * (g + t / B).powf(-0.5)
}

#[fastout]
fn main() {
    input!{
        A: f64,
        B: f64,
    }


    let a = f(A,B,0.0);
    let b = f(A,B,1.0);
    if a < b {
        println!("{}", a);
        return;
    }

    let mut l = 0.0;
    let mut r = 3.0*10f64.powi(18);
    while (l-r).abs() > 0.9 {
        let c1 = (2.0*l+r)/3.0;
        let c2 = (l+2.0*r)/3.0;

        if f(A,B,c1) > f(A,B,c2) {
            l = c1;
        } else {
            r = c2;
        }
    }

    let ans =  (-10..10).map(|h|{
                let t = (l.round() + h as f64).max(0.0);
                f(A,B,t)
            }).min_by(|x,y|x.partial_cmp(y).unwrap()).unwrap();
    println!("{}", ans);

    // let g = 1.0;
    // //let t = B * ((0.5 * A / B).powf(2.0/3.0) - g);
    // let t = 0.5f64.powf(2.0/3.0) * B.powf(1.0/3.0) * A.powf(2.0/3.0) - B * g;
    // let ans = if t < 0.0 {
    //     A / g.powf(0.5)
    // } else {
    //     // let ul = (t.floor() - 1.0).max(0.0);
    //     // let tl = t.floor();
    //     // let tr = t.ceil();
    //     // let ur = t.ceil()+1.0;
    //     // f(A,B,tl).min(f(A,B,tr)).min(f(A,B,ul)).min(f(A,B,ur))

    //     (-10..10).map(|h|{
    //         let t = (t.round() + h as f64).max(0.0);
    //         f(A,B,t)
    //     }).min_by(|x,y|x.partial_cmp(y).unwrap()).unwrap()
    // };

    // println!("{}", ans);

    
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

