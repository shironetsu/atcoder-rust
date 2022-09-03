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
        A: [i64;N],
        UV: [(Usize1,Usize1);M],
    }

    if M == 0 {
        println!("{}", 0);
        return;
    }

    let mut ad = vec![vec![];N];
    for &(u, v) in UV.iter(){
        ad[u].push(v);
        ad[v].push(u);
    }

    let mut sum = vec![0;N];

    for i in 0..N{
        for &j in ad[i].iter(){
            sum[i] += A[j];
        }
    }

    let mut l = 0;
    let mut r = 10i64.pow(18);
    while (l-r).abs() > 1 {
        //println!("{} {}", l, r);
        let m = (l+r)/2;
        let mut sum = sum.clone();
        let mut ad = ad.clone();
        let mut pushed = vec![false;N];
        let mut ord = vec![];
        for i in 0..N{
            if sum[i] <= m {
                ord.push(i);
                pushed[i] = true;
            }
        }
        //println!("{:?}", ord);
        for i in 0..N{
            if i >= ord.len() {
                l = m;
                break;
            }
            let u = ord[i];
            for &v in ad[u].clone().iter(){
                ad[v].retain(|&w| w!=u);
                sum[v] -= A[u];
                if sum[v] <= m && !pushed[v] {
                    ord.push(v);
                    pushed[v] = true;
                }
            }
            if i == N - 1 {
                r = m;
            }
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

