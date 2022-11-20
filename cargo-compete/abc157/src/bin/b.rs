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
        A: [[i32;3];3],
        N: usize,
        b: [i32;N],
    }

    let b = b.into_iter().collect::<BTreeSet<_>>();
    let mut u = vec![vec![false;3];3];
    for i in 0..3{
        for j in 0..3{
            if b.contains(&A[i][j]){
                u[i][j] = true;
            }
        }
    }

    if [
        (u[0][0],u[0][1],u[0][2]),
        (u[1][0],u[1][1],u[1][2]),
        (u[2][0],u[2][1],u[2][2]),
        (u[0][0],u[1][0],u[2][0]),
        (u[0][1],u[1][1],u[2][1]),
        (u[0][2],u[1][2],u[2][2]),
        (u[0][0],u[1][1],u[2][2]),
        (u[0][2],u[1][1],u[2][0]),
    ].iter().find(|&&t|t==(true,true,true)).is_some(){
        println!("Yes");
    } else {
        println!("No");
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

