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
        H: usize,
        W: usize,
        S: [Chars;H],
    }

    let mut ans = 0;
    for s in 0..H*W{
        let (a, b) = num_integer::div_rem(s, W);
        if S[a][b] == '#' {
            continue;
        }
        for g in 0..H*W{
            let (c, d) = num_integer::div_rem(g, W);
            if S[c][d] == '#' {
                continue;
            }
            let mut todo = VecDeque::new();
            todo.push_back((a, b));
            let mut dd = vec![vec![None;W];H];
            dd[a][b] = Some(0);
            loop {
                if let Some((x, y)) = todo.pop_front(){
                    for &(dx, dy) in [(1, 0), (!0, 0), (0, 1), (0, !0)].iter(){
                        let x1 = x + dx;
                        let y1 = y + dy;
                        if !(x1 < H && y1 < W){
                            continue;
                        }
                        if dd[x1][y1].is_some(){
                            continue;
                        }
                        if S[x1][y1] == '.' {
                            dd[x1][y1] = Some(dd[x][y].unwrap()+1);
                            todo.push_back((x1, y1));
                        }
                    }
                } else {
                    break;
                }
            }
            ans.chmax(dd[c][d].unwrap());
        }
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

