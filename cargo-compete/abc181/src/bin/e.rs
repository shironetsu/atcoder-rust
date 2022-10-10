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
        M: usize,
        mut H: [i64;N],
        W: [i64;M],
    }

    H.sort();
    let mut l = vec![0;N/2];
    let mut r = vec![0;N/2];
    for i in 0..N/2{
        l[i] = (H[2*i]-H[2*i+1]).abs();
        r[i] = (H[2*i+1]-H[2*i+2]).abs();
    }

    let mut ll = vec![0;N/2+1];
    let mut rr = vec![0;N/2+1];
    for i in 0..N/2{
        ll[i+1] = ll[i] + l[i];
        rr[i+1] = rr[i] + r[i];
    }

    let inf = 10i64.pow(18);
    let mut ans = inf;

    for &w in W.iter(){
        let  i = H.upper_bound(&w);
        let sum = 
        if i & 1 == 0 {
            //&l[0..i/2].iter().sum::<i64>() + (w-H[i]).abs() + &r[i/2..].iter().sum::<i64>()
            ll[i/2] + (w-H[i]).abs() + rr[N/2]-rr[i/2]
        } else {
            //&l[0..i/2].iter().sum::<i64>() + (w-H[i-1]).abs() + &r[i/2..].iter().sum::<i64>()
            ll[i/2] + (H[i-1]-w).abs()  + rr[N/2]-rr[i/2]
        };
        //println!("{} {} {}", w, i, sum);
        ans.chmin(sum);
    }

    println!("{}", ans);

    
}
pub trait Change<T: PartialOrd> {
    fn chmin(&mut self, rhs: Self) -> bool;
    fn chmax(&mut self, rhs: Self) -> bool;
}
impl<T: PartialOrd> Change<T> for T {
    fn chmax(&mut self, rhs: Self) -> bool {
        if *self < rhs {
            *self = rhs;
            true
        } else {
            false
        }
    }
    fn chmin(&mut self, rhs: Self) -> bool {
        if *self > rhs {
            *self = rhs;
            true
        } else {
            false
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