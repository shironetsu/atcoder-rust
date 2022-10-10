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
        S: Chars,
    }

    if S.len() == 1 {
        let S = (S[0] as u8 - b'0') as i32;
        if S == 8 {
            println!("Yes");
        } else {
            println!("No");
        }
        return;
    }

    if S.len() == 2 {
        let a = (S[0] as u8 - b'0') as i32;
        let b = (S[1] as u8 - b'0') as i32;
        if (a*10+b) % 8 == 0 || (b*10+a) % 8 == 0 {
            println!("Yes");
        } else {
            println!("No");
        }
        return;
    }

    let mut counts = BTreeMap::<char, i32>::new();
    for &c in S.iter(){
        *counts.entry(c).or_default() += 1;
    }

    let mut v = vec![];
    for n in 0..1000{
        if n % 8 == 0 {
            let n = format!("{:03}", n);
            if n.chars().find(|&c|c=='0').is_none(){
                v.push(n);
            }
        }
    }

    for n in v.iter(){
        let mut counts = counts.clone();
        let mut ok = true;
        for x in n.chars() {
            if let Some(c) = counts.get_mut(&x) {
                if *c > 0 {
                    *c -= 1;
                } else {
                    ok = false;
                }
            } else {
                ok = false
            }
        }
        if ok {
            println!("Yes");
            return;
        }
    }

    println!("No");


    
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

