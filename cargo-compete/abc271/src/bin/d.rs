#![allow(unused_imports)]
#![allow(non_snake_case)]
use proconio::{input, fastout};
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use std::fmt::{Write, Display};
use std::collections::*;
use maplit::*;
use itertools::*;
use superslice::{Ext, Ext2};
use bitset_fixed::BitSet;

#[fastout]
fn main() {
    input!{
        N: usize,
        S: usize,
        ab: [(usize, usize);N],
    }

    let mut dp = vec![vec![false;S+1];N+1];
    dp[0][0] = true;
    for i in 0..N{
        let (a, b) = ab[i];
        for j in 0..S+1{
            if j + a <= S {
                dp[i+1][j+a] |= dp[i][j];
            }
            if j + b <= S{
                dp[i+1][j+b] |= dp[i][j];
            }
        }
    }
    
    if dp[N][S] {
        println!("Yes");
        let mut ans = vec![];
        let mut j = S;
        for i in (0..N).rev(){
            let (a, b) = ab[i];
            if j >= a && dp[i][j-a] {
                ans.push('H');
                j -= a;
            } else {
                ans.push('T');
                j -= b;
            }
        }
        ans.reverse();
        let ans = ans.iter().collect::<String>();
        println!("{}", ans);
    } else {
        println!("No");
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

