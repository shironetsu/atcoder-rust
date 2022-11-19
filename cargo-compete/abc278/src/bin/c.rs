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
        N: i32,
        Q: usize,
        TAB: [(i32,i32,i32);Q],
    }

    let mut u = BTreeSet::new();
    for &(_, a, b) in TAB.iter(){
        u.insert(a);
        u.insert(b);
    }
    let u = u.into_iter().collect_vec();
    let rev = u.iter().enumerate().map(|(i, &x)|(x, i)).collect::<BTreeMap::<_, _>>();

    let mut f = vec![btreeset![];u.len()];
    let mut ans = vec![];
    for &(t, a, b) in TAB.iter(){
        let &i = rev.get(&a).unwrap();
        let &j = rev.get(&b).unwrap();
        match t {
            1 => {
                f[i].insert(j);
            }, 
            2 => {
                f[i].remove(&j);
            },
            3 => {
                ans.push(f[i].contains(&j) && f[j].contains(&i));
            },
            _ => unreachable!(),
        }
    }

    for &a in ans.iter(){
        if a {
            println!("Yes");
        } else {
            println!("No");
        }
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

