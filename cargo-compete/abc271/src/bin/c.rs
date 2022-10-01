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
        mut a: [i64;N],
    }

    if a == vec![1] {
        println!("{}", 1);
        return;
    }

    if a.len() == 1 {
        println!("{}", 0);
        return;
    }

    let mut s = BTreeSet::new();
    let mut aa = vec![];
    let mut inf = 10i64.pow(10);
    for &i in a.iter(){
        if s.insert(i) {
            aa.push(i);
        } else {
            aa.push(inf);
            inf += 1;
        }
    }
    a = aa;
    a.sort();
    let mut a = a.into_iter().collect::<VecDeque<_>>();
    let mut c = 1;

    loop {
        if let Some(&f) = a.front(){
            if f != c {
                if a.len() >= 2 {
                    a.pop_back().unwrap();
                    a.pop_back().unwrap();
                    a.push_front(c);
                } else {
                    println!("{}", c - 1);
                    return;
                }
            } else {
                a.pop_front().unwrap();
                c += 1;
            }
        } else {
            println!("{}", c - 1);
            return;
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

