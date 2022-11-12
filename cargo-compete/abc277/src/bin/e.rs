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
        K: usize,
        uva: [(Usize1, Usize1,usize);M],
        s: [Usize1;K],
    }

    let mut ad = vec![vec![vec![];N];2];

    for &(u, v, a) in uva.iter(){
            ad[a][u].push(v);
            ad[a][v].push(u);
    }

    //println!("{:?}", ad);

    let mut t = vec![false;N];
    for &i in s.iter(){
        t[i] = true;
    }

    let mut todo = VecDeque::new();
    let mut d = vec![vec![None;N];2];
    let start = 0;
    todo.push_back((start, 1));
    d[1][start] = Some(0);

    loop {
        if let Some((i, a)) = todo.pop_front() {
            for &j in ad[a][i].iter(){
                if d[a][j].is_none(){
                    d[a][j] = Some(d[a][i].unwrap()+1);
                    todo.push_back((j, a));
                }
            }
            if t[i] {
                let b = a ^ 1;
                for &j in ad[b][i].iter(){
                    if d[b][j].is_none(){
                        d[b][j] = Some(d[a][i].unwrap()+1);
                        todo.push_back((j, b));
                    }
                }
            }
        } else {
            break;
        }
    }

    let mut u = vec![];
    for a in 0..=1{
        if let Some(k) = d[a][N-1] {
            u.push(k);
        }
    }
    
    if let Some(k) = u.iter().min() {
        println!("{}", k);
    } else {
        println!("{}", -1);
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

