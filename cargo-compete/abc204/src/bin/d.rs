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
        T: [i64;N],
    }

    let n = 100_000i64;
    let mut b = BitSet::new(n as usize + 1);
    b.set(0, true);
    for &t in T.iter(){
        b.shl_or(t as usize);
    }

    let mut sum = T.iter().sum::<i64>();
    let mut a = (0..=n).filter(|n|b[*n as usize]).collect_vec();
    let i = a.lower_bound(&(sum/2));
    let ans = a[i].max(sum-a[i]);
    println!("{}", ans);



    
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

