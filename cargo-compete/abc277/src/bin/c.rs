#![allow(unused_imports)]
#![allow(non_snake_case)]
use proconio::{input, fastout};
use proconio::marker::{Bytes, Chars, Isize1, Usize1};
use std::fmt::{Write, Display};
use std::collections::*;
use maplit::*;
use itertools::*;
use superslice::{Ext, Ext2};
use petgraph::unionfind::UnionFind;

#[fastout]
fn main() {
    input!{
        N: usize,
        AB: [(Usize1, Usize1);N],
    }

    let mut s = BTreeSet::new();
    for &(a, b) in AB.iter(){
        s.insert(a);
        s.insert(b);
    }
    let mut v = s.into_iter().collect_vec();
    let mut rev = v.iter().enumerate().map(|(i, &x)|(x, i)).collect::<BTreeMap<_,_>>();

    let mut uf = UnionFind::new(v.len());
    for &(a ,b) in AB.iter(){
        uf.union(*rev.get(&a).unwrap(), *rev.get(&b).unwrap());
    }
    let l = uf.into_labeling();
    if let Some(&k) = rev.get(&0) {
        let r = l[k];
        for i in (0..v.len()).rev(){
            if l[i] == r {
                println!("{}", v[i]+1);
                return;
            }
        }
    } else {
        println!("{}", 1);
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

