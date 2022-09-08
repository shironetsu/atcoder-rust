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
        K: usize,
        A: [[i64;N];N],
    }

    let mut l = -1;
    let mut r = 10i64.pow(10);
    let h = (K*K)/2 + 1;
    while (l-r).abs() > 1 {
        //println!("{} {}", l, r);
        let m = (l+r)/2;
        let mut B = vec![vec![0;N];N];
        for i in 0..N{
            for j in 0..N{
                B[i][j] = if A[i][j] > m {
                    1
                } else {
                    0
                };
            }
        }

        let mut C = vec![vec![0;N+1];N+1];

        // for i in 0..N{
        //     for j in 0..N{
        //         C[i+1][j+1] = C[i+1][j] + B[i][j];
        //     }
        // }

        // for j in 0..N{
        //     for i in 0..N{
        //         C[i+1][j+1] += C[i][j+1];
        //     }
        // }

        for i in 0..N{
            for j in 0..N{
                C[i+1][j+1] = C[i][j+1] + C[i+1][j] - C[i][j] + B[i][j];
            }
        }

        let mut ok = false;

        for i in 0..=N-K{
            for j in 0..=N-K{
                let sum = C[i+K][j+K] + C[i][j] - C[i+K][j] - C[i][j+K];
                if sum < h {
                    ok = true;
                    break;
                }
            }
            if ok {
                break;
            }
        }

        if ok {
            r = m;
        } else {
            l = m;
        }
    }

    println!("{}", r);

    
}
//______________________________________________________________________________
//
pub trait Answer {
    fn ans(&self);
    fn ansl(&self);
}

impl<T: Display> Answer for Vec<T> {
    fn ans(&self) {
        let ans = self
            .iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<_>>()
            .join(" ");
        println!("{}", ans);
    }

    fn ansl(&self) {
        let ans = self
            .iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<_>>()
            .join("\n");
        println!("{}", ans);
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

