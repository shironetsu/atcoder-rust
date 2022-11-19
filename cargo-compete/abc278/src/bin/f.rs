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
        S: [Chars;N],
    }

    let mut ad = vec![vec![];N];

    for i in 0..N{
        for j in 0..N{
            if i == j {
                continue;
            }
            if S[i][S[i].len()-1] == S[j][0] {
                ad[i].push(j);
            }
        }
    }

    let mut valid = vec![vec![false;1<<N];N];
    //let mut st = vec![]; //可能な状態
    let mut tr = vec![vec![vec![];1<<N];N]; //遷移先
    for i in 0..N{
        valid[1<<i][i] = true;
    }
    for s in 0..1<<N{
        for i in 0..N{
            if !valid[s][i] {
                continue;
            }

            for &j in ad[i].iter(){
                if (s>>j) & 1 == 1{
                    continue;
                }
                let t = s ^ (1<<j);
                valid[t][j] = true;
                tr[s][i].push((t, j));
            }
        }
    }
    let mut g = vec![vec![0;1<<N];N];
    for s in (0..1<<N).rev(){
        for i in 0..N{
            if !valid[s][i] {
                continue;
            }
            if tr[s][i].len() == 0 {
                g[s][i] = 0;
            } else {
                let v = tr[s][i].iter().map(|&(t, j)|g[t][j]).collect::<BTreeSet<_>>();
                for n in 0..{
                    if !v.contains(&n){
                        g[s][i] = n;
                        break;
                    }
                }
            }
        }
    }

    let v = (0..N).map(|i|g[1<<i][i]).collect::<BTreeSet<_>>();
    if v.contains(&0) {
        println!("{}", "First");
    } else {
        println!("{}", "Second");
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

