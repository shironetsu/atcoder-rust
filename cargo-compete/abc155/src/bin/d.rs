#![allow(unused_imports)]
#![allow(non_snake_case)]
use proconio::{input, fastout};
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use std::fmt::{Write, Display};
use std::collections::*;
use maplit::*;
use itertools::*;
use superslice::{Ext, Ext2};

fn less(A: &Vec<i64>, m: i64)->i64{
    let n = A.len();
    let mut s = 0i64;
    for (i, &a) in A.iter().enumerate(){
        let ds = match a.cmp(&0) {
            std::cmp::Ordering::Greater => {
                let u = num_integer::div_ceil(m, a);
                (A.upper_bound(&u) as i64 - i as i64 - 1).max(0)
            },
            std::cmp::Ordering::Equal => {
                if m > 0 { (n-i-1) as i64 } else { 0 }
            },
            std::cmp::Ordering::Less => {
                let u = num_integer::div_floor(m, a);
                (A.len() - A.upper_bound(&u) - 1).min(n-i-1) as i64
            },
        };
        s += ds;
    }
    s
}

#[fastout]
fn main() {
    input!{
        N: usize,
        K: i64,
        mut A: [i64;N],
    }

    //K += 2;
    A.sort();

    // let mut s = vec![];
    // for i in 0..N{
    //     for j in 0..N{
    //         s.push(A[i]*A[j]);
    //     }
    // }
    // s.sort();
    //println!("{:?}", s);

    let mut l = -2*10i64.pow(18); //s=0
    let mut r = 2*10i64.pow(18);  //s=n*n
    while (l-r).abs() > 1 {
        let m = (l+r)/2;
        let s = less(&A, m);
        println!("{} {}", m, s);
        if s <= K {
            l = m;
        } else {
            r = m;
        }
    }

    println!("{}", l);
    
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

