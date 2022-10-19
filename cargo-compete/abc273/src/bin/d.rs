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
        H: isize,
        W: isize,
        rs: Isize1,
        cs: Isize1,
        N: usize,
        rc: [(Isize1,Isize1);N],
        Q: usize,
        dl: [(char, isize);Q],
    }

    let mut a = BTreeMap::new();
    let mut b = BTreeMap::new();

    for &(r, c) in rc.iter(){
        a.entry(r).or_insert(btreeset![-1,W]).insert(c);
        b.entry(c).or_insert(btreeset![-1,H]).insert(r);
    }

    let mut x = rs;
    let mut y = cs;
    for &(d, l) in dl.iter(){
        match d {
            'L' => {
                if let Some(s) = a.get_mut(&x) {
                    if let Some(&yy) = s.range(y-l..y).next_back() {
                        y = yy + 1;
                    } else {
                        y = y - l;
                    }
                } else {
                    y = (y-l).max(0);
                }
            },
            'R' => {
                if let Some(s) = a.get_mut(&x) {
                    if let Some(&yy) = s.range(y+1..=y+l).next() {
                        y = yy - 1;
                    } else {
                        y = y + l;
                    }
                } else {
                    y = (y+l).min(W-1);
                }
            },
            'U' => {
                if let Some(s) = b.get_mut(&y) {
                    if let Some(&xx) = s.range(x-l..x).next_back() {
                        x = xx + 1;
                    } else {
                        x = x - l;
                    }
                } else {
                    x = (x-l).max(0);
                }
            },
            'D' => {
                if let Some(s) = b.get_mut(&y) {
                    if let Some(&xx) = s.range(x+1..=x+l).next() {
                        x = xx - 1;
                    } else {
                        x = x + l;
                    }
                } else {
                    x = (x+l).min(H-1);
                }
            },
            _ => unreachable!(),
        }
        println!("{} {}", x+1, y+1);
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

