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
        S: Chars,
        Q: usize,
        mut TAB: [(i32, usize, usize);Q],
    }

    for i in 0..Q{
        if TAB[i].0 == 1 {
            TAB[i].1 -= 1;
            TAB[i].2 -= 2;
        }
    }

    let mut p = (0..2*N).collect_vec();
    let mut f = false;
    for &(t, a, b) in TAB.iter(){
        match t {
            1 => {
                let (a, b) = if f {
                    ((a+N) % (2*N), (b+N) % (2*N))
                } else {
                    (a, b)
                };
                let tmp = p[a];
                p[a] = p[b];
                p[b] = tmp;
            }, 
            2 => {
                f = !f;
            },
            _ => unreachable!(),
        }
    }

    let mut ans = vec!['0';2*N];
    for i in 0..2*N{
        let u = if f {
            p[i]
        } else {
            p[i]
        };
        ans[u] = S[i];
    }

    let ans = ans.iter().collect::<String>();
    println!("{}", ans);
    
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

