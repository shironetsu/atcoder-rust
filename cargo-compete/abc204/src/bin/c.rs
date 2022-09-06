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
        M: usize,
        AB: [(Usize1, Usize1);M],
    }

    let mut ad = vec![vec![];N];
    for &(a, b) in AB.iter(){
        ad[a].push(b);
    }

    let mut ans = 0;
    for u in 0..N{
        let mut seen = vec![false;N];
        let mut todo = VecDeque::new();
        todo.push_back(u);
        seen[u] = true;
        loop {
            if let Some(v) = todo.pop_front(){
                for &w in ad[v].iter(){
                    if !seen[w] {
                        todo.push_back(w);
                        seen[w] = true;
                    }
                }
            } else {
                break;
            }
        }
        ans += seen.iter().filter(|x|**x).count();
    }
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

