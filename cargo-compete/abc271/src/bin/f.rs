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
        a: [[i64;N];N],
    }

    let mut u = vec![vec![vec![];N];N];
    u[N-1][N-1] = vec![a[N-1][N-1]];
    for k in (0..2*N-2).rev(){
        for i in 0..=k{
            if N <= i {
                continue;
            }
            let j = k - i;
            if N <= j {
                continue;
            }
            if i + 1 < N {
                let v = u[i+1][j].iter().map(|x|x^a[i][j]).collect_vec();
                u[i][j].extend(v);
            }
            if j + 1 < N {
                let v = u[i][j+1].iter().map(|x|x^a[i][j]).collect_vec();
                u[i][j].extend(v);
            }
        }
    }

    let mut ans = 0;
    for &x in u[0][0].iter(){
        if x == 0 {
            ans += 1;
        }
    }

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

