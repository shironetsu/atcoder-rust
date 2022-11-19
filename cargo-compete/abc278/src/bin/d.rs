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
        A: [i64;N],
        Q: isize,
    }

    let mut ans = vec![];
    let mut d = vec![vec![];N];
    for i in 0..N{
        d[i].push((-1, A[i]));
    }
    let mut z = vec![];
    let mut ins = vec![];
    for i in 0..Q{
        input!{
            t: i32,
        }
        //println!("{} {:?}", i, d);
        match t {
            1 => {
                input!{
                    x: i64,
                }
                z.push(x);
                ins.push(i);
            },
            2 => {
                input!{
                    j: Usize1,
                    x: i64,
                }
                let (_, a) = d[j][d[j].len()-1]; //末尾
                d[j].push((i, a+x));
            },
            3 => {
                input!{
                    j: Usize1,
                }
                let a =
                if z.len() == 0 {
                    let (_, a) = d[j][d[j].len()-1];
                    a
                } else {
                    let a0 = z[z.len()-1];
                    let l = ins[ins.len()-1];
                    let k = d[j].lower_bound(&(l, 0));
                    let (_, b) = d[j][k-1];
                    let (_, c) = d[j][d[j].len()-1];
                    a0 + c - b
                };
                ans.push(a)
            },
            _ => unreachable!(),
        }
    }

    for &a in ans.iter(){
        println!("{}", a);
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

