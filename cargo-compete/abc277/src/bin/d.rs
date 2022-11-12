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
        M: i64,
        A: [i64;N],
    }
    
    let st = A.iter().copied().collect::<BTreeSet<i64>>();
    let s = st.iter().copied().collect_vec();

    let rev = s.iter().enumerate().map(|(i, &x)|(x, i)).collect::<BTreeMap<i64, usize>>();
    let mut count = vec![0;s.len()];
    for &a in A.iter(){
        let &i = rev.get(&a).unwrap();
        count[i] += 1;
    }

    let mut uf = UnionFind::new(s.len());

    for &a in s.iter(){
        let b = (a+1).rem_euclid(M);
        if st.contains(&b) {
            let &i = rev.get(&a).unwrap();
            let &j = rev.get(&b).unwrap();
            uf.union(i, j);
        }
    }

    let l = uf.into_labeling();
    

    // let mut m = BTreeMap::<i64, i64>::new();
    // for &a in A.iter(){
    //     *m.entry(a).or_default() += 1;
    // }
    
    let mut scores = BTreeMap::new();
    for (i, &rep) in l.iter().enumerate(){
        let a = s[i];
        *scores.entry(rep).or_default() += count[i] * a;
    }
    let (_, max) = scores.iter().max_by_key(|&(_, score)|score).unwrap();

    let ans = A.iter().sum::<i64>() - max;
    println!("{}", ans);



    
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

