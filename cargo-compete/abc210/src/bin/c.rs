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
        K: usize,
        c: [i64;N],
    }

    let mut m = MultiSet::new();
    for i in 0..K{
        m.insert(c[i]);
    }

    let mut ans = m.len();
    for i in K..N{
        m.remove(c[i-K]);
        m.insert(c[i]);
        ans.chmax(m.len());
    }

    println!("{}", ans);



    
}
pub trait Change<T: PartialOrd> {
    fn chmin(&mut self, rhs: Self) -> bool;
    fn chmax(&mut self, rhs: Self) -> bool;
}
impl<T: PartialOrd> Change<T> for T {
    fn chmax(&mut self, rhs: Self) -> bool {
        if *self < rhs {
            *self = rhs;
            true
        } else {
            false
        }
    }
    fn chmin(&mut self, rhs: Self) -> bool {
        if *self > rhs {
            *self = rhs;
            true
        } else {
            false
        }
    }
}
//______________________________________________________________________________
//
pub struct MultiSet<T> {
    map: BTreeMap<T, usize>
}

impl<T: Ord> MultiSet<T>{
    pub fn new() -> MultiSet<T> {
        MultiSet { map: BTreeMap::new() }
    }

    pub fn insert(&mut self, x: T) {
       *self.map.entry(x).or_default() += 1;
    }

    pub fn remove(&mut self, x: T){
        let &prev = self.map.get(&x).unwrap();
        if prev == 1 {
            self.map.remove(&x);
        } else {
            *self.map.get_mut(&x).unwrap() -= 1;
        }
    }

    pub fn count(&self, x: &T) -> usize{
        *self.map.get(x).unwrap()
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }
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

