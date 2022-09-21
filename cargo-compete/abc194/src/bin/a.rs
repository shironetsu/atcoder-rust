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
        A: i32, //無脂乳固形分
        B: i32, //乳脂肪分
    }

    //スヌケアイスがアイスクリームに当てはまる場合 : 1
    //スヌケアイスがアイスミルクに当てはまる場合 : 2
    //スヌケアイスがラクトアイスに当てはまる場合 : 3
    //スヌケアイスが氷菓に当てはまる場合 : 4

    //乳固形分は無脂乳固形分と乳脂肪分の 2 つから成ります。

    let C = A + B; //乳固形分
    let ans = if C>=15 && B>=8 {
        1
    } else if C>=10 && B >= 3 {
        2
    } else if C >= 3 {
        3
    } else {
        4
    };
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

