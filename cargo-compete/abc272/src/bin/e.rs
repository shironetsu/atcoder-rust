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
    //solve1();
    solve2();
}

fn solve2() {
    input! {
        N: usize,
        M: usize,
        A: [i32;N],
    }

    let mut c = vec![btreeset![];M];
    for i in 0..N {
        let a = A[i];
        let mut l = num_integer::div_ceil(-a, i as i32 + 1) - 1;
        l = l.max(0);
        let mut r = num_integer::div_floor(N as i32-1-a, i as i32 + 1) - 1;
        r = r.min(M as i32-1);
        for k in l..=r {
            let b = a + (i+1) as i32*(k+1);
            c[k as usize].insert(b);
        }
    }

    let mut ans = vec![0;M];
    for k in 0..M{
        for m in 0..{
            if !c[k].contains(&m) {
                ans[k] = m;
                break;
            }
        }
    }

    ans.ansl();
}

fn solve1() {
    input!{
        N: usize,
        M: usize,
        A: [i32;N],
    }

    let mut idx = vec![NekoSet::new();M+1];
    for (i, &a) in A.iter().enumerate() {
        let l = num_integer::div_ceil(-a, i as i32 + 1);
        let r = num_integer::div_floor(N as i32 - 1 - a, i as i32 + 1);
        for k in l.max(0) as usize..=r.min(M as i32) as usize {
            idx[k].insert(a + (i as i32+1)*k as i32);
        }
    }

    for k in 1..=M{
        println!("{}", idx[k].mex(0));
    }
}
//______________________________________________________________________________
//
//Ref: https://rsk0315.hatenablog.com/entry/2020/10/11/125049
#[derive(Clone)]
struct NekoSet {
    s: BTreeSet<(i32, i32)>,
}

impl NekoSet {
    fn new() -> Self {
        Self {
            s: vec![
                (std::i32::MIN, std::i32::MIN),
                (std::i32::MAX, std::i32::MAX),
            ]
            .into_iter()
            .collect(),
        }
    }
    fn insert(&mut self, x: i32) -> bool {
        let &(nl, nu) = self.s.range((x + 1, x + 1)..).next().unwrap();
        let &(l, u) = self.s.range(..(x + 1, x + 1)).next_back().unwrap();
        if l <= x && x <= u {
            return false;
        }
        if u == x - 1 {
            if nl == x + 1 {
                self.s.remove(&(nl, nu));
                self.s.remove(&(l, u));
                self.s.insert((l, nu));
            } else {
                self.s.remove(&(l, u));
                self.s.insert((l, x));
            }
        } else {
            if nl == x + 1 {
                self.s.remove(&(nl, nu));
                self.s.insert((x, nu));
            } else {
                self.s.insert((x, x));
            }
        }
        true
    }
    fn mex(&self, x: i32) -> i32 {
        let &(l, u) = self.s.range(..(x + 1, x + 1)).next_back().unwrap();
        if l <= x && x <= u {
            u + 1
        } else {
            x
        }
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

