#![allow(unused_imports)]
#![allow(non_snake_case)]
use proconio::{input, fastout};
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use std::fmt::{Write, Display};
use std::collections::*;
use maplit::*;
use itertools::*;
use superslice::{Ext, Ext2};
use std::cmp::Ordering;

trait ToRadixVec{
    fn to_radix_vec(&self, d: i128)->Vec<char>;
}

impl ToRadixVec for i128{
    fn to_radix_vec(&self, d: i128)->Vec<char>{
        let mut v = vec![];
        let mut n = *self;
        while n > 0 {
            let (q, r) = num_integer::div_rem(n, d); 
            v.push(r);
            n = q;
        }
        let mut w = vec!['9';v.len()];
        let mut found = false;
        for (i, &vv) in v.iter().enumerate().rev(){
            if vv >= 9 {
                found = true;
            }
            if !found {
                w[i] = ('0' as u8 + vv as u8) as char;
            }
        }
        w
    }
}

trait CmpAsNum {
    fn cmp_as_num(&self, rhs: &Self)->Ordering;
}

impl CmpAsNum for Vec<char>{
    fn cmp_as_num(&self, rhs: &Self)->Ordering{
        match self.len().cmp(&rhs.len()) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => {
                self.iter().zip(rhs.iter())
                .rev()
                .find(|(a, b)|{a != b})
                .map_or(Ordering::Equal, |(a, b)|a.cmp(b))
            }
            Ordering::Greater => Ordering::Greater,
        }
    }
}

#[fastout]
fn main() {
    input!{
        X: Chars,
        M: i128,
    }

    if X.len() == 1 {
        let X = X.iter().collect::<String>().parse::<i128>().unwrap();        
        if X <= M {
            println!("{}", 1);
        } else {
            println!("{}", 0);
        }
        return;
    }

    let mut X = X;
    X.reverse();
    
    let d = X.iter().max().unwrap();
    let d = (*d as u8 - b'0') as i128;
    let n = d + 1;
    let min = M.to_radix_vec(n);
    if X.cmp_as_num(&min) == Ordering::Greater {
        println!("{}", 0);
        return;
    }
    
    let mut l = n;
    let mut r = 2*10i128.pow(18);
    while (l-r).abs() > 1 {
        let m = (l+r)/2;
        let M = M.to_radix_vec(m);
        //println!("{:?}", M);
        match  X.cmp_as_num(&M){
            std::cmp::Ordering::Less 
            | std::cmp::Ordering::Equal  => {
                l = m;
            }, 
            std::cmp::Ordering::Greater => {
                r = m;
            }
        };
    }

    println!("{}", r - n);
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

